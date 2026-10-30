# Bridge error candidate

Fork only; base `81825c4`. The bridge scans JSON text to recover an ID and
interpolates it and an error message. Valid string IDs with commas/braces and
escaped quotes do not round-trip. Use the existing serde_json dependency.

The advertised MCP 2025-06-18 contract requires string/integer request IDs and
one message per POST. Reject arrays (including empty arrays), null/fractional
or composite IDs, wrong JSON-RPC versions, missing methods, and non-object
params. Do not implement generic JSON-RPC batch replies here. Valid
notifications have no reply; malformed JSON yields parse error with null ID.

Official contracts:
- https://modelcontextprotocol.io/specification/2025-06-18/basic
- https://modelcontextprotocol.io/specification/2025-06-18/basic/transports

Test escaped string/integer IDs, nested misleading IDs, notification suppression,
malformed JSON, invalid envelopes, and exact escaped message content. Keep the
existing application-unavailable wording. HTTP framing, studio envelope
validation, modern protocol support, and authentication are separate seams.
Rollback: revert this candidate. No new dependency or upstream submission.

Validation (Windows, rustc 1.98.0, locked): the escaped-ID/message test FAILS
on unpatched source with `expected comma or closing brace`. Candidate
`cargo test --locked --bin zigzag-mcp`: 2 PASS. Independent review and
cross-platform execution remain pending. No claim of full MCP conformance.

Numeric-encoding review: a whole decimal encoding (`17.0`) is an integer value,
not a fractional ID. Added a round-trip test that initially failed the new
envelope check; corrected it to accept whole numeric values while still rejecting
`1.5`. The value is serialized unchanged rather than coerced into another ID.

Notification review: invalid parameters previously produced a response even
when a recognized notification envelope had no ID. The new test reproduces
that response, then verifies suppression for array/null/scalar parameters while
the equivalent requests still receive their invalid-request errors. Notification
direction is now checked before parameter validation.
