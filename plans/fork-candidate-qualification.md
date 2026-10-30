# Fork candidate qualification

Scope: EffortlessSteven/zigzag only. Baseline `81825c4812a6b9df45880da3a97fc7f63ad4e51b`.
Tested combined code/fixture head: `35ad1be4201982d02c7fda077c3519676dd21c5a`. Later changes are documentation
only; `src`, `tests`, Cargo.toml and Cargo.lock are unchanged from that head.
No upstream submission, fork-main merge, release or hardware qualification is
claimed. These are open candidates, not a required count of contributions.

| Fork PR | Branch | Head |
| --- | --- | --- |
| #1 | `fix/mcp-save-result` | `4dd38e2c79cbc40ce699db34265b3617449e1e14` |
| #2 | `fix/atomic-file-replacement` | `bbaf8458408e215d72af5ddda40e51c2d175ad52` |
| #3 | `fix/current-document-batches` | `6530304ea2a76374e76ced0629dd07b0d45cb490` |
| #4 | `fix/bridge-json-rpc` | `d9db78c1d28ddcc38144a72de70e0674879322f6` |
| #5 | `fix/mcp-schema-validation` | `942b9ecf0f61112d38fb3cb7dee7d246efbd93bb` |
| #6 | `fix/mcp-expired-calls` | `920951104473f9987c4911730dca86621bb124ce` |
| #7 | `docs/skitch-qualification` | `430a2695fbb1b642728d829059645872f39aa3d2` |
| #8 | `test/vendored-geometry` | `3ac4f3affb06323743a33cea8e70c551a6621dc1` |
| #12 | `fix/ui-worker-test-completion` | `438481cb92d5a185b28cf2cc56b5d21cb27c8262` |
| #13 | `test/portable-agent-process` | `4e078fcbd840b00a4e05c11f3e0543b183558053` |
| #14 | `test/manual-simulator-clock` | `3b2aba6f390b1c0a0c6877d7e412a798420dbecf` |
| #15 | `test/session-writer-completion` | `b187d9a8e42772bda2b5ede000d5512ed514685c` |

## Executed qualification

Windows compiler: rustc 1.95.0 (59807616e 2026-04-14).
Native Ubuntu/WSL compiler: rustc 1.98.1 (48a229cea 2026-09-01).
Separate target directories avoid sharing Windows/Linux/compiler artifacts.

| Selection | Windows Rust 1.95 | Native Linux Rust 1.98.1 |
| --- | --- | --- |
| Library, GUI feature enabled | PASS: 143, 1 ignored | PASS: 143, 1 ignored |
| `zigzag-mcp` | PASS: 3 | PASS: 3 |
| `zigzag-studio` | PASS: 183, 3 ignored | PASS: 183, 3 ignored |
| `zigzag` CLI | 0 selected, no added coverage | 0 selected, no added coverage |

Commands:

- Windows: `cargo +1.95.0 test --locked --features gui --lib --bins --target-dir target/msrv-1.95 -j 4`.
- Linux: `cargo test --locked --features gui --lib --bins --target-dir /mnt/h/Code/Rust/zigzag/target/wsl-qualification -j 4`, executed natively in Ubuntu with the repository manifest selected explicitly.
- Windows GUI/Bluetooth compilation: `cargo +1.95.0 check --locked --features gui,ble --bins --target-dir target/msrv-1.95 -j 4`: PASS.
- Apple source compilation: the same check with `--target x86_64-apple-darwin`: PASS. This is not linking or execution on a Mac.
- `git diff 81825c4812a6b9df45880da3a97fc7f63ad4e51b --check`: PASS. Whole-repository formatting drift was previously identified; no broad formatting rewrite is included.

The ignored tests are existing benchmarks, not skipped regressions. No existing
assertion was weakened to achieve these passes. The real-send fixture compiles a
std-only Rust helper during execution and checks real executable identity, flags,
stdin and streamed results. It does not establish installed-agent interoperability.

Local raw logs and exact command/head receipts are
`.git/qualification/qualified-msrv-*.log`, `qualified-msrv.json`,
`qualified-linux-0.log`, and `qualified-linux.json`. They are local working
artifacts, not upstream-accessible evidence; the material results are above.

Earlier unchanged vendored evidence remains applicable: geometry 72 passed;
trace 222 passed and 2 ignored, selected through their own manifests. Those
standalone suites were not rerun for changes outside those crates. Root Cargo
metadata does not select them as workspace packages.

## What the failures established

- Read-only replacement was reproduced as an unprivileged Linux user before the
  non-truncating OS write-access check. A committed Save As identity test rejects
  discarding prepared state after replacement. Both are repaired in #2.
- Batch tests exposed grouping Undo, history cloning, stale deletion names and
  static-key preflight gaps. Combining #3 and #5 also exposed premature numeric
  reference validation. The composition repair in integration commit afaa940
  must accompany any future combination/extraction of those topics.
- Loaded UI failures read the old stitch plan while the worker was pending.
  Rendering generations also included transformed previews. #12 waits for actual
  completion and counts completed plans; removing drag commit fails its oracle.
- The portable process fixture exposed real output loss when Exit preceded
  stdout. #13 retains exit until EOF. Its event-order regression failed before
  that correction. Ignoring executable override or changing send's prompt also
  fails the restored real-send integration test using a safe decoy.
- Rust 1.95 caught a simulator test waiting for Initial after automatic homing
  had already reached Idle. #14 holds stages on an opt-in manual clock. Using
  wall time for a stage timestamp fails its Session lifecycle regression.
- The session test could proceed after a polling budget without writer
  completion. #15 joins the actual writer and preserves restoration assertions.
  That establishes its precondition, not a diagnosis of every older incident.

The older 176/4 and 182/1 Studio results remain in raw logs and issue #9; they
are not concealed or treated as current failures. The full selections above
exercise the combined repairs, including the originally failing cases.

## Review and remaining boundaries

Owner review checked completion ordering, real send wiring, clock consistency,
write-state outcomes and the preserved assertions, with discriminating controls.
Addressed review threads have evidence-backed replies. CodeRabbit reviewed #13
and its wiring feedback was repaired; no fresh independent approval of every
latest topic is claimed. Keep candidates open for substantive review and Steven's
selection. Green tests do not authorize upstream publication or release.

NOT RUN: native Mac application/filesystem execution, physical Skitch upload or
stitch-out, native OpenGL user sessions, power-loss tests, BLE transport runtime,
and unusual/network filesystems. Apple cross-checks do not close those gaps.
Basic file permissions and existing symlink behavior are covered; ownership,
ACLs, hard-link identity and extended metadata are outside replacement guarantees.

Running-Studio envelope conformance (#10), earlier client cancellation/deadlines
(#11), embedded-agent authority and user-scoped I/O remain separate work. Server
expiry is not client cancellation; client configuration is not server enforcement.
Fork-only plans must be omitted from any eventual upstream package. Reconcile
its exact extracted source and proof before submission.

Cleanup retains open topic branches and shared build caches; the temporary
baseline worktree and task-owned vendored locks were removed earlier. The scoped
license tool cache remains after automatic execution review rejected its removal.
No unrelated source, credentials or global toolchains were changed by cleanup.
