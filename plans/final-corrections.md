# Final narrow corrections before submission

Fork only. Both P3 findings are repaired and independently reviewed; no upstream
submission or fork-main merge is authorized.

- #3: source f67a48e, clean f758277. Transaction commit preserves the planning
  state left by the final step. The original regression failed with two plans
  instead of one. The repair passes 22 standalone tool tests, including one-plan,
  one-Undo restoration and a later edit that still requires another plan.
- #13: source 6d42865, clean 558bf9e. The child writes an exact completion witness
  only after all assertions; the parent requires it as well as successful exit.
  All 15 agent tests pass. Both unmatched-test and missing-enabling-flag controls
  fail because the witness is absent. Production adapter behavior is unchanged.
- Preferred schema stack #16: 589cf50 over repaired clean #3. All 29 MCP tool
  tests pass, including both composition regressions and final-plan reuse.
  Standalone #5 remains a fallback; submit the schema contribution once.

Independent read-only review of both frozen deltas found no actionable issue.
Review examined plan invalidation after every admitted edit, rollback/history,
marker placement, stale-marker isolation and both false-success controls.

Combined code head 74058d8edf50d3fd8f60663f6a006fa3b320bfb7 passes Windows
Rust 1.95 and native Linux qualification: 188 Studio tests on each, three bridge
and 144/145 library tests respectively. Existing ignored benchmarks remain
ignored. Windows GUI/Bluetooth and Apple-target source checks pass. Exact commands
and local receipts are in [qualification](fork-candidate-qualification.md).
The unchanged #2 native macOS persistence evidence remains applicable.

Local discriminating evidence: .git/qualification/batch-final-read-{before,after}.json,
fixture-witness-results.json (positive/fault/fault/restored exits 0/101/101/0),
final-schema-stack.json and corrections-{windows,linux,gui-ble,apple-check}.json.
These local artifacts are not public CI evidence. Installed-agent interoperability,
interactive GL, physical hardware and power-loss behavior are not established.
