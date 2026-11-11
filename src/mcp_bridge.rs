//! zigzag-mcp: stdio ↔ HTTP bridge for MCP clients that launch servers as
//! a command (e.g. Claude Desktop). Forwards each JSON-RPC line from stdin to
//! the running studio's MCP endpoint and prints the reply.
//!
//!   zigzag-mcp [--port 7878]
//!   zigzag-mcp --headless
//!
//! The studio must be open; requests sent while it isn't get a JSON-RPC error
//! explaining that, and the bridge keeps running so it works once it's opened.
//! With `--headless` it runs the studio's tools without a window instead
//! (`zigzag-studio --mcp-stdio`, found next to this program or on PATH),
//! so an agent can work while the app is closed.

use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

enum PostError {
    /// Nothing is listening: the studio isn't open.
    NotRunning,
    /// It was reached, but the exchange failed (or timed out).
    Failed(std::io::Error),
}

fn post(port: u16, body: &str) -> Result<(u16, String), PostError> {
    let mut s = TcpStream::connect(("127.0.0.1", port)).map_err(|_| PostError::NotRunning)?;
    let mut exchange = || -> std::io::Result<(u16, String)> {
        s.set_read_timeout(Some(Duration::from_secs(180)))?;
        write!(
            s,
            "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )?;
        let mut raw = Vec::new();
        s.read_to_end(&mut raw)?;
        Ok(parse_response(&raw))
    };
    exchange().map_err(PostError::Failed)
}

/// Status code and body of a raw HTTP response. Split on bytes, decoded
/// afterwards: a chunk boundary can fall inside a multi-byte character.
fn parse_response(raw: &[u8]) -> (u16, String) {
    let split = raw.windows(4).position(|w| w == b"\r\n\r\n");
    let (head, rest) = match split {
        Some(i) => (&raw[..i], &raw[i + 4..]),
        None => (raw, &raw[raw.len()..]),
    };
    let head = String::from_utf8_lossy(head);
    let status = head
        .split_whitespace()
        .nth(1)
        .and_then(|c| c.parse().ok())
        .unwrap_or(0);
    let chunked = head.lines().any(|l| {
        l.to_ascii_lowercase().starts_with("transfer-encoding:")
            && l.to_ascii_lowercase().contains("chunked")
    });
    let body = if chunked {
        dechunk(rest)
    } else {
        rest.to_vec()
    };
    (status, String::from_utf8_lossy(&body).into_owned())
}

fn dechunk(mut s: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    while let Some(eol) = s.windows(2).position(|w| w == b"\r\n") {
        // A chunk size line: hex digits, then optional `;extensions`.
        let line = String::from_utf8_lossy(&s[..eol]);
        let len = line.split(';').next().unwrap_or("").trim();
        let n = usize::from_str_radix(len, 16).unwrap_or(0);
        let rest = &s[eol + 2..];
        if n == 0 || rest.len() < n {
            break;
        }
        out.extend_from_slice(&rest[..n]);
        s = &rest[n..];
        s = s.strip_prefix(b"\r\n").unwrap_or(s);
    }
    out
}

/// A JSON-RPC error answering `line`; none for a notification (no id) or a
/// line that isn't JSON.
fn error_reply(line: &str, message: &str) -> Option<String> {
    let req: serde_json::Value = serde_json::from_str(line).ok()?;
    let id = req.get("id")?;
    Some(
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {"code": -32000, "message": message},
        })
        .to_string(),
    )
}

fn main() {
    let mut port: u16 = std::env::var("ZIGZAG_MCP_PORT")
        .or_else(|_| std::env::var("THREADLINE_MCP_PORT"))
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(7878);
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--port" {
            port = args.next().and_then(|v| v.parse().ok()).unwrap_or(port);
        } else if a == "--headless" {
            let name = format!("zigzag-studio{}", std::env::consts::EXE_SUFFIX);
            let beside = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join(&name)))
                .filter(|p| p.exists());
            let program = beside.unwrap_or_else(|| name.clone().into());
            let status = std::process::Command::new(&program)
                .arg("--mcp-stdio")
                .status();
            match status {
                Ok(s) => std::process::exit(s.code().unwrap_or(0)),
                Err(e) => {
                    eprintln!("zigzag-mcp: couldn't run {}: {e}", program.display());
                    std::process::exit(1);
                }
            }
        }
    }
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match post(port, &line) {
            Ok((200, body)) if !body.trim().is_empty() => Some(body.trim().to_string()),
            Ok((202, _)) | Ok((200, _)) => None,
            Ok((code, body)) => error_reply(
                &line,
                &format!("ZigZag returned HTTP {code}: {}", body.trim()),
            ),
            Err(PostError::Failed(e)) => error_reply(&line, &format!("ZigZag didn't answer: {e}")),
            Err(PostError::NotRunning) => error_reply(
                &line,
                &format!(
                    "ZigZag isn't running (nothing on 127.0.0.1:{port}). Open ZigZag, then retry, or configure zigzag-mcp --headless to work without the app."
                ),
            ),
        };
        if let Some(r) = reply {
            let _ = writeln!(stdout, "{r}");
            let _ = stdout.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_chunk_boundary_inside_a_character_is_put_back_together() {
        let body = "{\"text\":\"100 × 100 mm → ok\"}";
        let bytes = body.as_bytes();
        // Cut in the middle of the two-byte `×`.
        let cut = body.find('×').unwrap() + 1;
        let mut raw = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n".to_vec();
        for part in [&bytes[..cut], &bytes[cut..]] {
            raw.extend_from_slice(format!("{:x}\r\n", part.len()).as_bytes());
            raw.extend_from_slice(part);
            raw.extend_from_slice(b"\r\n");
        }
        raw.extend_from_slice(b"0\r\n\r\n");
        assert_eq!(parse_response(&raw), (200, body.to_string()));
    }

    #[test]
    fn an_error_reply_is_valid_json_for_the_requests_own_id() {
        let line = r#"{"jsonrpc":"2.0","method":"tools/call","params":{"id":"inner"},"id":7}"#;
        let r = error_reply(line, "HTTP 500: a \"quoted\"\nback\\slash").unwrap();
        let v: serde_json::Value = serde_json::from_str(&r).unwrap();
        assert_eq!(v["id"], 7);
        assert_eq!(v["error"]["message"], "HTTP 500: a \"quoted\"\nback\\slash");
        assert!(!r.contains('\n'), "one line");
        // Notifications get no reply.
        assert!(error_reply(r#"{"jsonrpc":"2.0","method":"x"}"#, "m").is_none());
    }

    #[test]
    fn a_plain_body_and_its_status_are_read() {
        let raw = b"HTTP/1.1 404 Not Found\r\nContent-Length: 4\r\n\r\nnope";
        assert_eq!(parse_response(raw), (404, "nope".to_string()));
        assert_eq!(parse_response(b"garbage"), (0, String::new()));
    }
}
