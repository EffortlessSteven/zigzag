# Clean contribution preparation

Fork only. Nothing has been submitted upstream. Existing working topics and PRs
retain their review history. Separate clean branches contain the contribution
without fork plans or campaign history. The baseline is
81825c4812a6b9df45880da3a97fc7f63ad4e51b.

## Prepared candidates

Read-only upstream check confirmed main still at the baseline above. Every row
has an identical complete tree outside plans/ to its source topic. No code,
tests, notices, lockfiles or fixtures were removed. Each prepare/ branch has one
contribution commit except #2, which has the necessary clean #1 base and
follow-up commits repairing special-file rejection and recovery metadata outcomes.
Shared history was not rewritten. The working topics and fork PRs are preserved.
All branches below are pushed only to the fork.

| Fork PR | Clean branch | Exact candidate head |
| --- | --- | --- |
| #1 | [prepare/mcp-save-result](https://github.com/EffortlessSteven/zigzag/tree/prepare/mcp-save-result) | `d0654f6c400cf66a18ecd41a98dd28a2cf4407fe` |
| #2 | [prepare/atomic-file-replacement](https://github.com/EffortlessSteven/zigzag/tree/prepare/atomic-file-replacement) | `54e15e49a2ca16df66e88c8c9ac15d638329df7a` |
| #3 | [prepare/current-document-batches](https://github.com/EffortlessSteven/zigzag/tree/prepare/current-document-batches) | `a17d2e3851117b3e40dd72757faca4a04faba42f` |
| #4 | [prepare/bridge-json-rpc](https://github.com/EffortlessSteven/zigzag/tree/prepare/bridge-json-rpc) | `c9f67b581855b0a222a53085c7451a768c557fa9` |
| #5 | [prepare/mcp-schema-validation](https://github.com/EffortlessSteven/zigzag/tree/prepare/mcp-schema-validation) | `4d3b01f4bdd43c8212b1f41cfbc653f854733775` |
| #6 | [prepare/mcp-expired-calls](https://github.com/EffortlessSteven/zigzag/tree/prepare/mcp-expired-calls) | `9ca68dacfe7729a931d09fb8e073f7ab177e6243` |
| #7 | [docs/skitch-qualification](https://github.com/EffortlessSteven/zigzag/tree/docs/skitch-qualification) | `430a2695fbb1b642728d829059645872f39aa3d2` |
| #8 | [test/vendored-geometry](https://github.com/EffortlessSteven/zigzag/tree/test/vendored-geometry) | `3ac4f3affb06323743a33cea8e70c551a6621dc1` |
| #12 | [prepare/ui-worker-test-completion](https://github.com/EffortlessSteven/zigzag/tree/prepare/ui-worker-test-completion) | `0ace88ce8178f6b4f269a730d790d823ba56040f` |
| #13 | [prepare/portable-agent-process](https://github.com/EffortlessSteven/zigzag/tree/prepare/portable-agent-process) | `d565f078d0edfd83d8cddd3ae6ecbfc7dbf067f5` |
| #14 | [prepare/manual-simulator-clock](https://github.com/EffortlessSteven/zigzag/tree/prepare/manual-simulator-clock) | `a0c483861c9cea91a3fe72892d2064daa88da0e0` |
| #15 | [prepare/session-writer-completion](https://github.com/EffortlessSteven/zigzag/tree/prepare/session-writer-completion) | `4e1ef2a1235674be1dc02dfd8f7441923049ec0d` |

The source-topic heads and applicable broader qualification are recorded in
[fork qualification](fork-candidate-qualification.md). Extraction identity checks
are in local `.git/qualification/clean-topics.json`. The reused #7/#8 branches
already contain one focused commit and no internal plans.

## Proof and limits

- Clean #1: Rust 1.95 `cargo test --locked --features gui --bin zigzag-studio save --target-dir target/msrv-1.95 -j 4 -- --nocapture`: PASS, five tests (two MCP/save regressions plus three session/save checks).
- Clean #4: Rust 1.95 `cargo test --locked --bin zigzag-mcp --target-dir target/msrv-1.95 -j 4`: PASS, three tests.
- Every extracted diff passes `git diff --check` and contains no plans/ files. Production code/tests have no dependency on plans/. Complete product-tree equality was rechecked before pushing.
- Other isolated-topic results retain their existing scope because extraction changed only history and omitted internal plans. No new full standalone pass for every topic is claimed. In particular, combined-fork GUI results are not a claim that an isolated topic fixes all baseline UI failures.
- Clean #2 now includes the non-regular-target guard. Its product tree matches the revised working topic 74b345aeaa6c5dbe86032dd9aa875d6376418d5f. Standalone library proof and fresh integrated qualification are recorded in the linked qualification record.
- Earlier #3/#5 direct composition proof still applies to the identical product trees; their static names/value boundary was preserved in both clean topics.

The local exact-head execution records are `clean-topic-tests.json`,
`clean-save.log` and `clean-bridge.log` under `.git/qualification/`.
Native Mac library/recovery checks now pass for clean #2; installed-agent
interoperability, physical hardware and other qualification limits remain as
recorded in the fork qualification.

PR descriptions were also reconciled: internal tracking/development-note text
was removed from #6/#12–#15, stale integrated counts were corrected, and #8’s
verified result is stated directly. Effective explanations and material limits
were preserved. No working PR was retargeted or replaced.

## Before an actual submission

Steven selects the contribution and explicitly authorizes publication. Recheck
upstream base and fresh feedback then; do not send this inventory as a PR queue.
Use the clean branch and its necessary dependency, preserve the effective PR
explanation, and reconcile any claim affected by the selected candidate. Keep
this preparation record and all campaign plans out of the submitted diff.
Independent review remains independent work, not something supplied by extraction.

## Optional schema topic stacked on batching

Review 5710300411 identified an integration-only regression omitted from the
initial clean #3/#5 combination. It is now preserved in
[prepare/schema-after-batches](https://github.com/EffortlessSteven/zigzag/tree/prepare/schema-after-batches)
at `49fa25905d52e5f3b46aac3c2c0fa9a505710eb6`, based on clean #3. The standalone #3 and #5
branches remain available independently; this is an optional submission order,
not another contribution or a dependency forced onto standalone validation.

The complete product tree matches direct composition 6b35cab plus exactly the
existing 23-line integrated_schemas_reject_static_batch_input_before_any_edit test.
No production change or fork plan was added. Windows Rust 1.95 MCP tool selection
passes all 28 tests. Replacing full static preflight with name-only validation
fails `static type error admitted earlier edits`, so rollback cannot conceal
execution. Restoring the source passes all 28 again. The broader integrated
source already contains this test and is unchanged by this packaging work.

Local proof is in `.git/qualification/stacked-schema*.json` and logs. No fresh
Linux standalone run is claimed. Preserve this stacked test when selecting both
contributions; its preflight guarantee depends on their combined behavior.

The optional stack has a draft fork review view, [#16](https://github.com/EffortlessSteven/zigzag/pull/16).
It is an alternate view of #5, not a thirteenth contribution. Independent agent
review of 49fa259 found no actionable issue. The persistence reviewer found a
cleanup gap at e5b5d65; the owner reproduced it and the reviewer verified the
Current/Retry repair at 74b345a. Current qualification includes both that repair
and native Mac Save-after-warning cleanup. CodeRabbit-specific requests remain
rate-limited; that is separate from the completed independent agent reviews.
Steven accepted the jsonschema choice; no bespoke validator rewrite or unmeasured
dependency-cost claim is implied.

## Persistence cost and simplicity review

A focused independent static assessment of #2 at 74b345a found no concrete
avoidable cost or unnecessary abstraction. Save adds one staged Document clone
per attempt with a destination; this preserves live normalization/identity on
failure. The existing Studio stable copy remains. Save renders/syncs once;
exports already built complete bytes, and recovery adds no full-document copy.

Replacement intentionally pays for a unique sibling, file sync, replacement,
Unix parent sync and existing-target write/permission checks. Current recovery
snapshots skip SVG rewrites; metadata-only warnings keep that cache, and a
successful retry settles. The frame path adds no filesystem probe, document copy
or serialization. Tab switching moves Current/Retry state.

The extra save copy can raise peak memory for large designs. No wall-time,
allocation or large-document benchmark was performed, so no quantified speed
claim is made. The existing local/native execution and targeted state/failure
proof remain applicable; no runtime code or redundant suite changed in this pass.
