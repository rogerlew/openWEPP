# Independent contract review — Agent B

Evidence: **Static.** I inspected the current uncommitted v15 contract, lifecycle
registry, contract-derived tests, Option-A lane receipt implementation, package
review reference, and the normative science-contract authoring/specification
procedures. I did not rely on Agent A's review and did not execute Rust or
contract gates. Commands used for inspection (`rg`, `sed`, `nl`, and `git diff`)
do not constitute runtime validation.

## Findings

### B-01 — CRITICAL — `INV-SNOWENERGY-041` has two incompatible meanings

The existing canonical-obligation table assigns `INV-SNOWENERGY-041` to the
rule that terminal numerical tolerances are distinct and cannot repair identity,
support, or state
([SC-SNOWENERGY-001.md:1095](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1095),
[SC-SNOWENERGY-001.md:1102](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1102)).
The v15 amendment reuses that same ID for the OFE-ground lane topology and
terminal-liquid rule
([SC-SNOWENERGY-001.md:1336](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1336),
[SC-SNOWENERGY-001.md:1343](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1343)).
The Binding Exposure Index still maps the terminal-enthlapy package to
`INV-SNOWENERGY-041`
([SC-SNOWENERGY-001.md:1123](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1123)).

Impact: a receipt, test, review, or restart authority citing `INV-041` cannot
identify which invariant it claims. This breaks stable identity and silently
aliases two unrelated scientific/governance obligations. The new substring test
only proves that the duplicated token exists
([snow_stage3_shared_carrier_authority_contract.rs:151](../../../../../../../../tests/integration/snow_stage3_shared_carrier_authority_contract.rs#L151)).

Proposed disposition: **accepted**. Preserve the established terminal-tolerance
meaning of `INV-SNOWENERGY-041`; assign the OFE-ground rule the next unused
stable ID, and update every new contract, test, package-reference, disposition,
and assurance binding before verification.

### B-02 — CRITICAL — the Binding Exposure Index does not expose the v15 binding addition

The v15 authority originates in this covered-consumer package and adds both the
OFE-ground invariant and `OBL-SNOWENERGY-C-018`, but the Binding Exposure Index
has no row for this package or obligation
([SC-SNOWENERGY-001.md:1104](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1104),
[SC-SNOWENERGY-001.md:1110](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1110),
[SC-SNOWENERGY-001.md:1351](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1351)).
The introductory assertion that no binding residue exists "for version 2" is
also stale and does not describe the current v15 consolidation
([SC-SNOWENERGY-001.md:1106](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1106)).

Impact: Binding Exposure Index conservation is not satisfied even if the
mechanical strict checker accepts the current table. The active package-local
authority can be dropped or moved without a canonical anti-drop mapping, which
violates the mandatory binding-exposure workflow.

Proposed disposition: **accepted**. Add an `active`,
`flagged-binding-addition` row for this package mapping the newly assigned
OFE-ground invariant ID and `OBL-SNOWENERGY-C-018`, with dual-review and
verification status. Rewrite the stale introductory claim to describe the
current contract rather than version 2, then rerun strict binding exposure.

### B-03 — HIGH — the new non-trivial invariant lacks canonical provenance/evidence and guard-map placement

The main invariant/guard table ends at `INV-SNOWENERGY-040` and supplies a
source-authority column plus `[DIRECT]`/`[INFERENCE]` evidence tags
([SC-SNOWENERGY-001.md:780](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L780),
[SC-SNOWENERGY-001.md:803](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L803)).
The new OFE-ground invariant appears only in a later three-column amendment
table, with no citation anchor or evidence tag
([SC-SNOWENERGY-001.md:1336](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1336)).
The change log's generic "Direct user authority" label is not an authority
anchor or per-claim `[DIRECT][Static]` mapping
([SC-SNOWENERGY-001.md:1385](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1385)).

Impact: the scientifically coherent Option-A choice is not traceable through
the contract's canonical invariant schema, and its runtime/restart/terminal
guard surface is compressed into the generic carrier error rather than mapped
to explicit enforcement paths and gate impact. This is not promotion-ready
under the authoring procedure.

Proposed disposition: **accepted**. Add a stable authority anchor for the
prospective user decision and repository state/terminal semantics, record
`[DIRECT][Static]` versus derived `[INFERENCE][Static]` claims explicitly, and
place the new invariant and consumer obligation in the canonical invariant,
obligation, guard, alias/unit, and test-vector surfaces rather than only the
appendix-like amendment section.

### B-04 — HIGH — the admitted topology tolerance is undefined

The v15 prose permits tile fractions to close to one "within the admitted
topology tolerance," but names no tolerance ID, value, units, derivation, or
normalization authority
([SC-SNOWENERGY-001.md:1245](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1245)).
Runtime independently hard-codes `1.0e-12`
([snow_stage3_terminal_handoff.rs:767](../../../../../../../../crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs#L767)).

Impact: valid topology and rejection behavior depend on an undocumented scalar;
contract review cannot assess whether this is only representation roundoff or
an unauthorized bounded canonicalization. Restart equivalence may change if the
implementation literal changes.

Proposed disposition: **accepted**. Define a named, dimensionless topology
closure tolerance with provenance and explicit no-renormalization semantics;
map it to the runtime guard and exact-side tests. Identity, ordering,
cardinality, duplication, and area-basis checks must remain exact.

### B-05 — HIGH — implementation treats single-column state operands as tile-weighted fluxes

Option A declares one persistent Stage 3 snow column per lane
([SC-SNOWENERGY-001.md:1245](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1245)),
and the carrier section says Stage 3 owns the snow-surface temperature
([SC-SNOWENERGY-001.md:1240](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1240)).
Nevertheless, each tile contribution carries its own `snow_temperature_k` and
`latent_heat_j_kg`, and validation area-weights both
([snow_stage3_terminal_handoff.rs:642](../../../../../../../../crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs#L642),
[snow_stage3_terminal_handoff.rs:751](../../../../../../../../crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs#L751)).
It then changes latent-heat reconstruction behavior at a net-vapor threshold of
`1e-15`
([snow_stage3_terminal_handoff.rs:794](../../../../../../../../crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs#L794)).

Impact: one lane owner can be represented by mutually inconsistent snow states;
opposing tile vapor fluxes can produce a nonzero latent-energy sum with an
unstable effective latent heat. This conflicts with the single-column physical
model and makes the aggregate dependent on an unauthorised numerical branch.

Proposed disposition: **accepted**. Bind a common beginning Stage 3 lane-state
digest, snow temperature, and corresponding latent heat across every
contribution; require bit identity and use the common operands directly. Only
tile fluxes should receive area weighting. Add unequal/poisoned-state and
opposing-vapor vectors.

### B-06 — HIGH — the receipt/test binding does not prove the complete typed physical surface set

The contract requires covered-canopy and open-snow receipts for mixed OFEs and
rejects missing or duplicate contributions
([SC-SNOWENERGY-001.md:1249](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1249)).
The implementation contribution has no closed boundary class or model-definition
identity and carries only one final receipt digest
([snow_stage3_terminal_handoff.rs:642](../../../../../../../../crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs#L642)).
The lane's four aggregate source identities are only checked for nonzero values,
not reconstructed from contributions
([snow_stage3_terminal_handoff.rs:698](../../../../../../../../crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs#L698)).
The mixed arithmetic test consequently labels tiles "covered" and "open" while
supplying arbitrary nonzero digests through the same untyped constructor
([snow_stage3_terminal_handoff.rs:1974](../../../../../../../../crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs#L1974)).

Impact: a rehashed receipt can claim a complete physical topology while its
source-set identities are unrelated to the ordered contributions, and an
arbitrary digest can masquerade as an admitted open-snow producer. The test
proves `0.6 * 100 = 60`, but not the contract's exact-once typed surface join.

Proposed disposition: **accepted**. Introduce a closed covered/open boundary
class (preferably typed receipt variants), bind configured surface class and
model identity, carry enough per-contribution source identities to reconstruct
every retained set digest—or remove unreconstructable redundant aggregates—and
add substitution, omission, duplication, wrong-class, and real mixed-producer
tests.

### B-07 — MEDIUM — v15's contract-derived test is presence-based and cannot detect semantic/schema regressions

The new test searches whole-file substrings for lifecycle fields, invariant IDs,
formula prose, and registry prose
([snow_stage3_shared_carrier_authority_contract.rs:143](../../../../../../../../tests/integration/snow_stage3_shared_carrier_authority_contract.rs#L143)).
It does not parse frontmatter, establish ID uniqueness, bind the Binding
Exposure Index row, prove the named tolerance, or reject the old covered-area
basis and normalization formula across production code.

Impact: it currently passes the critical duplicate-ID state and would also pass
if required text appeared only in a changelog or unrelated section. It is not a
contract-derived semantic guard for promotion.

Proposed disposition: **accepted**. Add structured lifecycle assertions,
contract-wide stable-ID uniqueness, BEI mapping, exact invariant/obligation
table binding, named-tolerance binding, and source-level/runtime negative guards
against `CoveredTileGround` and covered-fraction division.

## Lifecycle and promotion disposition

The frontmatter and registry now consistently describe v15 as
`in_review`/`draft` with `last_reviewed: pending`
([SC-SNOWENERGY-001.md:1](../../../../../../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md#L1),
[index.md:67](../../../../../../../specifications/science-contracts/index.md#L67)).
That lifecycle is truthful and must be retained during amendment. The Option-A
OFE-ground equation and no-renormalization direction are scientifically and
architecturally coherent, but the stable-ID collision, incomplete Binding
Exposure Index, missing provenance/tolerance authority, and non-authoritative
implementation bindings prevent promotion.

## Recommendation

**HOLD.** Amend B-01 through B-07, rerun the exact-head contract, assurance,
format/compile/test, and strict binding-exposure gates, disposition both
independent reviews, and obtain two independent verification results before
promotion to `approved`/`active`. Package status should remain
`EXECUTING / HOLD`; the current fail-closed mixed-surface runtime guard must not
be weakened while the real open-snow producer is absent.
