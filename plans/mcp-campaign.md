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

Save-result preservation review: the modified failure case now performs an
actual unsaved object edit rather than assigning the flag. It checks serialized
live artwork, Undo history and original disk bytes as well as stable identity
and error status. Both focused save tests PASS on Windows with locked inputs.
Normalization staging belongs to the separate persistence candidate; this test
compares represented design, not raw pre-save XML or revision allocation.
