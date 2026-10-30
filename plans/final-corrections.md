# Final narrow corrections before submission

Fork only. Two fresh review findings remain open at this point; previous passing
selections and completed reviews do not close them.

- #3 at 69570f6, thread PRRT_kwDOVchu4M6x7DjL / comment 4422511738:
  an edit followed by a final plan-reading step calculates twice because commit
  sets dirty again. Preserve the final execution planning state. Prove exactly
  one plan and one complete Undo, plus preservation when an edit follows a read.
- #13 at 4e078fc, thread PRRT_kwDOVchu4M6x7DkY / comment 4422511891:
  parent test accepts a successful child that ran zero tests or skipped its
  gated body. Add a completion witness after all child assertions and require
  it in the parent; demonstrate both false-success controls fail afterward.

Own #3 in the main checkout. Delegate #13 in a fresh isolated worktree, with
exclusive ownership of that topic and its targeted test/output records. Preserve
production adapter behavior and existing source/history. No force pushes.

After targeted red/green proof and review, carry both into existing clean
branches with normal commits. Refresh prepare/schema-after-batches over repaired
clean #3; preserve both composition regressions. This stacked candidate (#16) is
the preferred schema submission, with standalone #5 retained as fallback; do not
submit both. Qualify affected combined behavior, reconcile bodies/records and
close the two threads only after actual repairs. #2 remains verified. No upstream
submission or fork-main merge is authorized by this work packet.
