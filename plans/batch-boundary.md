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

Further review reproduced deletion followed by name-based editing: the stale
name cache edited the wrong survivor, and delete/delete by name panicked past the
end of the shape list. The delete handler now refreshes names via the existing
`replan_soon` path (cheap/deferred during batching). Regressions cover correct
survivors, a later failure restoring the original design/history, and a usable
name-based edit plus Undo after restoration.

Further preflight review: references defer value checks, but object keys and the
arguments container shape remain statically known. Reproduce a misspelled key
alongside a nested result reference, and an array containing a result reference;
both must fail before step zero. Separate name/container checks from full value
validation so the schema candidate can still accept correctly resolved values.
Preserve whole-object result references and existing successful batch behavior.

Reference-bearing invalid arguments reproduced execution before rejection.
After separating static names/shape validation, all 21 standalone MCP tool
tests PASS (Windows, locked dependencies), including valid reference passing.

Composition review: schema checking must never migrate into check_arg_names,
which atomic preflight calls before resolving references. Share a cached catalog
accessor with the schema topic and keep the names/container checker identical
and value-independent in both. The schema topic carries the full validator only
in check_args, plus the valid numeric-reference regression. Prove the actual
normal merge of the two topic heads, not only the earlier integrated branch.

Final-read planning correction: commit now preserves the last step's planning
state rather than marking an already current plan dirty. The new regression
fails on 69570f6 with two plans instead of one; it also checks that a later edit
still triggers a second needed plan and that either batch has one complete Undo.
Windows Rust 1.95 standalone MCP tools: 22 pass. This is redundant-work repair,
not a measured wall-time performance claim or a new generation mechanism.
