# Observe UI planning requests and accepted results

Fork PR #12 / issue #9; clean baseline 81825c4. A loaded panel failure read the
old stitch count with planner.pending=true. Fixed frame counts do not establish
worker completion. The renderer generation also changes for transformed previews,
so it cannot measure planning frequency.

## Contract and implementation

The test-only completion helper advances real UI frames until dirty work and
pending planning clear, with a failing elapsed-time bound. It does not force
synchronous replan. Panel, Undo and drag assertions remain.

For each point/object drag, observe both the existing planner request generation
and the test-only accepted-plan count. Neither may advance during dragging;
release advances each exactly once after the completion barrier. Accepted results
exclude stale worker generations; rendering-only changes are not plans. Naming
the counter accepted_plans makes that distinction explicit. The planner's test
accessor uses its existing generation, not new production telemetry state.

## Proof

- Focused drag test PASS on Windows, including deliberate worker scheduling.
- Both planner tests PASS. Controlled worker channels demonstrate two admitted
  requests with only one accepted completion, without relying on scheduler timing.
- Marking point-drag updates dirty FAILS with `planning was admitted while
  dragging points`.
- Marking object-drag updates dirty FAILS with `planning was admitted while
  moving the object`.
- The earlier missing-commit control fails the release assertion; no assertion
  was removed to obtain green. The current admission checks additionally detect
  extra work even when stale results are discarded.

Production planning, render invalidation and supported UI behavior are unchanged.
Integrated qualification must include these newer tests. This fork-only plan is
omitted from any eventual upstream package; it is not a hardware/runtime claim.
