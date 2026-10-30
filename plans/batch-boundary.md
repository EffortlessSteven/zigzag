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

## Successful transaction and history ownership review

Review identified two additional obligations: grouping colors must mark an edit
and preserve the pre-batch order in Undo, including when it precedes another
edit; capture must not clone up to 200 historical documents per batch.

Reproduce grouping directly and in successful batches, then check Undo/Redo.
Use allocation identity to distinguish cloned versus moved history buffers.
Move original history into the transaction, give steps scratch history, discard
intermediate step snapshots after each success, and restore original history
on rollback. On commit, append the transaction's original live document as the
single Undo entry. Keep readonly batches' history unchanged. No caller-authority
or native editing-engine redesign is needed for this correction.

Review witnesses: grouping lost modified/Undo state, and history allocation
identity showed cloning, before the corrections. Both now pass. Successful
grouping, grouping followed by another edit, create/group, Undo/Redo, history
limit trimming, readonly history preservation, and rollback are covered.
Snapshotting moves the existing history buffers; it has no history-length
document-copy cost. Each completed step's temporary history is dropped.

Static argument errors are now preflighted before step zero (a regression
previously observed rollback planning, proving earlier work ran). Arguments
depending on earlier results remain validated after reference resolution at
dispatch; failures still roll back the document. This composes with the separate
schema candidate without coupling this PR to that dependency. MCP tools: 19
PASS on Windows, locked dependencies. Independent review remains pending.
