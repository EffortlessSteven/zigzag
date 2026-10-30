# Clean contribution preparation

Fork only. No upstream publication or fork-main merge is authorized.
Use baseline 81825c4812a6b9df45880da3a97fc7f63ad4e51b. Preserve all existing
working topics and their review history. Create new prepare/ branches from the
baseline containing the net product diff, excluding only plans/. The persistence
topic remains stacked on the clean save-result topic. Reuse the already clean,
single-commit documentation and vendored-test topics rather than duplicate them.

For each extraction, inspect the changed paths, verify that the complete tree
outside plans/ matches its original topic, and run diff whitespace checks. Keep
all code, tests, dependency locks and notices; do not shorten the patch by dropping
proof. New commit identity does not itself invalidate unchanged source evidence.
Run the focused save and bridge regressions on their clean branches. Record exact
heads and remaining qualification limits. Do not claim the independent topics
inherit every result from the combined fork.

The clean branches are preparation artifacts, not selected submissions. Do not
retarget existing fork PRs, rewrite their history, or open upstream PRs. Broader
policy, protocol, release and hardware work remains outside this lane.
