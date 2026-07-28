# openWEPP Gate Policy Contracts v1

Status: frozen execution-contract compatibility; nonblocking mapping input

Prospective authority:
`docs/decisions/0043-gate-planner-is-a-non-authoritative-advisory-linter.md`
and `docs/standards/testing-and-gate-strategy.md`.

This directory preserves the v1 machine-readable contract shapes while the
ADR-0043 roadmap removes or quarantines legacy execution consumers. It creates
no permission, admission, lifecycle, evidence, receipt, runner, CI, assurance,
or closeout authority. Agents determine and execute applicable canonical
requirements directly.

The schemas, valid fixtures, invalid-fixture descriptors, gate definitions,
execution matrix, and assurance registry retain their historical bytes and
closed vocabularies for compatibility and migration analysis. Literals such as
`BLOCKING`, `DEFERRED_TO_QUALITY_CI`, planner ownership, receipt, or attestation
inside those frozen contracts describe the retired v1 protocol. They have no
prospective effect and must not be invoked to authorize or block work.

`impact-map.json` is generation 18 and explicitly
`SCHEMA_ONLY_NONBLOCKING`. It retains direct mappings that help locate
independently applicable science and external-authority obligations. It has no
planner, lifecycle, or gate-policy authority rows. A future advisory mapping
replaces this compatibility shape in roadmap Order 3; until then, agents treat
its rows as optional cited input and verify applicability against the governing
source.

The v1 schema retains `policy_id` `ADR-0039` solely for compatibility until
Order 3. Generation 18's `policy_sha256` binds the current direct-execution
`docs/standards/testing-and-gate-strategy.md`.

Historical generation 17 is separately frozen in
`gate-policy/history/adr0039-generation17.json`. Historical verification
resolves the registry's exact commit, path, Git blob, and SHA-256; it never
derives old identity from the current live standard. Historical results keep
their original meaning but confer no prospective authority.

`assurance-registry.json` remains structured dependency/watch data. Direct
assurance governance owns applicability, validity, impact, review, approval,
publication, campaign transfer, and release transfer. A registry match is
information only: it cannot create a pending plan, lifecycle transition,
report mutation, review verdict, or public output.

Identity-bearing historical JSON continues to use the I-JSON, RFC 8785, and
SHA-256 conventions encoded by its original protocol. Preserving those
verification rules is historical integrity, not revival of the retired
planner.
