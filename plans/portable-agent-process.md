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
