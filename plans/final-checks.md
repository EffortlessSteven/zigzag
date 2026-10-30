# Requested final checks

Fork only. Steven requested the outstanding review and native Mac checks and
accepted proposing jsonschema for #5. Keep that dependency; do not redesign the
validator or silently impose a new package constraint.

- Explicitly request CodeRabbit on #2 at 77d49f8; automatic review skipped the
  non-default base. Open a draft fork review view of prepare/schema-after-batches
  against prepare/current-document-batches and request review of that exact
  candidate. This is an alternate view of #5, not an additional contribution.
- Run one bounded macOS-hosted library qualification from a fork-only branch
  based on clean #2. Bootstrap through a push limited to that qualification
  branch; no default-branch mutation, upstream changes, releases or broad matrix.
  Pin actions, use read-only contents permission and no retained checkout token.
  Install Rust 1.95 and use the root Cargo.lock. Record native OS/architecture,
  source identity, command, result and logs as a workflow artifact.
- Inspect failures/findings and repair their owning seam if required. Existing
  Linux FIFO coverage remains Linux-only; a Mac run covers the regular-file,
  permissions, symlink and document-save paths available there.

Completion requires actual review/check outcomes or an explicit external block.
Do not call review requests approvals, cross-compilation execution, library tests
GUI qualification, or simulator tests physical-machine evidence. Keep all working
and clean candidates current if a correction changes product code.
