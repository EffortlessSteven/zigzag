# Authorized upstream submission

Steven authorized submission after qualification on 2026-10-30. Use the prepared
clean branches and preserve the reviewed bodies. Submit schema once from 589cf50,
using the standalone schema title/explanation with 29-test stacked verification.

Rechecked upstream main: 81825c4812a6b9df45880da3a97fc7f63ad4e51b, unchanged.
All extracted product trees match reviewed topics outside plans/, clean diffs
pass whitespace checks, and prerequisite ancestry is verified. Preserve save-result
then persistence, and batching then schema, with actual upstream dependency links.
Cross-fork dependent PRs target main and initially include prerequisite commits.
No upstream merge, release, tag, or history rewrite is authorized.

All 12 PRs were created and independently read back: titles, complete bodies,
branch names and exact heads match the prepared payloads. Schema was submitted
once, from 589cf50, with the actual upstream batching dependency. No merge was
performed. The fork working candidates and fallback schema branch are retained.

| Fork contribution | Upstream PR | Exact submitted head |
| --- | --- | --- |
| #1 | [fix: propagate save failures through the MCP tool](https://github.com/simpsoka/zigzag/pull/1) | `d0654f6c400cf66a18ecd41a98dd28a2cf4407fe` |
| #2 | [fix: replace documents and exports atomically](https://github.com/simpsoka/zigzag/pull/2) | `54e15e49a2ca16df66e88c8c9ac15d638329df7a` |
| #3 | [fix: keep atomic batches inside the current document](https://github.com/simpsoka/zigzag/pull/3) | `f758277802df8e57b24f756c0d3031565c9dd5ff` |
| #4 | [fix: parse MCP bridge errors as JSON-RPC](https://github.com/simpsoka/zigzag/pull/4) | `c9f67b581855b0a222a53085c7451a768c557fa9` |
| #5 | [fix: enforce advertised MCP argument schemas](https://github.com/simpsoka/zigzag/pull/5) | `589cf50ccfdbdbfe7ef8efd1df86050eea18c03c` |
| #6 | [fix: prevent expired queued MCP calls from starting](https://github.com/simpsoka/zigzag/pull/6) | `9ca68dacfe7729a931d09fb8e073f7ab177e6243` |
| #7 | [docs: qualify unvalidated Skitch hardware support](https://github.com/simpsoka/zigzag/pull/7) | `430a2695fbb1b642728d829059645872f39aa3d2` |
| #8 | [test: declare JSON dependency for vendored geometry tests](https://github.com/simpsoka/zigzag/pull/8) | `3ac4f3affb06323743a33cea8e70c551a6621dc1` |
| #12 | [test: wait for completed stitch plans in UI checks](https://github.com/simpsoka/zigzag/pull/9) | `0ace88ce8178f6b4f269a730d790d823ba56040f` |
| #13 | [fix: preserve agent output until stdout finishes](https://github.com/simpsoka/zigzag/pull/10) | `558bf9e90eb211798cc31f99a20da4494a52a1c0` |
| #14 | [test: control simulator stages with an opt-in manual clock](https://github.com/simpsoka/zigzag/pull/11) | `a0c483861c9cea91a3fe72892d2064daa88da0e0` |
| #15 | [test: join the sent-design writer before restoration](https://github.com/simpsoka/zigzag/pull/12) | `4e1ef2a1235674be1dc02dfd8f7441923049ec0d` |

Dependencies: upstream #1 → #2 and #3 → #5. Both dependent PRs target
main and explicitly include their prerequisite until it lands. Other summaries
were preserved. Local request/response/readback records are under
.git/qualification/upstream-*.json. Submission does not imply upstream acceptance
or hardware, interactive GL, or installed-agent qualification.
