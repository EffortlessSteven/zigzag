# ZigZag fork campaign

All development and review stays in EffortlessSteven/zigzag. Steven must explicitly
authorize any upstream submission. No upstream PR, issue, merge or release is part
of this lane. The original numbered suggestions are an investigation inventory,
not a delivery quota or an agreed upstream architecture.

## Current candidates

| Fork PR | Responsibility | Current state |
| --- | --- | --- |
| [#1](https://github.com/EffortlessSteven/zigzag/pull/1) | Preserve actual save results through Studio and MCP | Focused tests pass, including real unsaved work; no write-mechanics change. |
| [#2](https://github.com/EffortlessSteven/zigzag/pull/2) | Stage document state and replace whole files | Stacked on #1; Unix write-access and post-commit identity/recovery findings repaired. Apple sources cross-check; native Mac filesystem qualification remains. |
| [#3](https://github.com/EffortlessSteven/zigzag/pull/3) | Current-document atomic transactions | Grouping Undo, history-copy cost, deletion names, and reference-bearing static-key checks repaired. |
| [#4](https://github.com/EffortlessSteven/zigzag/pull/4) | Structured bridge fallback errors | Three tests pass; no claim of running-Studio envelope validation. |
| [#5](https://github.com/EffortlessSteven/zigzag/pull/5) | Advertised input types without silent defaults | Integral numeric encodings and actual unsaved-file preservation covered; Rust 1.95 tests pass; native Mac execution remains unrun. |
| [#6](https://github.com/EffortlessSteven/zigzag/pull/6) | Expired queued-call admission and truthful started outcomes | Real HTTP queued and committed paths tested; client cancellation remains separate. |
| [#7](https://github.com/EffortlessSteven/zigzag/pull/7) | Put hardware qualification next to its claim | Wording only; physical qualification remains unavailable. |
| [#8](https://github.com/EffortlessSteven/zigzag/pull/8) | Declare geometry tests' existing JSON dependency | 72 selected vendored tests pass; root test selection is different. |
| [#12](https://github.com/EffortlessSteven/zigzag/pull/12) | Observe completed UI stitch plans | Preserve Undo/drag assertions without depending on frame count or rendering generation. |
| [#13](https://github.com/EffortlessSteven/zigzag/pull/13) | Preserve agent output through EOF | Exit/EOF ordering repaired; real send lookup, flags, stdin and streamed completion tested. |
| [#14](https://github.com/EffortlessSteven/zigzag/pull/14) | Control simulator time explicitly in stage-sensitive tests | Normal simulation remains automatic; manual Session/UI lifecycle tests hold observed stages. |
| [#15](https://github.com/EffortlessSteven/zigzag/pull/15) | Join recovery writer before restoration assertions | UI persistence stays asynchronous; tests observe actual completion. |

These are open candidates, not accepted upstream contributions. Preserve useful
work, address findings on its owning topic, and integrate through normal merges.
Do not rewrite shared history. The combined review branch is `review/fork-candidate`.

## Boundaries and proof

Use the smallest complete behavior correction. Preserve successful workflows,
non-atomic partial progress, documented omission/reset/clamping, and truthful
pre- versus post-commit outcomes. A prior clean review does not suppress a later
finding. Source findings, reproduced defects, preservation tests and executed
qualification have different meanings.

[Current qualification](fork-candidate-qualification.md) records candidate heads,
commands, outcomes and gaps. [Review follow-ups](review-followups.md) records
remaining work. Earlier plan revisions are in Git history, not current readiness
claims. Keep these fork-only plans out of any eventual upstream topic package.

Remaining concrete investigations are [Windows test reliability #9](https://github.com/EffortlessSteven/zigzag/issues/9),
[Studio protocol envelopes #10](https://github.com/EffortlessSteven/zigzag/issues/10), and
[client cancellation/deadlines #11](https://github.com/EffortlessSteven/zigzag/issues/11).

Embedded authority/authentication, document targeting, HTTP limits, CI, release
inputs and protocol/SDK modernization remain separate design choices. Demonstrate
the useful authorized design workflow and actual enforcement, including nested
dispatch, before settling a capability model. Do not turn blanket tool removal,
a configuration string, a release manifest or a simulator into a stronger claim
than it establishes.
