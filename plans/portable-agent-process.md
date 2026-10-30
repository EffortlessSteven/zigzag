# Portable embedded-agent process fixture

Fork issue #9. Base: 81825c4. The existing process test creates a Unix shell
script and tries to execute it on Windows, then mutates the test process's
global ZIGZAG_CLAUDE environment unsafely. It fails before testing streamed
agent output. This is separate from design-agent permission policy.

Extract the existing private child-launch/streaming path from send into a small
private method. Production send still performs lookup, command construction and
prompt handling. The test executes a child of the just-built test executable,
selecting a private fixture test and marking only the child's environment. The
child reads actual stdin and emits serialized stream events. Inspect generated
Claude flags separately; do not claim real Claude/Codex compatibility.

Acceptance: preserve command construction and prompt semantics, consume actual
child output, verify session/prompt echo/completion, avoid global env mutation
and shell-specific fixtures, and fail on spawn/timeout/wrong output. Keep the
existing stream behavior unless a separate discriminating test reveals a defect.
Proof: focused agent tests on Windows, native Linux if GUI dependencies permit,
and the integrated Studio suite. No dependency or SDK/capability redesign.
Rollback: this topic alone. No upstream publication is authorized.

Windows proof: original process test fails on 81825c4. After extracting only
start_command and replacing the fixture, all 14 agent tests pass. A negative
control writing `lost prompt` instead of the supplied prompt fails with the
actual echoed child output, proving stdin is exercised. The restored test passes.
No global environment mutation or executable script remains in this fixture.
The child gets its marker through Command::env and reads stdin to EOF. Timeout
kills the owned child before returning an error. Real installed-agent permission
and CLI compatibility claims are outside this test.

The integrated Rust 1.95 run reproduced a real adapter race: the test saw only
Done("") and no streamed session. Exit arrives from the stderr/process-wait
thread, independently of stdout delivery, and poll drops Run on that event.
Revise this topic into the complete output-completion fix: retain process exit
until stdout EOF is observed after all lines. A deterministic event-order test
must reject exit-before-lines on the old adapter and cover both arrival orders.
The portable child fixture remains the end-to-end proof of the same behavior.
Client permissions and agent command construction are still out of scope.

The deterministic exit-first test failed before EOF tracking (`process exit
discarded unread stdout`). Run now retains exit independently of stdout EOF;
completion waits for both. The test also covers EOF-before-exit, late session,
assistant output, and the actual streamed completion rather than generic Done.
All 15 standalone Windows agent tests pass. Earlier native Linux tests passed
before the EOF correction; final integrated cross-platform qualification follows.

Review identified that direct start_command coverage bypassed send's executable
lookup/handoff. Restore the real send path with a small std-only Rust fixture
compiled during the test. Run the send test in a child test process with scoped
ZIGZAG_CLAUDE/PATH settings, including a safe decoy executable so wrong lookup is
observable without invoking an installed agent. Verify actual executable identity,
stdin bytes, session and completion. No global environment mutation or production
launcher injection is needed; remove the now-unnecessary private extraction.
The fixture requires rustc, as the project's Cargo build already does. Keep the
event-order regression and final source qualification on each supported runner.

The real-send fixture rejects two negative controls: ignoring the configured
executable selects the safe decoy and fails identity comparison; replacing send's
prompt fails the recorded stdin comparison. The selected executable also checks
the actual strict-MCP and disabled-built-in flags. The private launch extraction
is removed; production send uses its original inline flow plus EOF tracking.

All 15 standalone Windows agent tests pass with real send coverage restored.
