# Wait for completed stitch plans in UI tests

Fork issue #9; base is the integrated fork candidate e73c458. Keep this separate
from agent process portability and session persistence timing.

Research: Studio switches future plans to a worker when an observed synchronous
plan exceeds 12 ms. UI tests assert stitch counts/replan counters after a fixed
number of frames; those frames do not establish that a worker result arrived.
The same inputs can therefore pass alone and fail under load. Confirm pending
worker state at the stale-plan assertion before choosing the fix.

Use a test-only bounded completion helper that advances UI frames until dirty
work and worker pending state are both drained. Retain the existing document,
stitch and Undo assertions; do not call synchronous replan to make them pass.
Exercise the asynchronous route deliberately where relevant. A timeout is a
failure. Production scheduling, worker implementation and renderer are non-goals.

Proof: reproduce the current decorative-fill failure, run the affected panel and
drag tests, then qualify the integrated Studio suite. Rust 1.95 GUI/Bluetooth
compilation already passed on e73c458; test execution remains separate evidence.
Rollback: this test-only topic. Other baseline failures remain separate work.

Loaded witness: 177 passed, 4 failed, 3 ignored. The decorative-fill assertion
printed `Meander: 2465; pending=true, slow=true`. App and path-drag plan counters
also failed while each could pass alone. The new barrier uses real frame polling
until both dirty admission and pending completion clear, with a failing 10-second
bound. It does not call replan or disable the worker. The panel fixture explicitly
requests the worker route. Original negative drag and Undo assertions are retained.

A loaded rerun exposed the same stale-plan assumption in document toggles. It
also showed why simply waiting was insufficient for the move test: `replans`
is a renderer generation and is incremented by `shift_planned` for a temporary
transformed preview, then again by actual worker completion. Add a test-only
completed-plan counter at the synchronous and accepted-worker completion sites;
retain exactly-one-plan assertions against that counter. Render invalidation and
planning behavior remain unchanged. No worker result is forced synchronously.

After correction, the loaded integrated Studio run has 180 passed, 1 failed,
3 ignored; only the separate non-portable agent fixture fails. The restored
real drag test passes after a negative control removing point-drag commit failed
its exactly-one-completed-plan assertion. No timeout was treated as a pass.
