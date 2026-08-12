# Review Agent B: Repeat Science And Closure Review

Status: `FAIL — two material closure findings remain`

Evidence mode: `Static + Ran`

Reviewed the current exact worktree on 2026-08-11 against
`SC-VEGETATION-001@5`, `SC-BIOGEOCHEM-001@1`, the complete digest-bound model
stack authority artifacts, the historical Review-B and remediation review, the
current finding disposition, and the current production/test bytes. This was a
fresh causal trace through `execute_candidate_with_failure` and
`run_default_off_diagnostic_at_phase`; artifact assertions were not accepted as
implementation evidence.

Ran against the final bytes reviewed:

- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick` — PASS, 12/12.
- `cargo nextest run --test c3_vegetation_implementation_contract --profile quick` — PASS, 10/10, including the updated nonzero-rain public transaction fixture.
- `cargo nextest run -p openwepp-vegetation --profile quick` — PASS, 6/6.
- `cargo nextest run -p openwepp-biogeochemistry --profile quick` — PASS, 3/3.
- `cargo check -p openwepp-hillslope-orchestrator` — PASS.

## Corrections independently confirmed

Static inspection confirms that the former VPD/PAR proxy transaction, fixed
RK4 optics, fixed-`kd` sun/shade shortcut, combined NH4/NO3 debit, producer
residual array, Atkin clamp, non-strict GSI crossings, sequential Brent
rotation bug, residual-only Newton acceptance, untyped material receiver, and
three-owner rollback test have been removed or corrected. In particular:

- The A0 suite is restored separately from implementation tests.
- Typed water and `(layer, NH4|NO3)` identity survives request,
  authorization, finalized use, candidate debit, and owner reconciliation.
- Arbitration is centralized, owner-sorted, and compensated.
- Plant optics no longer creates photosynthetic leaf area from stem area; dry
  stem has a nonzero shortwave owner where applicable.
- The public path invokes radiation, interception, FvCB/Medlyn, explicit
  energy nodes, four-potential hydraulics, water cap re-solve, C/N allocation,
  phenology, turnover, mortality, material proposals, and five-ledger
  validation.
- Condensation is capacity-bounded in the active energy residual, stem path
  vulnerability is applied, gas/hydraulic equality uses the selected
  scale-aware tolerance, and Newton requires both residual and step closure.
- Material donor/receiver/proposal identities are typed and BGC requires exact
  proposal/receipt equality with pre-existing receiver pools.
- The diagnostic now holds vegetation, water, BGC, and energy states and the
  phase-injection test compares serialized bytes for all four owners.

Those corrections close the direct defects in the original
`B-CRITICAL-001/002/003/004/005`, most of `B-HIGH-006/007`, and
`RB-CRITICAL-001/002/003`, `RB-HIGH-005/006`. The following findings prevent a
PASS.

## Material Findings

### RBR-CRITICAL-001 — Multirank E01--E03 loses the upward diffuse boundary between strata

`radiation_by_stratum` solves each stratum independently in top-to-bottom order
(`crates/openwepp-vegetation/src/transaction.rs:807-875`). For every
nonterminal stratum it passes `ground_albedo=0` at `:830-837`, which imposes a
zero upward-flux lower boundary on that stratum. It then passes only
`transmitted_direct` and `transmitted_diffuse` downward at `:854-857`. When a
lower stratum produces an upward reflected flux, that value is never fed back
through the overlying stratum; instead, `EnergyAccumulator::add` sums each
stratum's `reflected` value directly as if every stratum top were the column top
(`transaction.rs:645-653`).

Thus lower-canopy/ground upward radiation bypasses attenuation and scattering
by all overlying strata. This is not the admitted directional two-stream
column boundary problem and violates E01/E03, `INV-VEGETATION-063`, and the
required ground-boundary/upward-downward direction identity. The new
`multi_rank_columns_route_one_rain_and_shortwave_boundary` test reconstructs
`incident = sum(local absorbed) + sum(local reflected) + terminal` from these
same bypassed outputs, so it is a self-consistency check; it does not compare
column-top reflection or per-direction profiles with the independent oracle
and cannot reject this poison.

Required correction: solve or compose the complete multistratum two-stream
boundary system so lower upward flux traverses every overlying stratum before
becoming column-top reflection. Add an independent two-rank vector with
nonzero lower reflection and ground albedo that differs from the rejected
direct-sum/bypass result, plus direct/diffuse and VIS/NIR directional closure.

### RBR-CRITICAL-002 — The energy owner still copies producer operands instead of independently reconstructing energy closure

The new `DiagnosticEnergyState` contains only `last_transaction_id` and
`last_operands` (`crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs:35-40`).
The diagnostic constructs its candidate by cloning
`vegetation_candidate.ledger_operands().energy` at `:222-225`; its owner
validation then checks only that the clone still equals the vegetation-produced
object at `:275-279`. No energy-owned candidate is constructed from independent
radiation, temperature, sensible, latent, stem, wet-surface, and ground/storage
operands. No authority-tagged `h_v` is supplied by the energy owner, and no
energy-side validation proves `Q_T = -h_v * sum(F_W)` or prevents a duplicate
latent debit.

The generic ledger validator is outside the producing equation module, but its
energy inputs still originate wholly in vegetation's `EnergyAccumulator` and
are copied unchanged to the nominal owner. Equality with one's own copy is not
dual reconstruction. Consequently canopy/ground substitution, omitted stem
energy, wrong latent conversion, rate/interval substitution, and a
producer-consistent poisoned energy ledger can pass this owner boundary.

This leaves `B-HIGH-006` and `RB-CRITICAL-004` materially unresolved and
violates SC-VEGETATION step 8, `INV-VEGETATION-066`,
`OBL-VEGETATION-C-002`, and the state-ownership/transaction ledger requirement
that energy independently reconstruct closure before atomic commit.

Required correction: expose immutable authoritative component operands to a
real diagnostic energy owner; have that owner supply/validate the digest-bound
latent heat lineage, independently calculate its candidate debits and five-
ledger energy result, and compare the independently derived receipt to the
vegetation proposal. Add all-distinct poisons for omitted stem energy,
canopy/ground substitution, authorization-versus-finalized water, rate versus
amount, and duplicate latent debit.

## Finding Reassessment

| Finding family | Repeat assessment |
|---|---|
| Original `B-CRITICAL-001`--`005` | Direct mechanisms corrected, but the indivisible public E01--E22 claim remains `FAIL` because RBR-CRITICAL-001 breaks E01--E03 column use. |
| Original `B-HIGH-006` | `FAIL`: typed operands and four-state rollback exist, but RBR-CRITICAL-002 is not independent energy-owner reconstruction. |
| Original `B-HIGH-007` | `PASS` for the inspected finite guards, temperature response, Brent state, physical residual scales, step/residual convergence, and typed numerical failures. |
| Remediation `RB-CRITICAL-001/002/003` | `PASS` for the specific leaf/stem, coupled-solver, and C/N defects previously reported. |
| Remediation `RB-CRITICAL-004` | `PARTIAL / FAIL`: four owner states and serialized rollback are present; energy ownership is still tautological. |
| Remediation `RB-HIGH-005/006` | `PASS` for the specific numerical and typed-arbitration defects previously reported. |

## Review Disposition

`FAIL`. Heavy closure gates, terminal verification, prompt archival, and
`science_implementation_status=IMPLEMENTED` remain premature. Both findings
are implementation defects inside the existing package authority and write
set; neither is an authority contradiction or legitimate HOLD boundary.

## Targeted Authority Adjudication: Heterogeneous Liquid Topology

Status: `CANONICAL OMISSION — legitimate HOLD for the affected mechanism and
the unrestricted full-model completion claim`

This adjudication concerns the current bytes after the two implementation
findings above were corrected. It does not rewrite that earlier review
evidence.

The authority defines every quantity on the horizontal stand/OFE basis unless
it explicitly says otherwise (`SC-VEGETATION-001.md:125-126`). It defines one
projected cover `C_s`, one ground-area `LAI_s`/`WAI_s`, one persistent
ground-area `S_liq,s`, and one ground-area incident amount `P_liq,s` for each
stratum (`:133-140`). The exact topology permits the same stratum to occur in
multiple non-overlapping tiles and defines `C_s` only as the sum of their
fractions (`:214-221`). The complete-state manifest likewise admits exactly
one canopy-liquid state per stratum, not one per `(tile, stratum)`
(`parameter-and-configuration-manifest.md:41-49`).

E04 is a nonlinear, ordered scalar stratum calculation:
`fint=alpha_liq*tanh(L+S)`, capacity is `p_liq*(L+S)`, drainage is a `max`
branch, and wet fraction has a `2/3` power (`SC-VEGETATION-001.md:315-330`;
`equation-authority-ledger.md:17-18`). Unlike E03, which expressly requires
per-tile column traversal and a tile-fraction ground-area sum
(`equation-authority-ledger.md:16`), neither E04 nor the topology rules define:

- a tile-resolved share of the single `S_liq,s`, `LAI_s`, or `WAI_s`;
- an aggregation that maps heterogeneous tile incidents `{P_liq,s,t}` to the
  one accepted stratum store transition; or
- a disaggregation of the stratum release to heterogeneous descendant tile
  columns.

The missing join is constitutive, not merely numerical. In the current
implementation, `rain_by_stratum` first invokes E04 independently in each tile
while replicating the complete stratum store and areas into every invocation
(`transaction.rs:987-1022`), then returns a tile-weighted incident amount for a
later single accepted stratum calculation (`:1024-1031`). Replicating the
stand-ground store in each tile creates an unauthorized state interpretation;
aggregating incident first can define one scalar E04 transition but cannot
uniquely route its nonlinear release among distinct lower tile columns. In
general, a weighted sum of tile-local E04 evaluations is not E04 evaluated at
weighted incident, and a tile-local evaluation cannot be formed without a
rule for distributing the shared state and plant areas.

The in-scope routes were exhausted: aggregate-first lacks downstream routing;
tile-first requires an unstated store/area distribution and recombination;
replicating the scalar state double-counts its availability; and rejecting
otherwise valid heterogeneous topology would add an unauthorized model-domain
restriction. Therefore this is a genuine canonical omission (not contradictory
equations). The affected E04/topology mechanism must fail closed, and the
package cannot truthfully claim the complete unrestricted
`OPENWEPP_C3_WOODY_V1` transaction until authority selects an exact rule.

First lift action: amend the canonical contract and digest-bound model
definition to choose either tile-resolved liquid/area state or an explicit
aggregate-state plus tile-release allocation rule, including conservation and
poison vectors; then implement that selected identity.

The current coverage-corrected radiation basis is sound with respect to this
question. `radiation_by_stratum` converts stand-ground plant area to the
conditional area on every containing tile by dividing by `C_s`, solves the
whole tile column, and weights the result by `f_t`
(`transaction.rs:889-976`). That is the aggregation explicitly required by
E01/E03 (`SC-VEGETATION-001.md:277-314`; equation ledger E03), and radiation
has no analogous persistent shared liquid store. This finding therefore does
not invalidate the corrected radiation topology/basis path.
