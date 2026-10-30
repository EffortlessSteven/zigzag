# Requested final checks

Fork only. Steven requested the outstanding review and native Mac checks and
accepted proposing jsonschema for #5. That dependency is retained; no bespoke
validator rewrite or unmeasured footprint claim was added.

## Outcomes

- PASS: CodeRabbit reviewed #2 at 77d49f8 and generated no actionable comments.
  Its raw report identifies that commit range; it is not approval of later code.
- REPAIRED: a separate fresh review found that session recovery lost the metadata
  for a committed-but-uncertain replacement. The original caller fails the new
  real-write/restore test. Working #2 e5b5d65 and clean #2 2760a36 publish flags for
  active/inactive recovery, retain retry eligibility and stop on pre-commit
  failure. Two negative controls reject caching uncertainty and handling only
  the active tab. Both new tests pass in current qualification.
- PASS: native macOS 15.8.2 ARM64, Rust 1.95.0, clean #2 library and focused
  recovery/save execution. [Initial helper run](https://github.com/EffortlessSteven/zigzag/actions/runs/41039769416)
  passed 143 library tests, one benchmark ignored. [Repaired caller run](https://github.com/EffortlessSteven/zigzag/actions/runs/41041036913)
  passed the same library selection plus all four selected GUI-enabled
  recovery/save tests. Downloaded logs, exact source blobs and lock hash match
  the clean candidate. This is native execution, not an Apple cross-check.
- PASS: current full Windows Rust 1.95 and native Linux qualification plus
  Windows/Apple-target GUI/Bluetooth compilation. Exact heads, commands and
  counts are in [qualification](fork-candidate-qualification.md).
- BLOCKED: requested independent re-review of #2's new recovery commit and of
  the optional stacked schema view [draft #16](https://github.com/EffortlessSteven/zigzag/pull/16),
  head 49fa259. CodeRabbit reports its one-included-review-per-hour quota is
  exhausted. Both explicit requests received rate-limit responses. Retry those
  exact heads after quota resets; no current-head independent approval is claimed.

## Scope and retained evidence

The native workflow lives only on qualification/macos-persistence, outside every
clean contribution branch. Push is limited to that branch, actions are SHA-pinned,
contents access is read-only and checkout credentials are not retained. Actionlint
passed. Two bounded jobs ran; no default-branch mutation, upstream CI adoption,
release, account-setting change or broad test matrix was introduced.

The native source at d33d30e differs from clean #2 only by this workflow. Artifacts
include OS, architecture, compiler, commit, file/session source blobs, root lock
hash and raw output. Local downloads are in `.git/qualification/native-macos-initial/`
and `native-macos-recovery/`; local full-suite records are `recovery-integrated-*`.
Earlier failures and negative controls are retained; they are not qualification
passes. Mac GUI interaction/full Studio suite, BLE runtime, hardware stitch-out,
crash/power-loss durability and unusual filesystems remain outside these checks.

## Additional independent agent review

Steven authorized agents after the CodeRabbit quota block. The schema reviewer
found no actionable issue at 49fa259 and inspected the applicable raw proof.
The persistence reviewer found a source-derived P3: committed-but-uncertain
recovery deliberately clears durable caches, so normal Save before a retry can
leave its sidecar orphaned. Reproduce active/inactive Save, then clean application-
owned recovery files when a document is clean regardless of cache presence.
Keep IO at session-write time, not every frame; preserve uncertain-file retry
semantics while modified. Requalify the caller and request the same reviewer’s
focused review of the repair. No unrelated policy or dependency change.
