# Recovery sidecar cleanup

The independent review found an orphan after committed recovery warning followed
by successful Save. The regression reproduces it on e5b5d65: successful Save
retained recovery (inactive=false). No edit loss is claimed.

Track three in-memory states explicitly: None (no known recovery file), Current
(revision/snapshot), Retry (a committed file exists but needs rewriting). Keep
Retry through failed retries and tab swaps. A clean document can then use the
existing known-file cleanup; do not delete unknown orphan files or probe paths
on every frame. Metadata-only warnings do not invalidate Current SVG contents.
Restore reconstructs Current from read recovery bytes; disk format is unchanged.

Prove active/inactive normal Save removes its known sidecar after uncertainty,
failed retry retains that presence, and unknown orphan bytes are preserved.
Existing post-commit and pre-commit recovery tests retain their guarantees;
retry must produce Current, not merely cache identical metadata. Review the
small state patch independently, then rerun local/native qualification.
