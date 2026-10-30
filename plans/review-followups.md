# Current fork review work

Fork only; no upstream submission or release is authorized. The integrated
candidate is `review/fork-candidate`; the last full studio run at `aaa9755`
had 174 pass, 2 base-reproduced failures, and 3 ignored tests. All 28 MCP tool
tests passed. Raw review inputs/results are retained locally under
`.git/qualification/`.

Current review disposition (fork only):

- PR #2: repaired in `0c0ee09`. Native uid-1000 Linux reproduction exposed
  read-only replacement; the shared writer now performs a non-truncating OS
  access check. A typed post-commit outcome commits document identity while
  Studio retains recovery and reports uncertainty. Linux and Windows library
  suites each pass 142 tests with one ignored. Save/export checks pass; session
  suite remains intermittent (8 pass, 1 fail; isolated retry passes), issue #9.
- PR #3: stale deletion names repaired in `e0eb013`; delete/edit and delete/delete
  regression witnesses passed after cheap cache refresh. Static keys/container
  shape around result references repaired in `6530304` (21 tool tests pass).
- PR #4: invalid-parameter request IDs repaired in `d9db78c`. Three bridge tests
  pass on the integrated candidate; claims remain limited to fallback replies.
- PR #5: pin save(path: number/null) rejection on an opened, edited document,
  proving that disk bytes and unsaved work survive. Decimal integer handling and
  UTF-8 corrections are already in the candidate. Rust 1.95 GUI execution is a
  separate outstanding qualification requirement.
- PR #6: the new real-HTTP committed-outcome test and shared-state negative
  control address the started-call review gap. Explicit client cancellation and
  client deadlines are separate from ZigZag's server deadline; capture that
  follow-up without pretending disconnect means cancellation.
- PR #1: strengthen the modified-save preservation case with a real edit;
  normalization-before-write remains owned by #2. Input-type rejection is #5.

Review threads need evidence-backed replies/dispositions on the final heads.
Keep fork plans out of any eventual upstream package. Issues #9 and #10 track
the Windows test baseline and studio protocol envelopes. Source work is owned
by this lane; parallel reviews do not authorize another writer in this checkout.

Integration after these corrections exposed a schema/preflight interaction:
checking full schemas in the static-key path rejected a valid numeric result
reference before resolution. A new numeric-reference regression failed on the
combined implementation; name/container validation and full schema validation
are now separate. All 31 integrated MCP tests pass. Full Studio: 176 pass,
4 fail, 3 ignored. Two failures were reproduced on the original baseline;
two additional UI/replan assertions pass in isolation and remain unclassified
under issue #9. GUI/Bluetooth binaries check passes. The corrected bridge binary
selection runs 3 passing tests; an earlier zero-test selection is not proof.

Local skills: `pr-body`, `code-review`, and `pull-request` were revised together
for adaptive explanations, whole-contribution review, proof-class distinctions,
evidence applicability, publication authority and shared-copy readback. Their
format validators pass (UTF-8 Python mode required). No installed `review-pr`,
`finish-pr`, or Deliver Engineering Work procedure was found in the checked skill
roots; no unrelated repository procedure was changed.
