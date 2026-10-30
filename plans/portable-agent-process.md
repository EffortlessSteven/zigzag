# Preserve agent output through process completion

Fork PR #13, issue #9; baseline 81825c4. Process exit is reported by the stderr/
wait thread independently of stdout. Dropping Run on Exit can lose unread JSON,
including the session and final message. The integrated Rust 1.95 run reproduced
only generic Done with no session; a deterministic exit-first test also failed.

## Current implementation and boundary

Retain observed exit status until stdout EOF, emitted after the stdout reader's
last line on the same sender. Poll consumes those lines before completion. The
regression covers Exit-before-lines and EOF-before-Exit, including session,
assistant text and the real streamed completion. Stop and error mapping retain
their behavior. This does not change client permissions or CLI compatibility.

Production send uses its original inline launch flow. Its portable integration
test compiles a std-only Rust process fixture, then calls real AgentState::send
in a child test process with child-scoped executable/PATH settings. The fixture
checks actual strict-MCP/disabled-built-in flags and records executable identity
and stdin bytes. A safe PATH decoy prevents fallback to an installed agent. There
is no global environment mutation, shell script, new dependency or launcher hook.
The test needs rustc, as the Cargo build does. Temporary outputs are removed.

## Evidence

All 15 standalone Windows agent tests pass. Negative controls fail when:
- process exit drops unread stdout;
- lookup ignores the configured executable (the safe decoy is detected);
- send changes the supplied stdin (the recorded bytes differ).

The final combined code/test head is 35ad1be4201982d02c7fda077c3519676dd21c5a.
On Windows Rust 1.95.0 and native Ubuntu/WSL Rust 1.98.1, full library/binary
selection passes: 143 library tests (1 ignored), 3 bridge tests, and 183 Studio
tests (3 ignored). The CLI selects zero tests, not extra coverage. GUI/Bluetooth
binary checks pass on Windows and for the x86_64-apple-darwin target on Rust 1.95.
The Apple result is cross-compilation only, not native Mac or hardware execution.

Commands and source identity are recorded in the integrated branch's
plans/fork-candidate-qualification.md and local qualified-msrv/qualified-linux
receipts. Documentation-only updates do not invalidate unchanged source proof.
The review's send-wiring gap is addressed by the real-send test and negative
controls; current qualification is not a claim of fresh independent approval.

Fork only. This plan is excluded from any eventual upstream package. Real
installed-agent interoperability and authority policy remain separate work.
