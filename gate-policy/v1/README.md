# openWEPP Gate Policy Contracts v1

Status: blocking normal-increment authority

Decision authority: `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md`

Operational authority: `docs/standards/testing-and-gate-strategy.md`

This directory contains the machine-readable contracts for deterministic
gate planning and evidence. `schemas/` defines strict JSON Schema Draft 2020-12
documents. `fixtures/valid/` demonstrates admitted payloads;
`fixtures/invalid/` contains one-mutation descriptors that derive each negative
case from its positive counterpart and bind the intended instance/schema error
path. `impact-map.json` seeds non-Cargo ownership for the gate-policy authority
itself. `gate-definitions.json` supplies the planner's closed typed
argument-vector registry; it never contains shell source.

TESTGATE-PLAN-01 consumes these contracts for blocking normal-increment
selection and verification. TESTGATE-CI-01 executes the selected plan and
publishes its locally untrusted receipt. Normal increment authority additionally
requires the trusted GitHub workflow's verifiable artifact-attestation bundle;
TESTGATE-ASSURE-01 owns assurance planner integration.

Identity-bearing JSON uses I-JSON constraints, RFC 8785 canonicalization, and
SHA-256 as required by the operational standard. Derived ID fields are excluded
from their own digest payload; referenced predecessor and input identities
remain included. The v1 schemas constrain shape and closed vocabularies. The
verifier independently enforces canonicalization, digest equality,
DAG acyclicity/topological order, transition folds, transitive-root
reconstruction, exact Nextest and authority-suite inventory reconstruction,
immutable tool/environment bindings, attestation trust, and hermetic reuse.

The schemas nevertheless carry every input required for those future checks:
planner promotion/ownership and execution-context identities, typed assurance
axes, governed zero-work evidence, complete gate-node receipt snapshots,
canonical authority outcomes, a single typed receipt attestation subject,
discriminated campaign events, anchored backstop/certification state, and
immutable assurance-impact events. Schema conditionals reject locally
expressible contradictions; the source-level semantic guard additionally
rejects cross-field inventory, attempt, outcome, mutation-digest,
certification-head, receipt, and authorization inconsistencies. The verifier
implements those same semantic checks over arbitrary records.

In v1, `policy_id` is `ADR-0039` and `policy_sha256` binds the exact bytes of
`docs/standards/testing-and-gate-strategy.md`, the living operational policy
adopted by that ADR.

The v1 execution matrix is deliberately limited to the exact `rustc` host and
the default feature set. Any other target or declared non-default feature fails
closed until a later versioned matrix explicitly admits it.

`assurance-registry.json` is the planner-owned, versioned dependency/watch
registry for TESTGATE-ASSURE-01. Its report IDs must equal the canonical
`assurance/v2/catalog.yaml` report set. Loading is structural and fail-closed;
operators cannot preselect reports. Exact paths, component prefixes,
repository-rooted globs, contracts, Cargo packages, process/domain tags,
result procedures, and builder/schema surfaces are closed watch kinds.
Mechanical matches create pending exact-target plan records only: they do not
rewrite reports, results, lifecycle state, review authority, or public output.
When a resolution principal is known, `role_record_sha256` is the RFC 8785
digest of `principal_id`, `record_version`, and `role_id`; policy loading
reconstructs it from `assurance/v2/principals.yaml`. A missing principal and
digest is explicit incomplete ownership and yields `OPEN_UNKNOWN`.
Each registry report also binds `source_root` and the best available assessed
realization root (`realization_root`, otherwise `preapproval_realization_root`)
from its generated review lock. Policy loading rejects drift, so a `CURRENT`
integrity axis is never target-free while ordinary impact planning remains
read-only.
