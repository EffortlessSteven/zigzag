# Current fork review work

Fork only; no upstream submission or release is authorized. The integrated
candidate is `review/fork-candidate`; the last full studio run at `aaa9755`
had 174 pass, 2 base-reproduced failures, and 3 ignored tests. All 28 MCP tool
tests passed. Raw review inputs/results are retained locally under
`.git/qualification/`.

New source/test review findings to settle before calling candidates ready:

- PR #2: respect the existing destination's OS write access before replacement;
  permission-mode copying alone allows a Unix rename over a read-only file.
  A non-root Linux probe confirms the distinction; Rust Unix execution remains
  unrun. Also carry post-commit durability uncertainty through document identity
  and Studio status/recovery state. Disk commit must not leave the old Save As
  identity, and uncertainty must not discard recovery state.
- PR #3: deletion misses the cheap name-cache refresh. Delete then edit/delete
  by a previously returned name can target the wrong survivor or panic, bypassing
  rollback. Reproduce both, fix the refresh, and test rollback plus usable state.
- PR #4: parameter errors must preserve an already validated request ID; keep
  notification suppression and limit envelope claims to fallback error replies.
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
