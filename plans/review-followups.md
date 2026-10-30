# Current fork review work

Fork only. No upstream submission or release is authorized. The combined
candidate is `review/fork-candidate`; [qualification](fork-candidate-qualification.md)
records exact heads, commands, results and applicability.

## Completed qualification

Windows Rust 1.95 and native Linux GUI selections pass, as do Windows core
without GUI and Windows/Apple-target GUI/Bluetooth compilation. Exact commands,
counts, candidate identity and scope are recorded in the linked qualification
record. Earlier failures remain in issue #9 and raw logs.

PRs #12–#15 repair the test/completion problems behind that qualification work:
worker completion versus frame count/render generations, agent exit versus stdout
EOF and real send wiring, simulator wall time versus observed stages, and recovery
writer completion versus a polling budget. Earlier save/batch/schema/bridge/expiry
repairs remain covered by the integrated run.

## Remaining work

- Independent review and Steven’s selection remain. The additional owner pass
  found no new blocker in the changed paths, and live threads are resolved;
  neither establishes independent approval of every head. No fork-main merge
  is implied.
- Native Mac runtime/filesystem, physical machine, native GL and crash durability
  checks remain unrun. Compilation and simulator tests are narrower evidence.
- [#10](https://github.com/EffortlessSteven/zigzag/issues/10): running-Studio
  envelope conformance, separate from bridge fallback serialization.
- [#11](https://github.com/EffortlessSteven/zigzag/issues/11): explicit client
  cancellation/earlier deadlines, separate from server expiry and disconnect.
- Embedded authority, user-scoped I/O, document targeting, authentication and
  protocol/release changes require their own settled workflow decisions.

Keep fork plans out of eventual upstream packages. The #3/#5 composition repair
now lives in the topics, and direct topic merge 6b35cab passes all 27 MCP tool
tests. #12 now observes admitted requests as well as accepted results. Both
fresh findings have been repaired and their counterexamples tested. The later
#2 FIFO replacement finding is also fixed in both working and clean topics, with
a bounded Linux regression and renewed full qualification. Refresh
proof for the actual extracted contribution before Steven authorizes submission.

Clean branches and their proof are in [contribution preparation](upstream-preparation.md).
Their product trees match the reviewed topics; extraction is not independent
approval or authorization to submit.

The integration-only static-schema preflight regression is now carried in the
optional clean schema-after-batches stack. Its 28 MCP tests pass and the
preflight-removal control fails; see contribution preparation for the exact head.
