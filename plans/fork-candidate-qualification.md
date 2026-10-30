# Fork candidate qualification

Scope: EffortlessSteven/zigzag only. Upstream baseline:
`81825c4812a6b9df45880da3a97fc7f63ad4e51b`.
Integrated code/test candidate: `2fdfd17bab7488cb52c3337c504323177608e5c0`. Later documentation-only commits do
not alter the tested Rust source. No upstream submission or fork-main merge is
claimed. Findings below were addressed by the implementation owner; this is not
an independent approval of the latest heads.

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

## Current evidence

Windows uses Rust 1.98.0. Native Ubuntu/WSL uses Rust 1.98.1, uid 1000, with
separate build output and native temporary files for filesystem execution.

| Command / selection | Outcome | Applicability |
| --- | --- | --- |
| `cargo test --locked --features gui --bin zigzag-studio tools::tests:: -- --skip view_tools:: --skip path_tools::` | PASS, 32 tests | Integrated candidate above. Includes actual HTTP queue expiry and committed outcome, batch scope/rollback/Undo/reference checks, schemas, real unsaved-save preservation and design workflow. |
| Same binary, `committed_save_warning` | PASS, 1 test | Integrated candidate: visible Save As identity, stable state, recovery and successful retry. |
| `cargo test --locked --bin zigzag-mcp` | PASS, 3 tests | Bridge source unchanged since `d9db78c`, executed on integrated source. An earlier `--bin zigzag bridge_` selected zero tests and is not proof. |
| `cargo check --locked --features gui,ble --bins` | PASS | Integrated production source at `afaa940`; subsequent changes only add/strengthen tests. |
| `cargo test --locked --lib` | PASS, 142 tests, 1 ignored on each OS | Persistence source `0c0ee09`, unchanged in the integrated library. Native Unix mode/symlink and effective write-access behavior executed. |
| Same Studio binary, `save_` and `atomic_exports` | PASS, 3 and 1 tests | Persistence branch `0c0ee09`; later real-edit save test passes independently and in the combined suite. PES and HTML original-byte preservation covered. |
| Same Studio binary, `session::tests::` | FAIL, 8 pass / 1 fail | Persistence branch: machine-design restoration failed; exact isolated retry passed. Cause not established, issue #9. |
| `cargo test --locked --features gui --bin zigzag-studio` | FAIL, 176 pass / 4 fail / 3 ignored | Integrated source `afaa940`; two later test-only changes are separately exercised above. No full-suite green claim. |

The full-suite failures are agent fake-executable streaming and decorative-fill
rendering (both previously reproduced on the unchanged baseline), plus
`type_buttons_change_stitch_type_and_undo_restores` and
`dragging_plans_the_stitches_once_when_let_go` (both pass in exact isolated runs;
origin/cause unclassified). Issue #9 carries the current distinctions. No tests
were weakened or skipped to manufacture a green result.

Prior unchanged vendored evidence: geometry 72 tests pass; trace 222 tests pass,
2 ignored, selected by their own manifests. Root Cargo metadata does not select
them as workspace packages. This pass does not claim those suites were rerun
for documentation/test-only changes outside those crates.

## Discriminating witnesses

- Unprivileged native Rust read-only replacement failed before the OS write probe:
  ordinary write was denied but rename replaced the file. It passes afterward.
- The committed Save As test fails a deliberately wrong implementation that
  discards prepared identity on any error. Its failure is `committed Save As
  identity lost`; the actual implementation passes.
- Deletion/name tests reproduced wrong-survivor editing and an out-of-bounds
  panic before cheap name refresh. Static-key tests observed earlier execution
  and rollback planning before the preflight correction.
- Combining schema and key preflight rejected a valid numeric result reference
  as a string before resolution. The integrated numeric-reference test now
  passes alongside static invalid-input rejection.
- Earlier save false-success, bridge serialization, integer-default and queued
  execution witnesses remain in the topic tests/review history. Preservation
  checks are not all claimed as red/green regressions.

Raw local outputs are retained under `.git/qualification/` with descriptive
`persistence-*`, `integrated-review-*` and `integrated-final-*` names. They are
local working evidence, not accessible upstream artifacts; this record carries
the material results. Topic plans/review replies identify the individual repairs.

## Limits and remaining decisions

NOT RUN: macOS application/filesystem execution, Rust 1.95 GUI qualification,
real native-GUI sessions, physical Skitch upload/stitch-out, power-loss testing,
and unusual/network filesystems. File replacement copies basic permissions;
ACLs, ownership, hard-link identity and extended metadata are not guaranteed.
A write-access probe is not an authorization lease across filesystem races.

Server expiry is not earlier client cancellation or retry deduplication (#11).
Bridge fallback parsing is not running-Studio envelope conformance (#10).
Embedded-agent authority, authentication and useful user-scoped I/O need their
own settled design and actual negative integration evidence.

Local contribution skills were revised in place, not imported into ZigZag:
`pr-body`, `code-review`, `pull-request`. Their validators pass; twelve scenario
walkthroughs were performed by the owning session, not independent agent tests.
Backups and the walkthrough remain local under `.git/skill-backups/` and
`.git/qualification/skill-scenario-evaluation.md`.

Cleanup preserves all open topic branches and shared build caches. The temporary
baseline worktree and task-owned vendored lockfiles were removed after inspection.
An automatic approval review rejected recursive removal of the scoped license
tool cache as “blocked by policy”; `.git/license-tools` remains. No attempt was
made to bypass that rejection or delete unrelated work.
