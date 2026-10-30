# Queued call expiry candidate

Fork only; base `81825c4`. HTTP currently waits 120 seconds, but a tool queued
behind UI work can execute after that wait expires. A retry may therefore
duplicate an edit. This is an unproven source finding until the test below runs.

Inject the existing wait duration into server construction for a deterministic
real-HTTP test. Withhold UI dispatch, set a zero wait, receive the timeout, then
resume dispatch. The original code should execute the edit; the candidate must
refuse to start it. No two-minute sleep or timing race needed.

Use a small shared execution lifecycle at the queue boundary. UI start and HTTP
expiry arbitrate on the same state; a queued/expired call cannot later start.
If execution already started, timeout reports outcome unknown and advises
inspection before retry. It does not claim cancellation or roll back commits.
Keep the normal 120-second wait. Test deadline expiry, successful completion,
expired queued edits, and in-progress uncertain outcomes.

This does not add durable request deduplication, active-operation cancellation,
document targeting, authentication, HTTP worker limits or protocol revisions.
Revert this candidate to roll back. Independent review precedes any upstream
decision by Steven.

Validation: real HTTP regression with only wait-duration injection FAILS against
the original lifecycle: `expired queued edit executed after timeout`. Candidate
passes. Execution lifecycle tests (2) PASS: both ordering cases (expiry before
start, start before timeout), deadline admission, no double start and later
completion of an uncertain operation. MCP tools (12) PASS, including the normal
HTTP workflow. Windows/rustc 1.98.0, locked dependencies. No sleeps in the new
tests. Independent review, Mac and hardware checks remain pending/not run.

Final race review adds a real HTTP start/commit gate: execute the edit, observe
the four committed shapes, withhold completion/reply, and let the injected
five-second response wait expire. The reply must say outcome unknown, never
no-effect cancellation. This is a preservation/integration test, not a claim
that every part reproduces on the old API.

A negative control replaced the shared execution Arc with separate UI state.
The new test FAILS: a committed operation is falsely reported as expired before
starting. Restoring shared state makes it pass. Review also checked deadline
sampling under the admission lock, expiry-first/start-first ordering, completed
reply races, and poisoned-state behavior. No production change was needed in
this pass. This is implementation-owner review; independent approval is pending.
