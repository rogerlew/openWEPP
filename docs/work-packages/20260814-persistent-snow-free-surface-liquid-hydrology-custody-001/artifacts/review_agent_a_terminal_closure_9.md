# Review Agent A — Terminal Closure 9 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `fd8633865df289620aa5b9cf8c4e1bd206432f30`

Verdict: `HOLD / NO-GO`.

The review used a source archive generated from the exact reviewed Git object.
Concurrent work later advanced and modified the shared checkout; none of those
later bytes is assessed or used as evidence here.

## Findings

### High — The new ending-state validator fabricates the first OFE for aggregate and structural failures

`validate_projected_ending_state()` correctly joins the independently replayed
ending values, but several new error branches do not preserve the exact
offender context required by `SC-SURFACELIQUID-001`.

At
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:2238-2252`,
owner mismatch, configuration mismatch, store cardinality mismatch and
continuation cardinality mismatch are collapsed into one E010 and assigned the
first configured OFE. A missing or extra non-first store/continuation therefore
reports an unrelated OFE. Same-length reordered or replacement continuation
rows reach `:2276-2295`, but the error context uses the expected positional OFE
rather than retaining the actual offending row for replacement/reorder cases.

Digest reconstruction, digest mismatch and complete-state validation likewise
unconditionally attach `configuration.ofe_topology[0]` at
`surface_liquid_closure.rs:2297-2322`. A whole-owner digest failure has no
unique OFE; the contract requires typed absence when an identity is not
applicable or cannot be proven, not a fabricated first member.

The endpoint poison test covers wrong store/cumulative/cadence/lineage,
missing/duplicate/reordered/wrong-OFE continuation and digest cases at
`surface_liquid_ingress_tests.rs:1878-1954`, but it asserts only E010. It does
not assert the offending owner/OFE/store context or rollback hashes. This lets
the exact diagnostic-contract regression pass all focused tests.

The numerical rejection is fail-closed, so this finding does not admit a wrong
ending mass or continuation. It is nevertheless high severity because the
canonical typed payload is part of the public failure contract and exact
identity has repeatedly been required for rollback and defect localization.

Required correction:

1. Separate owner/configuration/digest-wide failures from row failures and use
   typed `None` for OFE/tile/surface/source when no unique row is known.
2. Apply the existing membership-aware sequence policy to ending records and
   continuations: identify the expected missing row for deletion, the actual
   extra/replacement/reordered row when present, and the exact configured row
   for a value mismatch.
3. Add first/middle/last deletion, addition, duplicate, reorder, replacement,
   owner/configuration and digest poisons that assert exact code, phase,
   context and beginning/attempted rollback hashes through the public closure
   boundary.

### Medium — Canonical parcel order and source-ID construction remain duplicated, while the new routed vector has no independent frozen numerical oracle

The shared named water density, heat capacity and reference-temperature
constants close the literal-transcription part of the closure-8 finding. The
remaining canonical definitions are still mirrored:

- Production orders timed parcels by start, end, origin store, kind and parcel
  ID in `surface_liquid_ingress.rs:1909-1915`.
- The independent projector repeats that comparator for infiltration
  attribution at `surface_liquid_closure.rs:1332-1340` and for retention/final
  remainder ownership at `:1505-1514`.
- Production formats local and condensation source IDs at
  `surface_liquid_ingress.rs:1030-1033` and `:1145-1148`. Closure capture
  repeats those formats at `surface_liquid_closure.rs:979-982` and
  `:1013-1016`, then frozen-identity validation repeats them again at
  `:1067-1070` and `:1081-1084`.

The high-level production and expected allocation algorithms must remain
separate to preserve anti-tautology. The canonical ordering key and source-ID
syntax are identity definitions, however, not independent physical
calculations. Their substantial duplication can silently change final-
remainder source ownership and the per-source mass/enthalpy join.

The new mixed-kind vector is nondegenerate: it uses unequal source
temperatures, unequal OFE areas, multiple canopy kinds and downstream-local
overlap (`surface_liquid_ingress_tests.rs:345-418`). It proves canonical
`UpstreamRunon` conversion and caller-input-order invariance. It does not freeze
any independently calculated window `h_mix`, per-source infiltration/
retention/runoff mass and enthalpy, final remainder owner, ending store or
continuation value. Because candidate construction and closure validation use
the mirrored order/ID definitions, a coordinated drift can keep the test green.

Required correction: centralize a typed canonical parcel-order key and the
local/condensation source-ID constructors while retaining separate physical
allocation implementations. Add an independent frozen numerical oracle for
the mixed-kind vector that fixes every window mixture, attributed child,
remainder owner, unequal-area descendant and final owner/continuation output.
Record the anti-tautology justification for the remaining production/projector
algorithm duplication.

### Medium — The 2,324-line ingress test module is incorrectly dispositioned as PASS

`surface_liquid_ingress_tests.rs` is 2,324 lines at the exact reviewed commit.
The package's `artifacts/line-count-governance.md` labels it `PASS` and supplies
neither decomposition rationale nor follow-on split intent. `crates/AGENTS.md`
requires every `.rs` file at or above 2,000 lines to be `WARN` with both.

No affected file reaches the mandatory 3,000-line refactor threshold. This is
a bounded governance defect: change the row to WARN, explain why the focused
test surface remains cohesive for this increment, and name a follow-on split
such as endpoint/precedence tests versus chronological-routing tests. The
production closure module's 2,532-line WARN already has adequate rationale and
future split intent.

## Closure-8 Findings Re-Audited

The substantive endpoint and precedence corrections are present and
numerically sound:

- `ParcelArithmeticProjection` now retains the receipt-free
  `expected_store_liquid` map and one `DirectProjectedContinuation` per OFE
  (`surface_liquid_closure.rs:570-587`).
- Independent chronological replay initializes from frozen beginning
  continuation, advances the shared WB14 transition and retains final
  cumulative supply/infiltration at `surface_liquid_closure.rs:1298-1395` and
  `:1743-1752`. The final tile stores are retained at `:1727-1740` and returned
  at `:1790-1801`.
- `validate_projected_ending_state()` bitwise joins every ordered store and
  continuation, accepted transaction lineage and cadence before recomputing
  the complete digest and invoking strict state validation
  (`surface_liquid_closure.rs:2254-2323`). The finding above concerns offender
  attribution, not the value comparisons.
- A coordinated forged retained operand plus ending store fails independent
  E010 because the expected W1 comes from the receipt-free store map, not the
  producer-captured retained value
  (`surface_liquid_ingress_tests.rs:1925-1942`).
- Partition arithmetic preflight now checks finiteness/nonnegativity,
  `cumulative infiltration <= cumulative supply`, `cumulative infiltration <=
  storage capacity`, and valid ending interval before any zero-supply shortcut
  (`surface_liquid_closure.rs:1915-1940`).
- Missing, duplicate, reordered and replacement partition inputs use
  membership-aware producer E009. E003 arithmetic/domain preflight runs first,
  and immutable E009 reconstruction runs before independent E010
  (`surface_liquid_ingress.rs:337-406`,
  `surface_liquid_closure.rs:2046-2095`). The combined zero-supply/capacity and
  E003/E009/E010 poisons retain exact rollback hashes
  (`surface_liquid_ingress_tests.rs:1959-2073`).
- Closure uses the production module's named `WATER_DENSITY_KG_M3`,
  `LIQUID_HEAT_CAPACITY_J_KG_K` and `REFERENCE_TEMPERATURE_K` constants while
  retaining its own expected-side arithmetic. No raw dimensional literal was
  added to the projector.

## Full Endpoint And Historical Finding Re-Audit

- Expected partition construction still has zero receipt access. Receipts feed
  only the actual comparison map at `surface_liquid_closure.rs:1194-1213`;
  expected WB14 partition, retention, routing, store and continuation values
  derive from frozen raw sources/state/configuration thereafter.
- Exact parcel identity retains owner, source parcel, origin store,
  current/recipient store, complete typed recipient, basis OFE, routed kind,
  support bits and disposition. Independently routed descendants become
  `UpstreamRunon` and apply the destination area ratio once.
- Frozen raw Q remains bit-joined to mass times specific enthalpy before
  mixing. The chronological expected side retains canonical
  `h_mix,b = supply_enthalpy / supply_mass`; the remediation does not modify
  production mixture, attribution, clamp, guard or unit-conversion arithmetic.
- Exact D/A/F custody, finalized-use-only debit, signed condensation,
  pre-ingress capacity, one stateful WB14 call per OFE, soil-liquid/
  soil-thermal/retained-LSE receivers, restart lineage, snow/frost rejection
  and default-off production exclusion remain intact.
- Candidate construction and validation remain clone-only. Reviewed failure
  vectors preserve byte-identical resource state and carry beginning and
  attempted hashes. The high finding is contextual attribution, not partial
  mutation or missing rollback data.
- All historical failed reviews and finding dispositions remain preserved.

No new numerical, constitutive, serialization, receiver, rollback or
production-selection defect was found in the exact bytes beyond the findings
above.

## Line-Count Inventory

| File | Lines | Review disposition |
|---|---:|---|
| `direct_runtime/runoff.rs` | 2,852 | WARN, below mandatory stop. |
| `direct_runtime/00_core_frames.rs` | 2,783 | WARN, below mandatory stop. |
| `direct_runtime/surface_liquid_owner.rs` | 2,347 | WARN, below mandatory stop. |
| `direct_runtime/surface_liquid_owner_tests.rs` | 876 | PASS. |
| `direct_runtime/surface_liquid_ingress.rs` | 1,953 | PASS. |
| `direct_runtime/surface_liquid_ingress_tests.rs` | 2,324 | WARN; package disposition is incomplete. |
| `land_surface_energy_shadow/mod.rs` | 2,881 | WARN, below mandatory stop. |
| `direct_runtime/surface_liquid_closure.rs` | 2,532 | WARN with split intent. |
| `direct_runtime/surface_liquid_wb14.rs` | 303 | PASS. |
| `vegetation_real_hydrology_shadow.rs` | 2,157 | WARN, below mandatory stop. |

## Exact-Commit Validation

Ran from a source archive generated from
`fd8633865df289620aa5b9cf8c4e1bd206432f30`:

```text
CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  -p openwepp-hillslope-orchestrator --profile quick
PASS: 562/562; 3 slow retained routing-oracle tests; 0 skipped

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo clippy \
  -p openwepp-hillslope-orchestrator --all-targets --all-features -- \
  -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check \
  298acedbb47455d5ce54ec0bac2b7382955b11ee...\
  fd8633865df289620aa5b9cf8c4e1bd206432f30
PASS
```

These commands prove the focused contract/consumer surface, complete owning
crate test set and strict source quality of the exact bytes. They do not prove
the missing exact-context assertions or provide an independent frozen routed
numerical oracle. Full-workspace, doctest, dependency-policy and release gates
were not rerun by this reviewer; the package's separately required terminal
verification remains pending.

## Residual Risk And Missing Tests

The independently replayed W1, WB14 continuation and digest are now protected
against coordinated producer drift. Remaining risk is localized to canonical
failure attribution and to coordinated ordering/source-ID drift that the
current self-consistent mixed-kind vector cannot detect. The ingress test-file
WARN also lacks its required maintenance disposition.

Missing evidence is the exact-context endpoint poison matrix and the frozen
multi-kind numerical oracle described above. No broader science-authority or
new-package dependency is indicated; both corrections fit the existing
custody package.

## Approval Statement

`NO-GO`: exact commit `fd8633865df289620aa5b9cf8c4e1bd206432f30`
closes the closure-8 persistent endpoint and partition-precedence defects, but
the new ending-state branches violate canonical offender-context rules, the
duplicated order/identity seam lacks an independent frozen numerical oracle,
and the 2,324-line test module lacks required WARN disposition. Correct these
bounded findings and rerun the focused endpoint, rollback, routing and strict
source-quality gates before terminal approval.
