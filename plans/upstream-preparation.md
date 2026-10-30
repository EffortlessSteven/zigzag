# Clean contribution preparation

Fork only. Nothing has been submitted upstream. Existing working topics and PRs
retain their review history. Separate clean branches contain the contribution
without fork plans or iterative history. The baseline is
81825c4812a6b9df45880da3a97fc7f63ad4e51b.

## Prepared candidates

Read-only upstream check confirmed main still at the baseline above. Every row
has an identical complete tree outside plans/ to its source topic. No code,
tests, notices, lockfiles or fixtures were removed. Each prepare/ branch has one
contribution commit; #2 has the necessary clean #1 commit beneath it. The working
topics and fork PRs are preserved. All branches below are pushed only to the fork.

| Fork PR | Clean branch | Exact candidate head |
| --- | --- | --- |
| #1 | [prepare/mcp-save-result](https://github.com/EffortlessSteven/zigzag/tree/prepare/mcp-save-result) | `d0654f6c400cf66a18ecd41a98dd28a2cf4407fe` |
| #2 | [prepare/atomic-file-replacement](https://github.com/EffortlessSteven/zigzag/tree/prepare/atomic-file-replacement) | `ae862a289d86366460e7c740daaf23d5b0a3106c` |
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
- Earlier #3/#5 direct composition proof still applies to the identical product trees; their static names/value boundary was preserved in both clean topics.

The local exact-head execution records are `clean-topic-tests.json`,
`clean-save.log` and `clean-bridge.log` under `.git/qualification/`.
Native Mac execution, installed-agent interoperability, physical hardware and
other qualification limits remain as recorded in the fork qualification.

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
