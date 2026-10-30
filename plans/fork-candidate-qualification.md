# Fork candidate qualification

Scope: EffortlessSteven/zigzag only. Baseline `81825c4812a6b9df45880da3a97fc7f63ad4e51b`.
Tested combined code/fixture head: `74058d8edf50d3fd8f60663f6a006fa3b320bfb7`. Later changes are documentation
only; `src`, `tests`, Cargo.toml and Cargo.lock are unchanged from that head.
Upstream submission is recorded [separately](upstream-submission.md). No
fork-main merge, release or hardware qualification is claimed. These are open candidates, not a required count of contributions.

| Fork PR | Branch | Head |
| --- | --- | --- |
| #1 | `fix/mcp-save-result` | `4dd38e2c79cbc40ce699db34265b3617449e1e14` |
| #2 | `fix/atomic-file-replacement` | `74b345aeaa6c5dbe86032dd9aa875d6376418d5f` |
| #3 | `fix/current-document-batches` | `f67a48e0413d1df63776f75b518065e5e16d716f` |
| #4 | `fix/bridge-json-rpc` | `d9db78c1d28ddcc38144a72de70e0674879322f6` |
| #5 | `fix/mcp-schema-validation` | `30f6fd28498d1e691669554d6e5f921eed5688fa` |
| #6 | `fix/mcp-expired-calls` | `920951104473f9987c4911730dca86621bb124ce` |
| #7 | `docs/skitch-qualification` | `430a2695fbb1b642728d829059645872f39aa3d2` |
| #8 | `test/vendored-geometry` | `3ac4f3affb06323743a33cea8e70c551a6621dc1` |
| #12 | `fix/ui-worker-test-completion` | `eb5af9b17b55722751fe828dcc66d6c3d080e30c` |
| #13 | `test/portable-agent-process` | `6d4286546cc326110fe19f0d89ac66f1b9945a1d` |
| #14 | `test/manual-simulator-clock` | `3b2aba6f390b1c0a0c6877d7e412a798420dbecf` |
| #15 | `test/session-writer-completion` | `b187d9a8e42772bda2b5ede000d5512ed514685c` |

## Executed qualification

Windows compiler: rustc 1.95.0 (59807616e 2026-04-14).
Native Ubuntu/WSL compiler: rustc 1.98.1 (48a229cea 2026-09-01).
Separate target directories avoid sharing Windows/Linux/compiler artifacts.

| Selection | Windows Rust 1.95 | Native Linux Rust 1.98.1 |
| --- | --- | --- |
| Library, GUI feature enabled | PASS: 144, 1 ignored | PASS: 145, 1 ignored |
| `zigzag-mcp` | PASS: 3 | PASS: 3 |
| `zigzag-studio` | PASS: 188, 3 ignored | PASS: 188, 3 ignored |
| `zigzag` CLI | 0 selected, no added coverage | 0 selected, no added coverage |

Commands:

- Windows: `cargo +1.95.0 test --locked --features gui --lib --bins --target-dir target/msrv-1.95 -j 4`.
- Linux: `cargo test --locked --features gui --lib --bins --target-dir /mnt/h/Code/Rust/zigzag/target/wsl-qualification -j 4`, executed natively in Ubuntu with the repository manifest selected explicitly.
- Windows core without GUI: `cargo +1.95.0 test --locked --lib --bins --target-dir target/msrv-1.95 -j 4`: PASS, 144 library tests (one benchmark ignored), three bridge tests; CLI selects zero. Run at 1bbdb3c; the later changes are confined to GUI-only Studio code/tests, so this selection is unchanged.
- Windows GUI/Bluetooth compilation: `cargo +1.95.0 check --locked --features gui,ble --bins --target-dir target/msrv-1.95 -j 4`: PASS.
- Apple source compilation: the same check with `--target x86_64-apple-darwin`: PASS. This is not linking or execution on a Mac.
- `git diff 81825c4812a6b9df45880da3a97fc7f63ad4e51b --check`: PASS. Whole-repository formatting drift was previously identified; no broad formatting rewrite is included.

The ignored tests are existing benchmarks, not skipped regressions. No existing
assertion was weakened to achieve these passes. The real-send fixture compiles a
std-only Rust helper during execution and checks real executable identity, flags,
stdin and streamed results. It does not establish installed-agent interoperability.

Local raw logs and exact command/head receipts are
`.git/qualification/corrections-windows.log`, `corrections-windows.json`,
`corrections-linux.log`, `corrections-linux.json`, `fifo-core.log`,
`fifo-core.json`, `corrections-gui-ble.*` and `corrections-apple-check.*`. These are local
working artifacts, not upstream-accessible evidence; the material results are above.

Earlier unchanged vendored evidence remains applicable: geometry 72 passed;
trace 222 passed and 2 ignored, selected through their own manifests. Those
standalone suites were not rerun for changes outside those crates. Root Cargo
metadata does not select them as workspace packages.

## Native macOS persistence qualification

[Run 41045633785](https://github.com/EffortlessSteven/zigzag/actions/runs/41045633785)
PASS on macOS 15.8.2 ARM64 with Rust 1.95.0: 143 library tests (one existing
benchmark ignored), plus five GUI-enabled recovery/save tests. This is the clean
#2 source, not the full integrated fork. Workflow head 95fdf36 differs from clean
#2 only by its fork qualification workflow. Persistence, session and tab source
blobs match the clean topic exactly; the root Cargo.lock was used unchanged.
The tests exercise real temporary files, permissions, symlinks, replacement,
Document saves, injected post-commit recovery warnings, metadata publication,
restart and retry. The FIFO fixture remains Linux-only. This is not a full Mac
Studio suite, interactive GL session, BLE or hardware qualification.

Commands: `cargo +1.95.0 test --locked --lib -- --nocapture` and
`cargo +1.95.0 test --locked --features gui --bin zigzag-studio recovery_ -- --nocapture`.
Downloaded environment/results are under `.git/qualification/native-macos-state/`.
The earlier [run 41039769416](https://github.com/EffortlessSteven/zigzag/actions/runs/41039769416)
also passed the same library selection before the GUI caller repair.
The workflow is isolated on qualification/macos-persistence and excluded from
all clean submission branches. Pinned actions, read-only contents permission,
no persisted checkout token, one bounded job; actionlint passed.

## What the failures established

- Read-only replacement was reproduced as an unprivileged Linux user before the
  non-truncating OS write-access check. A committed Save As identity test rejects
  discarding prepared state after replacement. Both are repaired in #2.
- Batch tests exposed grouping Undo, history cloning, stale deletion names and
  static-key preflight gaps. Combining #3 and #5 also exposed premature numeric
  reference validation. The repair now lives in both topics (69570f6 and 30f6fd2), not only
  integration commit afaa940. Their direct merge 6b35cab passes all 27 MCP tool
  tests. Restoring value validation in raw preflight fails the numeric-reference
  test with an unresolved string; the restored test checks the applied angle.
  The initially broader selection also hit the known UI timing assertion in
  decorative fills; that selection did not include #12. The final integrated
  full runs above include #12 and pass without weakening that assertion.
- Loaded UI failures read the old stitch plan while the worker was pending.
  Rendering generations also included transformed previews. #12 waits for actual
  completion and checks both request generation and accepted results. A
  controlled two-request/one-accepted-result case demonstrates the distinction.
  Scheduling work during either point or object drag fails the new admission
  assertion; removing drag commit also fails its oracle.
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

The fresh #2 special-file review reproduced replacement of a FIFO by a regular
file. The original helper fails the bounded Linux regression with `FIFO accepted`.
The repair rejects non-regular metadata before the write-access open, including
resolved symlink targets. The test checks unchanged FIFO/link kind, no writer
callback and no leaked temporary. Linux keeps the pipe open read/write to prevent
the old probe from blocking; that fixture is intentionally Linux-only. A common
directory test checks InvalidInput and preservation on Windows too. Standalone
#2 library suites pass 143 on Windows and 144 on Linux (one benchmark ignored
each). The integrated counts above also include the separate simulator repair.
No new dependency, streaming export or concurrent-path lease is introduced.

A later independent review reproduced a different post-commit outcome bug in
session recovery. #2 now publishes metadata after active or inactive recovery
replacement commits, keeps uncertain writes eligible for retry, and retains early
return on pre-commit failure. The deterministic regression failed first with
`committed recovery was not advertised`; it now checks real restart and retry,
including metadata warnings and equal metadata payloads. Negative controls
caching uncertainty or returning early for inactive warnings both fail. The
broader Windows/Linux selections and native Mac checks above include the repair.

## Focused proof for the changed behavior

Qualification follows the failure model at the changed boundary. Unavailable
end-to-end environments are not blanket prerequisites for these contributions.
Use direct observation where available and controlled faults/events where they
make the relevant outcome deterministic. Existing raw runs above already include
these tests; this assessment checked 16 source/receipt mappings and adds no
product code or duplicate suite.

| Changed seam | Relevant focused proof | What it establishes |
| --- | --- | --- |
| Whole-file persistence | Partial-write failure, Windows replacement failure, directory-sync error after commit, permissions/symlinks on real OS filesystems, Linux FIFO rejection | Old bytes survive preparation failure; commit outcomes and cleanup are truthful. Controller/cache behavior during physical power loss is a different claim. |
| Document and recovery state | Staged identity/normalization; real temporary-file save/restart; active/inactive warnings, pre-commit failure, equal-metadata retry, Save cleanup and unknown-orphan preservation | Application state follows actual commit and recovery state. This does not depend on a Skitch or GPU. |
| Atomic edits and UI planning | Distinct-tab rollback and one-step Undo; real headless frames; request generations versus accepted results; controlled worker completion | Correct edit/history/selection behavior and planning admission. Native graphics-driver behavior is unchanged. |
| Preview arguments | Invalid-type rejection and actual PNG width after decimal-integer input; create/edit/preview/check workflow | The supplied value reaches the output rather than silently defaulting. No native GL context is needed for that contract. |
| Simulator/test lifecycle | Real Session through Sim with controlled time; short/late replies and permanently short responses; thread-break/unpaired cases; headless whole-job sewing window | Timing, state progression and application workflow without physical machinery. The manual-clock change is covered by the integrated Windows/Linux run; these are not radio/firmware compatibility claims. |
| Queued calls and agent output | Real HTTP admission/commit gates; explicit Exit/EOF ordering; portable executable fixture through the real send path | Expired queued work does not start, started timeouts remain uncertain, and late output survives. This is separate from full installed-client interoperability. |

A read-only follow-up assessment found no additional cheap persistence test
necessary for the stated contract. The focused fault/state tests already cover
its pre/post-commit boundaries on the relevant platforms. No new physical device,
BLE transport, GPU renderer or on-disk session schema is introduced by these fixes.
Broader hardware/driver/release qualification becomes necessary when changing or
claiming those behaviors; it is not outstanding acceptance work for unrelated
save, parser, transaction or test-synchronization corrections.

## Review and remaining boundaries

Owner review checked completion ordering, real send wiring, clock consistency,
write-state outcomes and the preserved assertions, with discriminating controls.
The fresh composition and hidden-request findings are fixed on their owning
topics and their threads have evidence-backed replies. CodeRabbit reviewed #13
and its wiring feedback was repaired; no fresh independent approval of every
latest topic is claimed. Keep candidates open for substantive review and Steven's
selection. Green tests do not authorize upstream publication or release.

The subsequent owner pass rechecked transaction capture/commit/restore, reference
validation order, deadline admission, agent stdout/exit ordering, and simulator/
writer completion. It found no additional blocker in those changed paths. Live
review threads were all resolved at inspection. Issue #9 now names the current
qualified source; #11 received newline-only cleanup. This is bounded owner review,
not independent approval or qualification of the deferred protocol/authority work.

Broader, unclaimed qualification: full native Mac Studio/GL interaction,
physical Skitch upload/stitch-out, BLE radio/firmware behavior, real power-cut
survival and unusual/network filesystems. These are not prerequisites for the
bounded claims above. Native Mac execution establishes the selected real
filesystem/library/recovery paths; fault injection establishes application
handling of controlled failures.
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

Two independent read-only agents subsequently reviewed exact candidates. Schema
stack 49fa259 had no actionable finding; its source/claims, raw 28-test result and
negative-control proof were inspected without rebuilding. Persistence e5b5d65
had one source-derived P3: an uncertain committed sidecar was forgotten before
normal Save cleanup. Owner reproduction failed with `successful Save retained
recovery`. At 74b345a, an explicit Current/Retry state retains known-file presence
across failed retries and tab swaps while preserving unknown orphan bytes.
Metadata-only warnings retain Current; successful retries refresh it. The same
reviewer examined the repair and found no new actionable issue. Controls for
indiscriminate deletion and skipped recovery writes both fail; restored tests
pass. The current local/native qualification above includes this repair.

CodeRabbit reviewed the earlier 77d49f8 without actionable comments. Its later
requests were rate-limited; no newer CodeRabbit result is claimed. The separate
agent source/evidence reviews are complete, with scope recorded in
[final checks](final-checks.md). They do not authorize upstream publication or
replace native interactive/hardware qualification.

## Final scoped corrections

[Final corrections](final-corrections.md) records #3 final-plan reuse and #13
fixture completion proof, their independent reviews and discriminating controls.
The fresh full runs above cover both. Preferred schema stack 589cf50 passes all
29 tool tests and retains both composition regressions. Earlier direct-merge
results earlier in this record describe their historical candidates, not the current stack.
