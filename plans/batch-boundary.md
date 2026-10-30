# Document-atomic batch candidate

Fork only. Base: `81825c4812a6b9df45880da3a97fc7f63ad4e51b`.
The numbered campaign is an investigation inventory, not a delivery quota.

The current batch rollback cannot restore tab transitions, filesystem writes,
or device actions. Preflight every atomic step before step zero; reject unknown
tools and effects outside the transaction. Trace may read an image and edit the
document: effects compose, and reversibility does not establish authorization.

Capture live and stable documents independently, full history, selection,
layout/isolation/preview state, path selection, view/fit and stitch player.
Rebuild derived caches on rollback. Preserve an explicit failure diagnostic.
Successful batches still collapse to one undo step; non-atomic failures keep
earlier work. Keep annotations conservative for the generic batch surface.

Proof: new preflight/two-tab and rollback tests first; then existing MCP tool
tests. Review changes against nesting and non-atomic compatibility. No full
Studio clone, caller authorization, authentication, document targeting changes,
protocol migration or upstream submission. Revert this candidate to roll back.

Agent authority requires its own workflow and transitive dispatch design. It
must preserve user-authorized image tracing and export. A catalog allowlist
alone, or atomic-batch restrictions, cannot enforce that boundary.

## Validation and review

Windows, rustc 1.98.0; dependencies locked. Baseline test-only insertion at
`81825c4`: `atomic_batch_cannot_replace_another_tabs_document` FAILS with
`failed batch corrupted tab state`. Same test passes in this candidate.
The preflight regression also fails on base (later disallowed tools were not
reported). Candidate MCP tools: 15 PASS, using
`cargo test --locked --features gui --bin zigzag-studio tools::tests:: -- --skip view_tools:: --skip path_tools::`.

Implementation review checked atomic/non-atomic dispatch, unknown tools,
nested batches, stable versus live snapshot, and cache revision reuse after
rollback. Preserved JSON error/result envelope for preflight failures. Independent
review remains pending. Full GUI, Mac and hardware tests are NOT RUN; the base
has a separately reproduced decorative-fill GUI failure. No upstream action.
