# MCP reliability campaign

Destination: `EffortlessSteven/zigzag` (fork). Do not submit upstream PRs.
The numbered inventory below is an investigation map, not eleven commitments.
Reproduce, narrow or reject findings; choose contribution boundaries from proof.
Steven explicitly decides any upstream submission after review. No release
publishing or hardware qualification is authorized by this plan. Keep internal
campaign context on the fork; extract clean upstream candidates only later.

| Slice | Seam | Dependencies | State |
| --- | --- | --- | --- |
| 1 | Propagate save failures through MCP | none | verified; fork PR pending |
| 2 | Atomic document and export replacement | 1 | pending |
| 3 | Current-document atomic batches and shared tool policy | none | pending |
| 4 | Design-only embedded agent capabilities | 3 | pending |
| 5 | JSON-RPC bridge error serialization | none | pending |
| 6 | Advertised input schema enforcement | none | pending; supplied detail ends mid-spec |
| 7 | Loopback transport hardening | 5 | pending; detailed criteria needed |
| 8 | Engine, studio, feature and vendored CI | none | pending; detailed criteria needed |
| 9 | Locked and recorded Mac release inputs | 8 desirable | pending; detailed criteria needed |
| 10 | Qualify unvalidated Skitch support claims | human qualification | pending |
| 11 | Modern and legacy MCP protocol support | 7 | pending; detailed criteria needed |

## Slice 1 verified plan

`Document::save` already returns its filesystem error and commits path/name only
after the write. `Studio::save` discards the result; `run_tool("save")` infers it
from historical modified state. Preserve the result at the Studio boundary,
retain UI status wording, and explicitly discard results at UI-only callers.

Files: `src/studio/{app,commands,tools,session}.rs`.
Acceptance: successful modified save writes expected SVG and clears modified;
failed saves return MCP errors for both modified and unmodified documents;
failure preserves identity, stable document and flags; native failure reports
`Couldn't save:`. Test failures use a regular file as destination parent.

Proof: `cargo test --locked --features gui --bin zigzag-studio save_`, then the
studio tool tests. Inspect diff and whitespace. Repository-wide rustfmt currently
reports extensive existing differences; do not reformat unrelated source.

Non-goals: atomic writes, export changes, protocol changes, and serialization.
Rollback: revert slice 1. Remaining slices stay separate and preserve their
dependency order. PR receipts should report actual commands and limitations.

## Slice 1 validation

Base: `81825c4`. Windows, rustc 1.98.0. Locked dependencies.

- PASS: focused save regression (1 test).
- PASS: `cargo test --locked --features gui --bin zigzag-studio tools::tests:: -- --skip view_tools:: --skip path_tools::` (12 tests).
- FAIL: broader `tools::tests` substring selection (29 pass, 1 fail):
  `view_tools::tests::decorative_fills_and_satin_styles_from_the_panel`, line 882.
  Same failure reproduced using unchanged base source, then restored this patch.
- PASS: `git diff --check`.
- FAIL: `cargo fmt --all -- --check`, extensive pre-existing formatting drift.
- NOT RUN: full studio suite, Mac GUI, Rust 1.95, hardware qualification.

Proof covers save result propagation and preserved save state. It does not prove
atomic replacement or power-loss durability; slice 2 owns that contract.

Red/green witness added after initial verification: test-only insertion of
`unmodified_failed_save_as_is_an_mcp_error` into the original `81825c4` source
(tools, app, commands and session) FAILS with `filesystem failure was reported
as MCP success`. Patched save tests PASS (2). This directly discriminates the
false-success defect. Independent review remains pending; do not treat an open
fork PR as reviewed or ready for upstream submission.

## Slice 2 verified plan

Use `tempfile::NamedTempFile::persist` for same-directory, unique, atomic
replacement of existing files on Windows and Unix. Flush and sync the file
before commit; sync the parent on Unix where supported. This dependency is
already in the lockfile transitively. Keep one library implementation, accessible
to the separate studio binary through a hidden public module.

Route Document saves, studio exports, and the existing small recovery wrapper
through it. Private injectable replacement closures let document/export tests
fail after partial temporary writes without process-global failure switches.
Test successful overwrite, partial-write cleanup, replacement failure, identity
preservation, and PES/HTML export preservation. Do not change format writers.
Rollback is this slice alone, leaving slice 1's result contract intact.

Slice 2 proof (Windows, rustc 1.98.0, locked): PASS document failure/identity
test (1), shared replacement test (1), PES/HTML export test (1), recovery/session
tests (9). `tempfile` 3.27.0 declares Rust 1.63 and MIT/Apache-2.0 licensing;
promoting the locked transitive dependency adds no package versions. Added its MIT notice to
`THIRD-PARTY-NOTICES.md`. Mac/Unix execution and power-loss tests
are NOT RUN. Parent-directory sync errors after rename report a durability
failure; they cannot roll back an already committed filesystem replacement.

Review correction: a new regression reproduced in-memory normalization on a
failed save in the first candidate. Save now serializes a cloned document and
commits normalization/identity only after persistence succeeds. Windows focused
document test and shared replacement tests PASS. An injected post-commit sync
failure confirms new bytes remain visible and the error explicitly reports that
commit. Basic permissions are copied; existing symlinks resolve to their target,
and dangling links fail before writing. Unix mode/symlink test is NOT RUN here.
ACLs, ownership, extended metadata, hard-link identity and unusual/network
filesystem behavior remain qualification gaps; this is not universal durability.

Notice qualification corrected: regenerated the complete configured Mac
dependency notices (218 libraries) with isolated cargo-about 0.9.3 and the repo
renderer, replacing the initial hand-added notice. Lock fingerprint matches.
The installed 0.6.4 tool cannot accept `--offline` or the existing Ubuntu SPDX
identifier; it was left untouched. No Mac application build is claimed.

Additional metadata review preserves standard new-file Unix permissions
(`0666 & umask`) instead of silently giving new documents tempfile's private
mode. Existing destinations retain their basic permissions. A Unix comparison
test against ordinary file creation was added but is NOT RUN on this Windows
host. Windows replacement tests remain PASS (3).

## Revised campaign contract

The numbered rows are an investigation inventory, not eleven commitments.
Every finding may be fixed, narrowed or ruled out. All work stays on this fork;
upstream submission requires Steven's explicit word after review. Independent
review is pending. Upstream packages should omit fork-only campaign machinery.
Preserve useful authorized tracing/export workflows when designing embedded
authority; nested dispatch must inherit grants. Client flags alone are not a
server authorization boundary. Authentication and document targeting need
explicit workflow decisions. Queued timeout/cancellation is a separate candidate.

## Persistence review follow-up (fork only)

Repair two distinct outcomes in the shared replacement seam. First reproduce a
read-only destination replacement as an unprivileged Linux user; require the
same effective write-access decision as an ordinary non-truncating write open.
Do not infer access from mode bits (privileged users and ACLs differ).

Represent a directory-sync failure after replacement as a typed committed
outcome. Document normalization and Save As identity must then reflect the
visible replacement, while Studio reports uncertainty, returns an error to MCP,
and retains recovery until a successful retry. Pre-replacement errors preserve
both disk bytes and live document state. Tests inject sync failure without
claiming power-loss durability. No changes to serialization or format writers.

Proof: native Linux replacement tests; Windows document, save, export and session
regressions. macOS, unusual filesystems and physical crash testing remain outside
this candidate's qualification. The write-access probe is not an authorization
lease across concurrent filesystem changes.

Persistence review results: the unprivileged Ubuntu/WSL regression failed before
the write-access probe (ordinary write denied, replacement succeeded). With the
probe, native Linux library tests PASS: 142 passed, 1 ignored, Rust 1.98.1.
This executes Unix mode and symlink tests on native temporary files, not merely
cross-compilation. Windows library tests PASS: 142 passed, 1 ignored, Rust 1.98.0.
The committed-identity test also fails under a deliberate wrong implementation
that discards prepared state on every error (`committed Save As identity lost`).
The actual implementation restores the typed committed outcome; the Studio
warning/recovery/retry test passes. No macOS or power-loss qualification is claimed.

Windows focused Studio save tests PASS (3); PES/HTML export regression PASS (1).
Session suite FAIL: 8 passed, 1 failed (`the_design_on_the_machine_comes_back`,
restore_sent assertion). Exact isolated retry PASS. This repeats the unresolved
session/simulator intermittency tracked in fork issue #9; do not classify the
suite as green or claim a proven root cause. Raw current-run logs are retained
under .git/qualification/persistence-*.log (local evidence, not upstream files).
