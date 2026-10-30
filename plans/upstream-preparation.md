# Clean contribution preparation

The clean candidates have now been [submitted upstream](upstream-submission.md)
with Steven’s authorization. Existing working topics and PRs
retain their review history. Separate clean branches contain the contribution
without fork plans or campaign history. The baseline is
81825c4812a6b9df45880da3a97fc7f63ad4e51b.

## Prepared candidates

Read-only upstream check confirmed main still at the baseline above. Every row
has an identical complete tree outside plans/ to its source topic. No code,
tests, notices, lockfiles or fixtures were removed. Clean branches carry focused
contribution commits and necessary follow-ups;
#2 includes the clean #1 dependency, while #3 and #13 include their final narrow
corrections.
Shared history was not rewritten. The working topics and fork PRs are preserved.
All branches below are pushed only to the fork.

| Fork PR | Clean branch | Exact candidate head |
| --- | --- | --- |
| #1 | [prepare/mcp-save-result](https://github.com/EffortlessSteven/zigzag/tree/prepare/mcp-save-result) | `d0654f6c400cf66a18ecd41a98dd28a2cf4407fe` |
| #2 | [prepare/atomic-file-replacement](https://github.com/EffortlessSteven/zigzag/tree/prepare/atomic-file-replacement) | `54e15e49a2ca16df66e88c8c9ac15d638329df7a` |
| #3 | [prepare/current-document-batches](https://github.com/EffortlessSteven/zigzag/tree/prepare/current-document-batches) | `f758277802df8e57b24f756c0d3031565c9dd5ff` |
| #4 | [prepare/bridge-json-rpc](https://github.com/EffortlessSteven/zigzag/tree/prepare/bridge-json-rpc) | `c9f67b581855b0a222a53085c7451a768c557fa9` |
| #5 | [prepare/mcp-schema-validation](https://github.com/EffortlessSteven/zigzag/tree/prepare/mcp-schema-validation) | `4d3b01f4bdd43c8212b1f41cfbc653f854733775` |
| #6 | [prepare/mcp-expired-calls](https://github.com/EffortlessSteven/zigzag/tree/prepare/mcp-expired-calls) | `9ca68dacfe7729a931d09fb8e073f7ab177e6243` |
| #7 | [docs/skitch-qualification](https://github.com/EffortlessSteven/zigzag/tree/docs/skitch-qualification) | `430a2695fbb1b642728d829059645872f39aa3d2` |
| #8 | [test/vendored-geometry](https://github.com/EffortlessSteven/zigzag/tree/test/vendored-geometry) | `3ac4f3affb06323743a33cea8e70c551a6621dc1` |
| #12 | [prepare/ui-worker-test-completion](https://github.com/EffortlessSteven/zigzag/tree/prepare/ui-worker-test-completion) | `0ace88ce8178f6b4f269a730d790d823ba56040f` |
| #13 | [prepare/portable-agent-process](https://github.com/EffortlessSteven/zigzag/tree/prepare/portable-agent-process) | `558bf9e90eb211798cc31f99a20da4494a52a1c0` |
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

## Preferred schema topic stacked on batching

Review 5710300411 identified an integration-only regression omitted from the
initial clean combination. It is preserved in prepare/schema-after-batches (#16),
now 589cf50ccfdbdbfe7ef8efd1df86050eea18c03c over repaired clean #3 f758277.
Both composition regressions and the final-plan reuse regression remain present.
All 29 MCP tool tests pass on Windows Rust 1.95. Exact-head proof is local
.git/qualification/final-schema-stack.json. Earlier stacked-schema receipts
retain the historical 49fa259/28-test packaging and mutation controls.

Clean #13 558bf9e includes the completion witness; positive agent tests pass and
both false-success controls fail. See [final corrections](final-corrections.md).
Product trees match their source topics outside plans/. No history was rewritten.
Use #16 for the schema contribution once; standalone #5 remains a fallback.
Submission URLs and exact heads are in [the submission record](upstream-submission.md).

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
