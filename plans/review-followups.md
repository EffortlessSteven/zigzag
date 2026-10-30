# Current fork review work

Fork only. No upstream submission or release is authorized. The combined
candidate is `review/fork-candidate`; [qualification](fork-candidate-qualification.md)
records exact topic heads, applicable runs and their limits.

Current focused proof: 32 integrated MCP tests and the application durability
warning/recovery test pass. The broader Studio run has 176 pass, 4 fail, 3 ignored.
Do not call the full candidate green or independently approved on that basis.

## Remaining work

- [#9](https://github.com/EffortlessSteven/zigzag/issues/9): classify and repair
  Windows test reliability. Two full-suite failures also occur on the original
  baseline. Two additional UI/replan assertions pass in isolation; their cause
  remains unclassified. Machine-session restoration has also been intermittent.
- [#10](https://github.com/EffortlessSteven/zigzag/issues/10): examine running-Studio
  envelope conformance separately from bridge fallback serialization.
- [#11](https://github.com/EffortlessSteven/zigzag/issues/11): investigate explicit
  client cancellation/earlier deadlines using the existing admission lifecycle.
  Connection loss alone is not cancellation; started outcomes require honesty.
- Rust 1.95 GUI and macOS qualification remain unrun. Physical machine and crash
  durability qualification are separate from simulator/format/library evidence.
- Review the latest integrated candidate substantively; addressed older threads
  do not constitute a fresh approval. Reconcile any new valid findings.

## Repairs carried into the candidate

Destination write-access and post-commit Save As state/recovery are corrected.
Batch grouping Undo, history-copy cost, deletion names and static keys around
references are corrected. Bridge parameter errors preserve validated IDs and
notifications remain silent. Schema tests preserve real unsaved work for invalid
save paths; the result-only tests now make an actual unsaved edit.

Integration itself exposed premature schema validation of numeric references.
Static key/container validation and resolved-value schema validation are now
separate; both rejection-before-edit and valid-reference tests pass. Source and
review history retain the discriminating failures, without making every PR body
a running investigation log.

Keep fork plans out of eventual upstream packages. Embedded authority, user-scoped
I/O, document targeting, authentication and protocol/release changes require
separate settled workflow decisions; none is implied by these correctness fixes.
