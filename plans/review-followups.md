# Current fork review work

Fork only. No upstream submission or release is authorized. The combined
candidate is `review/fork-candidate`; [qualification](fork-candidate-qualification.md)
records exact heads, commands, results and applicability.

## Completed qualification

The current code passes Windows Rust 1.95 and native Linux selections: 143 library
tests, 3 bridge tests and 183 Studio tests per platform. Existing benchmarks remain
ignored (1 library, 3 Studio). Windows and Apple-target GUI/Bluetooth compilation
passes. Earlier full-suite failures are retained in issue #9 and raw logs.

PRs #12–#15 repair the test/completion problems behind that qualification work:
worker completion versus frame count/render generations, agent exit versus stdout
EOF and real send wiring, simulator wall time versus observed stages, and recovery
writer completion versus a polling budget. Earlier save/batch/schema/bridge/expiry
repairs remain covered by the integrated run.

## Remaining work

- Review the latest candidates substantively and select coherent contributions.
  Addressed older threads and owner verification are not fresh independent
  approval of every head. No fork-main merge is implied.
- Native Mac runtime/filesystem, physical machine, native GL and crash durability
  checks remain unrun. Compilation and simulator tests are narrower evidence.
- [#10](https://github.com/EffortlessSteven/zigzag/issues/10): running-Studio
  envelope conformance, separate from bridge fallback serialization.
- [#11](https://github.com/EffortlessSteven/zigzag/issues/11): explicit client
  cancellation/earlier deadlines, separate from server expiry and disconnect.
- Embedded authority, user-scoped I/O, document targeting, authentication and
  protocol/release changes require their own settled workflow decisions.

Keep fork plans out of eventual upstream packages. When combining #3 and #5,
retain the integrated preflight/schema composition repair; independent passing
branches alone do not establish their combined behavior. Refresh proof for the
actual extracted contribution before Steven authorizes submission.
