# Tool input validation candidate

Fork only. Base `81825c4`. Wrong types currently look like omitted arguments
in several handlers. Compile and cache the existing advertised schemas, keeping
semantic checks in handlers. Use jsonschema 0.60.2 with default features off:
no HTTP/file resolver or TLS. GUI-only optional dependency; engine unchanged.
It declares Rust 1.85 and MIT licensing. Cargo resolves 33 new locked packages:
this is a dependency-cost tradeoff for standards-compliant validation rather
than maintaining an incomplete schema implementation.

Reconcile known schema/handler discrepancies explicitly: satin_underlay null
already resets, and null coordinate pairs already mean omission. Preserve these
in schemas. Root null remains empty arguments for backward compatibility, with
required properties still enforced. Keep numeric clamping in handlers; do not
add new range limits except nonnegative document indices. Reject wrong types,
missing required fields, invalid enums, fractional integers and malformed pairs.

Resolve batch references before validating each child tool's arguments. Validate
only batch envelope and argument-object shape before reference resolution.
Keep the existing top-level unknown-key check and its helpful wording.

Proof: cheap invalid-argument table first, then preservation tests for omission,
null resets, clamping and valid/invalid batch references; finally MCP tool tests.
Rollback: revert candidate including manifest/lock/notices. No protocol migration,
caller authority changes, domain-rule rewrite or upstream submission.

Validation: unpatched `check_args` FAILS the new table by silently accepting
`add_text.size_mm = "large"`. Candidate input tests: 3 PASS; MCP tools: 14 PASS.
Tests exercise actual dispatch, omitted fields, clamped values, null resets,
and references that resolve to valid indices or invalid numeric arguments.
All schemas compile through the cached validator. A null simplify level is also
preserved as automatic simplification, matching its existing explicit handler.

License generation: installed cargo-about 0.6.4 cannot run the repo command.
An isolated 0.9.3 tool under `.git/license-tools` generates the configured Mac
dependency notices successfully after adding MIT-0 (borrow-or-share) to accepted
licenses. Used the existing renderer, normalized Windows output line endings,
and appended the actual lockfile digest. This qualifies notice generation, not
a Mac build. Independent review, Rust 1.95 execution, Mac and hardware tests
remain pending/not run.
