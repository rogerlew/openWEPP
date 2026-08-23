# SC-SNOWENERGY-001 v15 independent review — Agent A

Evidence: `Static:` review of the uncommitted working tree based on
`cf178f5a41313dc71416e68e654a9aa71f72a51f`; no compile, test, strict-binding,
or assurance command result is claimed here.

Scope: scientific authority alignment, invariant soundness, evidence labels,
Binding Exposure Index conservation, lifecycle consistency, contract-derived
tests, and the Option-A OFE-ground runtime binding.

## Summary

The Option-A physical rule is coherent with the single persistent Stage 3
lane/OFE column: flux-like tile operands are converted exactly once with
`sum(f_i X_i)`, the covered subset is not renormalized, and incomplete mixed
open/covered execution fails closed. The amended frontmatter and registry now
truthfully agree on `v15 / in_review / draft / pending`.

The revision is not promotion-ready. A stable invariant ID is reused for two
different obligations, the new binding residue is absent from the canonical
tables and Binding Exposure Index, and the implementation still treats common
lane snow-state operands as tile-weighted quantities. The receipt also cannot
independently prove its declared source sets or surface classes.

## Findings

### A-001 — Critical — `INV-SNOWENERGY-041` has two incompatible meanings

References:

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md:1102`
  assigns `INV-SNOWENERGY-041` to typed terminal numerical tolerances.
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md:1123`
  exposes that terminal invariant through the Binding Exposure Index.
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md:1343`
  reassigns the same stable ID to OFE-ground topology and area semantics.

Impact: Stable contract identity is ambiguous. Tests, guard mappings, package
claims, and future assurance/restart references cannot identify which
scientific obligation `SC-SNOWENERGY-001#INV-SNOWENERGY-041` denotes. This is a
canonical authority defect and blocks v15 promotion.

Proposed disposition: `accepted`. Preserve the previously released terminal
tolerance invariant as `INV-SNOWENERGY-041`; assign a new unused stable ID to
the OFE-ground invariant (apparently `INV-SNOWENERGY-042`), and update every
contract, package, test, and Binding Exposure Index reference consistently.

### A-002 — Critical — the v15 binding is not integrated into the canonical schema or Binding Exposure Index

References:

- The canonical invariant/guard table ends at `INV-SNOWENERGY-040` at
  `SC-SNOWENERGY-001.md:803-807`.
- The canonical producer/consumer table ends without `OBL-SNOWENERGY-C-018` at
  `SC-SNOWENERGY-001.md:809-837`.
- The v15 invariant appears only in the later Child 2C narrative table and lacks
  the required authority and evidence columns at `SC-SNOWENERGY-001.md:1336-1343`.
- The v15 obligation appears only as prose at
  `SC-SNOWENERGY-001.md:1345-1354`.
- The Binding Exposure Index row for Child 2C omits both new v15 bindings at
  `SC-SNOWENERGY-001.md:1110-1112`.

Impact: The revision violates the invariant-table, guard-map, and Binding
Exposure conservation requirements. Direct user authority is named only in the
change log (`SC-SNOWENERGY-001.md:1385`), rather than being given an authority
anchor and `[DIRECT][Static]` evidence on the binding invariant. A binding
package addendum is therefore not fully promoted into canonical IDs, and guard
coverage cannot be audited.

Proposed disposition: `accepted`. Add a direct-user Option-A authority anchor;
place the newly numbered invariant in the canonical invariant and guard-map
tables with evidence and typed failure posture; place C-018 in the canonical
consumer table; add the current package as an active Binding Exposure row (or
amend the appropriate row) mapping both IDs; and add explicit test-vector
obligations for unequal covered tiles, mixed open/covered completeness,
duplicate/missing tiles, basis/topology substitution, and terminal projection.

### A-003 — High — the topology closure tolerance is invoked but not admitted

References:

- `SC-SNOWENERGY-001.md:1249-1251` permits closure within an “admitted topology
  tolerance” but provides no value, units/dimensionless declaration, provenance,
  or named tolerance ID.
- Runtime hard-codes `1.0e-12` at
  `crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs:767`.
- The covered runtime repeats the same literal at
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs:552` and
  `:602`.

Impact: Bounded normalization/acceptance is not contract-authorized with a
threshold and provenance, and three copies may drift. Because topology closure
controls whether physical surface area is omitted, silently widening it could
hide missing OFE-ground flux.

Proposed disposition: `accepted`. Define one dimensionless topology closure
tolerance in the contract's tolerance table, state that it is an acceptance
residual only and never renormalizes fractions, map it to a named implementation
constant/helper, and test exact sides plus a materially incomplete set.

### A-004 — High — one-column snow state is incorrectly area-averaged across tile contributions

References:

- The contract establishes exactly one persistent lane snow column at
  `SC-SNOWENERGY-001.md:1245-1248`.
- `LaneBoundaryContributionV1` carries per-tile `snow_temperature_k` and
  `latent_heat_j_kg` but no beginning Stage 3 state identity at
  `snow_stage3_terminal_handoff.rs:641-653`.
- Validation area-weights both state operands at
  `snow_stage3_terminal_handoff.rs:752-765` and derives an effective latent heat
  with a `1e-15` regime switch at `:794-800`.
- Runtime constructs the same weighted/effective values at
  `v9_real_consumer_shadow.rs:631-676`.

Impact: This permits different snow temperatures for tiles that allegedly
consume one lane column, and the effective-latent switch is ill-conditioned for
opposing vapor fluxes. It breaks the intended exact identity
`sum(f_i V_i L_s(T_s)) = L_s(T_s) sum(f_i V_i)`.

Proposed disposition: `accepted`. Bind the common beginning Stage 3 lane-state
digest in every contribution; require bit-identical snow temperature and latent
heat across the lane; copy those common operands into the aggregate instead of
weighting them; reject mismatch and opposing-vapor mutation cases.

### A-005 — High — aggregate receipt identities and open/covered provenance are self-declared

References:

- `LaneStage3BoundaryReceiptV1` declares four aggregate source identities at
  `snow_stage3_terminal_handoff.rs:661-665`.
- Contributions carry only an untyped final digest at
  `snow_stage3_terminal_handoff.rs:641-652`.
- `validate_body()` checks aggregate identities only for nonzero values at
  `snow_stage3_terminal_handoff.rs:698-705`; it cannot reconstruct them from the
  ordered contributions.
- Runtime currently groups only canopy receipts at
  `v9_real_consumer_shadow.rs:567-593`.

Impact: Rehashing an internally inconsistent receipt succeeds, and a random
nonzero digest can stand in for an open-snow producer. The arithmetic unit case
does not prove a complete typed mixed-surface consumer or equality between
configured surface class and producer definition.

Proposed disposition: `accepted`. Use a closed covered-canopy/open-snow boundary
class (preferably typed receipt variants), carry all source identities per
contribution, and reconstruct each ordered aggregate set digest in validation.
Bind configured surface class and model-definition identity and add digest/class
substitution tests. Keep the current runtime fail-closed until a real open-snow
producer exists.

### A-006 — High — the lane identity is deterministic but not canonical framed identity

References:

- `snow_stage3_terminal_handoff.rs:810-855` hashes a positional custom byte
  stream with little-endian values and untagged fields.
- The package disposition currently calls it a “closed framed” preimage in
  `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts/review-finding-disposition.md:15`.

Impact: The implementation is deterministic, but it is not the repository's
tagged `OPENWEPP_CANONICAL_FRAMED_SHA256_V1` scheme and has no normative wire
definition/test vector in v15. Calling it canonical overstates evidence and
risks freezing an adopter-specific implementation detail into restart or parent
authority.

Proposed disposition: `accepted`. Register an adopter-specific canonical framed
domain and use the canonical helper, or normatively specify the complete
alternative wire (domain/version, tags, widths, endianness, collection ordering,
optionality) with fixed test vectors. Until then label the digest deterministic
and noncanonical and prohibit restart/parent-authority adoption.

### A-007 — Medium — the new contract test cannot detect the canonical defects

References:

- `tests/integration/snow_stage3_shared_carrier_authority_contract.rs:153-178`
  checks only broad substring presence for lifecycle fields and IDs.

Impact: The test passes with the duplicate `INV-SNOWENERGY-041`, with C-018
outside the canonical obligation table, without a Binding Exposure mapping, and
without a declared tolerance. It therefore does not constitute a sufficient
contract-derived binding gate for v15.

Proposed disposition: `accepted`. Add structural assertions for unique
invariant/obligation IDs, exact canonical-table rows, guard-map membership,
Binding Exposure mapping, declared tolerance, registry/frontmatter equality,
and the required negative/runtime vectors.

### A-008 — High — promotion gates are not yet evidenced

References:

- Frontmatter and registry correctly retain review lifecycle at
  `SC-SNOWENERGY-001.md:4-17` and
  `docs/specifications/science-contracts/index.md:67`.
- Exact-head Rust, formatting, strict contract binding, and assurance identity
  were explicitly not run for `cf178f5a` in
  `artifacts/gate-results.md:170-188`.
- At review time, the required contract-cycle directory was not complete and
  this file is the first Agent A review artifact.

Impact: The truthful `in_review/draft` lifecycle is correct, but v15 cannot be
promoted or described as released until dual review, disposition, amendments,
dual verification, exact-head validation, and assurance verification close.

Proposed disposition: `accepted`. Retain `in_review/draft/pending`; complete the
full contract-cycle artifacts and exact-head gates after resolving findings;
rerun typed assurance adoption after final contract bytes; promote only after
both verifiers pass.

## Binding Exposure Index conservation verdict

`FAIL`. The active Option-A binding residue from the current package is not
mapped in the Binding Exposure Index, and its apparent invariant ID collides
with an existing indexed terminal invariant. No v15 promotion is permissible
until stable-ID uniqueness and exposure mapping are repaired.

## Lifecycle verdict

`PASS` for the current review state only: contract frontmatter and registry
truthfully agree on `v15 / in_review / draft / pending`. Promotion readiness is
`FAIL` until the findings and mandatory cycle gates close.

## Final recommendation

**HOLD**

Retain Option A and the no-renormalization/fail-closed runtime direction. Do not
promote `SC-SNOWENERGY-001@15`, adopt the lane receipt into restart/parent
authority, or proceed to precipitation/soil-heat work until A-001 through A-007
are dispositioned and verified. A-008 then closes only with the complete dual
review/disposition/verification and exact-head assurance/validation evidence.
