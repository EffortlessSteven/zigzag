# Requested final checks

Fork only. Steven requested the remaining checks, accepted jsonschema in #5,
and then explicitly authorized agents to complete independent review.

## Review outcomes

- Schema: a read-only reviewer examined 49fa259 against a17d2e3 and combined
  behavior against 81825c4. No actionable finding. Scope included transaction
  preflight/rollback/Undo, references, schema semantics and integer consumers.
  The applicable raw 28-test/negative-control records were inspected; no new
  Rust execution was claimed in that review. A review comment is on draft #16.
- Persistence: a separate reviewer examined e5b5d65 and found a P3 cleanup gap,
  not edit loss. Normal Save after an uncertain recovery write could leave its
  known sidecar orphaned. The owner regression reproduced the actual failure.
  Repair 74b345a distinguishes Current from Retry instead of treating absence of
  a durable cache as absence of a file. Failed retries and tab swaps preserve
  known presence; normal Save cleans it; unknown orphan bytes remain untouched.
  Metadata-only uncertainty preserves Current. The same reviewer examined the
  immutable repair and reported no new actionable finding, without editing or
  building. These are bounded independent source/test reviews, not formal
  upstream approvals or an authorization audit.
- CodeRabbit reviewed 77d49f8 without actionable comments. Newer requests were
  rate-limited. That service-specific limitation is retained; it no longer
  prevents the independent review work above. No newer bot result is claimed.

## Execution and proof

Current integrated source: `d4171921f3a566ffbd144a35ca0583aa20998a3e`. Clean #2 source
matches its working topic outside fork plans. [Qualification](fork-candidate-qualification.md)
records full Windows/Linux results and compile checks. Existing benchmark ignores
are retained. No source changes were made after the final reviewed code.

[Native Mac run 41045633785](https://github.com/EffortlessSteven/zigzag/actions/runs/41045633785)
PASS: macOS 15.8.2 ARM64, Rust 1.95, 143 library tests (one benchmark ignored)
and five GUI-enabled recovery/save tests. Workflow head 95fdf36 matches the clean
candidate except for its fork workflow. Environment, lock hash and source blobs
were downloaded and verified. Earlier runs 41039769416 and 41041036913 remain
applicable historical proof of their earlier sources, not current-head claims.

The cleanup regression fails on e5b5d65. Controls for deleting unknown recovery
and skipping Retry writes fail for the intended reasons; restored tests pass.
Local records are `.git/qualification/agent-*.json`, `recovery-state-*.json`,
raw logs and `native-macos-state/`. Reviewer summaries are stored separately
from execution receipts. No fresh native Mac run of #5 is claimed.

## Scope and cleanup

The Mac workflow remains only on qualification/macos-persistence: pinned actions,
read-only contents, no retained checkout token and one bounded job. Actionlint
passed. Three targeted runs qualified the changing persistence candidates; no
broad matrix or default-branch/upstream CI change was introduced. Reviewers made
no branch switches, commits or public posts; the owner records their findings.

Focused failure/state proof is sufficient for the current changed claims; see
the qualification record's proof mapping. Full native Mac Studio/GL interaction,
BLE radio/firmware behavior, physical stitch-out, power-cut survival and unusual
filesystems remain unclaimed broader qualification, not acceptance blockers for
these fixes. A read-only follow-up audit found no useful missing cheap persistence
test. No release, upstream publication or fork-main merge was performed.

A later focused independent cost/simplicity assessment of #2 at 74b345a found no
avoidable work. Its deliberate costs are the staged save copy and replacement
checks/syncs; recovery caching and settling remain effective, with no new
per-frame IO or document copy. This was static review, not a latency/allocation
benchmark. Source and applicable qualification are unchanged.
