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

- Steven’s contribution selection remains. Separate agent reviews of current
  #2 and the stacked #3/#5 source are complete; the one cleanup finding was
  reproduced, repaired and reviewed. No fork-main merge or upstream approval
  is implied.
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

Current #2 recovery metadata and known-sidecar cleanup pass full Windows/Linux
qualification and focused native Mac execution. Independent reviews are complete
for #2 at 74b345a and the optional schema stack at 49fa259. CodeRabbit-specific
requests were rate-limited, so its earlier 77d49f8 result is not represented as a
review of the newer code. See final-checks.md for attribution and limits.

## Qualification boundaries

[Focused proof](fork-candidate-qualification.md#focused-proof-for-the-changed-behavior)
covers the relevant controllable failure modes. Physical hardware, full native
GL interaction, power-cut survival and exotic filesystems are unclaimed support
areas, not a remaining checklist for these fixes. Add a targeted test when a
specific changed contract has a gap; broader end-to-end checks belong to work
that changes or advertises those capabilities. The current coverage assessment
found no missing cheap persistence test.
