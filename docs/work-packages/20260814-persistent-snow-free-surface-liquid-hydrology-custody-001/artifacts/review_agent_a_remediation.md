# Review Agent A — Rust Remediation Re-review

Evidence class: `Static exact-commit + Ran exact-commit`  
Reviewed commit: `a4138bee2ae2caefab20fbb9474612ed346b759e`  
Verdict: `HOLD / material Rust remediation required`.

The live worktree changed after this review began. All source findings and the
reported passing tests below come from a separate `git archive a4138bee2`
export. In-progress remediation bytes are intentionally excluded.

## Findings

### High — Post-authorization mutation can replace the required proportional authorization

`DirectSurfaceLiquidArbitration` exposes `beginning_state`, `requests`, and
`authorizations` as public mutable fields in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs:1073`.
`apply_surface_liquid_resource_phase_inner` validates the possibly modified
beginning state and then accepts those vectors at lines 1316-1328. The D/A/F
check at lines 1503-1545 tests only `F <=` the currently stored authorization
and that authorization `<=` the currently stored request; it neither
reconstructs `A` from the immutable beginning snapshot nor checks the stored
authorization identity. The sealed resource candidate then retains finalized
uses and credits, but not the original D/A operands, so its independent
reconstruction at lines 1437-1487 cannot recover this guarantee.

This admits a silent science-contract divergence. For two same-store demands
of `1` with beginning supply `1`, the authorized batch is `0.5/0.5`. A caller
can change the public authorization amounts to `1/0`, submit finalized uses
`1/0`, and pass the present `F <= A <= D` and state-closure checks. That is not
the proportional same-snapshot equation required by
`SC-SURFACELIQUID-001:277-301` and `INV-SURFACELIQUID-003/004`.

Required correction: make the complete arbitration envelope immutable outside
its owner module and/or cryptographically or structurally bind and independently
recompute request identity, store mapping, beginning supply, and every
authorization before applying finalized use. Preserve the original D/A
operands through candidate validation.

### High — The unified callback can supply an incomplete or forged actual receiver envelope

The caller-supplied `UnifiedLseFinalization` fields and the returned unified
candidate vectors are public at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:127-157`.
`validate_receiver_sets` at lines 478-510 checks only the set of `(OFE,tile)`
pairs plus `before == after` rollback hashes. It does not validate thermal
owner identity, beginning digest, ordered layer identity/cardinality, finite
layer values, or an exact rollback owner/cardinality set. The rollback join at
lines 513-549 uses `contains`/`any`, so duplicate and extra rows are admitted and
a forged thermal owner/digest can be made to pass by appending a matching
rollback row. Infiltration then finds and credits only the named top layer at
lines 605-646. Production soil infiltration at lines 681-704 applies the shared
transition but does not independently reconstruct its ordered per-layer mass
deltas.

Consequently, a callback can return a thermal candidate containing only the
top layer, arbitrary unchecked layer values, or a substituted owner/digest with
a matching extra rollback record, and the unified transaction can return it as
an actual receiver candidate. This violates the exact ordered production-layer
binding and exact receiving-layer requirements at
`SC-SURFACELIQUID-001:132-138`, the independent ordered layer-delta requirement
at lines 337-356, and the complete owner join/atomic envelope requirements at
lines 442-459.

Required correction: freeze primitive beginning/ending receiver operands,
validate exact owner/tile/layer order and cardinality, reconstruct each
production soil-layer delta and thermal/LSE enthalpy credit independently, and
require one exact rollback record set with no omissions, duplicates, or extras.

### High — Canonical surface-liquid failure taxonomy remains unreachable or misclassified

The low-level mapping in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs:162-170`
can produce only E001, E002, E003, E005, E006, and E010. At this commit, the
only production-side explicit special code is E009; E004, E007, E008, and E011
otherwise occur only in the enum/string coverage test. Concrete public-path
misclassifications include:

- interval/cadence mismatch returns `Identity`/E002 at
  `surface_liquid_ingress.rs:304-312`, and continuation index/carry mismatches
  also return E002 at lines 442-477, although the contract requires E008;
- a WB14 continuation failure returns `Domain`/E003 at lines 745-754; the test
  at lines 1669-1679 asserts E003, although producer/carry failure requires E008;
- receiver/rollback complete-owner failures in
  `land_surface_energy_shadow/mod.rs:478-550` return generic LSE `Identity`
  errors rather than the required surface-liquid E010/E011 envelope payload;
- capacity, attribution, routing, and parcel-enthalpy closure commonly enters
  the generic `Closure` mapping E010 even though the branch table assigns these
  candidate failures E009.

The public ingress wrapper at `surface_liquid_ingress.rs:267-296` adds only the
transaction ID, sets the attempted owner hash to typed absence for every
failure, and drops available OFE/tile/surface/source/parcel identity. This does
not meet the exact code, precedence, phase, context, and beginning/attempted
hash payload required by `SC-SURFACELIQUID-001:461-481`.

Required correction: construct the canonical code at each guard, preserve it
through wrapping, populate every identity available at the failure site, and
add executed reachability tests for E001 through E011 rather than enumerating
their strings.

### Medium — Public state serialization can emit invalid or stale bytes as canonical

`DirectSurfaceLiquidOwnedState` exposes all state and digest fields publicly at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs:713-720`.
Unlike configuration serialization, which calls `validate()` at lines 553-555,
state `canonical_bytes()` at lines 962-964 directly serializes the current
fields. The private digest recomputation at lines 1029-1033 likewise serializes
without first validating against configuration. A caller can therefore mutate
mass, lineage, order, continuation, or `state_sha256` and obtain bytes labeled
canonical even though the strict parser later rejects them.

That is asymmetric with the strict parse/serialize/digest contract at
`SC-SURFACELIQUID-001:193-222`. Required correction: prevent external state
mutation and make canonical serialization validate the complete configuration-
bound state and self digest, with a distinct internal zero-digest encoder used
only during validated digest construction.

### Medium — Ingress and unified candidates remain mutable, duplicated representations without a sealed validator

Although `DirectSurfaceLiquidResourceCandidate` is now private-field sealed,
`DirectSurfaceLiquidIngressCandidate` still exposes transaction, beginning and
ending states, receipts, ledgers, and call counts publicly at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:218-234`.
`UnifiedRealHydrologyCandidate` exposes every nested candidate and owner vector
at `land_surface_energy_shadow/mod.rs:145-158`. It also duplicates the same
ending surface state in `ending_frame.surface_liquid_shadow` and
`surface_ingress.ending_state` when constructed at lines 420-445, without a
public sealed validation boundary that proves the two copies and all receiver
operands still agree.

There is no production selector/commit consumer in this default-off package,
so this is not an observed production mutation. It is nevertheless a material
runtime/serialization seam for the package's promised uncommitted candidate:
ordinary downstream Rust can mutate one copy, ledger, receipt, or call count
while retaining the other. Required correction: use private candidate fields
with read-only accessors and a complete validator, or carry one canonical state
representation and independently frozen operands into any future atomic
consumer.

## Confirmed corrections at `a4138bee2`

Static review confirms that the remediation did make these material advances:

- the resource candidate itself now has private fields, immutable accessors,
  and an independent state/credit/overflow reconstruction;
- strict initial/accepted continuation combinations and canonical state parsing
  are implemented;
- the unified beginning identity now binds production lane/run/area/layer data
  and SHA-256-based canonical soil/surface snapshot material rather than the
  prior legacy 64-bit token;
- actual LSE, production soil, soil-thermal, retained-energy, and rollback
  candidates are carried and applied to clones instead of being discarded;
- daily WB14 and the 1800-second continuation delegate to one shared transition,
  with threshold/capacity/roundoff and 48-step parity vectors;
- no production selector/default activation was introduced; and
- no touched Rust file reaches the mandatory 3000-line split threshold. The
  touched files above 2000 lines have recorded WARN dispositions.

These corrections do not close the findings above because the remaining public
mutation and incomplete validation paths can bypass their intended invariants.

## Residual risk and missing tests

Missing comparator-sensitive or poison evidence includes:

- mutate a completed arbitration's D/A rows and prove application rejects it;
- missing/extra/reordered production and thermal layers, wrong thermal owner or
  beginning digest, duplicate/extra rollback hashes, and nonfinite actual
  receiver candidates;
- independent reconstruction of every applied production soil-layer mass delta
  and every thermal/LSE enthalpy credit in a multi-tile, multi-OFE unified run;
- executed exact code/phase/context/hash assertions for every E001-E011 branch,
  including E004, E007, E008, E009, E010, and E011 poisons;
- mutation of each public state field followed by canonical serialization; and
- contradictory nested/duplicated ingress and unified candidate state.

Ran against the isolated exact-commit export:

- `cargo nextest run --profile quick --test surface_liquid_hydrology_custody_authority_contract` — 9 passed;
- `cargo nextest run --profile quick -p openwepp-hillslope-orchestrator -E 'test(/surface_liquid/)'` — 25 passed.

No full-workspace or campaign comparator was run for this bounded re-review.
The passing focused suites do not contain the poisons above and therefore do
not clear the HOLD.

## Approval statement

`NO-GO`: exact commit `a4138bee2` is not acceptable for work-package closure.
The five findings above require remediation and fresh independent exact-byte
Rust review.
