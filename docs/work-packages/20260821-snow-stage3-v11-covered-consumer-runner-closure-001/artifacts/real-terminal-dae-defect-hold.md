# Real terminal DAE exploration disposition

Status: `EXECUTED / HOLD / CHILD1-REAL-DAE-001`.

Base: exact clean `64fdeb02942f62efd92428ef538440596b90668f`.
Parent analytical checkpoint:
`bba020889d58bf49567e628c4597d78b955990ef`.
Last fully qualified physical implementation:
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.

Hermite--Gauss disposition remains `ANALYTICAL_PASS_REAL_INCOMPLETE`.

Evidence classification: `Ran` for the commands named below; `Static` for the
source graph, component layout, and stop-condition proof.

## Completed first corrections

Ran: the package Rust receipt tool now gives every DAG node a distinct local
payload and executes nine separate poison reconstructions. Each case proves
unchanged ancestors, a changed target, changed transitive descendants,
unchanged unrelated nodes, exact changed-set equality, deterministic replay,
and no forward reference. Its package-local Rust test passed `1/1`.

Ran: the Rust analytical parity test reads the package-Python matrix and
reconstructs all 48 evaluated smooth floor rows. Installed CN values, signed
Hermite--Gauss estimates, and both defects agree under the declared binary64
parity bound. The focused numerical module passed `4/4`, including exact
599,999,999-ns rejection, exact 600,000,000-ns acceptance, prescribed-amount
and exact outer-support/time-abscissa behavior, nonfinite/cardinality guards, and scaled
residual-plus-step convergence diagnostics.

Ran: the real `BelowCarrierDomain` fixture printed and now asserts the exact
counts:

```text
owners                  = 7
snow lanes              = 1
soil layers             = 6
covered destinations    = 1
LSE component surfaces  = 8
lower boundaries        = 2
precipitation lanes     = 1
```

The numerical scaffold now carries exact outer `TimeSupport` nanoseconds into
every rate/Jacobian callback together with a normalized stage abscissa, rejects
sub-600-ms support, separates prescribed cumulative increments from endpoint
rates, accepts time-dependent callbacks without converting the absolute model
clock to `f64`, threads explicit unknown/residual scales through CN and both
reference methods, validates all vector/matrix cardinalities and finite values,
requires both scaled residual and scaled Newton direction convergence, rejects
nonfinite directions/trials, and retains pivot conditioning and backtracking
diagnostics.

## Source-resolved component census and layout defect

The following is the source-resolved candidate-active storage census, not the
prior 12-row planning inventory and not a complete seven-owner DAE layout.
Ranges are half-open and local to this known candidate-active partition. The
repository's endpoint owners contain additional hydrology aggregate/cumulative
custody fields for which no owner-local dynamic residual or storage map exists;
they cannot truthfully be assigned DAE offsets. That inability is part of
`CHILD1-REAL-DAE-001`, not permission to omit those fields from a claimed
complete layout.

### Dynamic-storage candidates `x`

| Range | Canonical component IDs | Owner | Count | Units | `S` / inverse | Active-set tag | Scale class | Forcing | Generated counterpart | Beginning source -> ending destination |
| --- | --- | --- | ---: | --- | --- | --- | --- | --- | --- | --- |
| `0..1` | `snow.lane-1.ice` | `snow` | 1 | `kg m^-2` | identity / identity | solid-present, sublimation/deposition, melt | mass abs/rel | solid parcel and vapor | liquid melt or vapor transfer | Stage-3 lane aggregate -> Stage-3 lane aggregate |
| `1..2` | `snow.lane-1.liquid` | `snow` | 1 | `kg m^-2` | identity / identity | refreeze/retention/melt | mass abs/rel | liquid parcel | equal/opposite ice conversion or terminal liquid | Stage-3 lane aggregate -> Stage-3 lane aggregate |
| `2..3` | `snow.lane-1.cold_content` | `snow` | 1 | `J m^-2` | `S=-cold_content` / `cold_content=-S` | cold-only versus melt | energy abs/rel | exact advection/radiation amounts | latent conversion and snow--soil heat | Stage-3 lane aggregate -> Stage-3 lane aggregate |
| `3..5` | `vegetation.stratum-z-upper.forest.liquid`, `vegetation.stratum-a-lower.forest.liquid` | `vegetation` | 2 | `kg m^-2 tile` | identity / identity | wet/dry, drainage, evaporation/condensation | water closure | exact cumulative top rain | throughfall, stemflow, drainage, vapor | V8 occupancy liquid store -> accepted V8 occupancy liquid store |
| `5..11` | `hydrology.ofe-1.{thermal-1,thermal-2,soil-1,soil-2,soil-dry,soil-frozen}.water` | `hydrology` | 6 | owner typed layer water storage | owning typed map / inverse not exposed at the carrier boundary | request/authorization/use and routing | owning water closure | exact ingress | root use, infiltration, runoff/runon | `DirectRunFrame` lane layers -> unified hydrology candidate |
| `11..17` | `soil-thermal.ofe-1.{thermal-1,thermal-2,soil-1,soil-2,soil-dry,soil-frozen}.enthalpy` | `soil_thermal` | 6 | `J m^-2 OFE` | `H=H0+C(T-T0)` / `T=T0+(H-H0)/C` | snow-contact topology | energy closure | none | equal/opposite snow--soil heat for `thermal-1` | six-layer snapshot -> six-layer snapshot |
| `17..19` | `surface-liquid.ofe-1.{forest,open}.storage` | `surface_liquid` | 2 | `kg m^-2 tile` | identity / identity | route/capacity/overflow | water closure | exact ingress | vegetation release, infiltration, runoff | persistent surface owner -> persistent surface owner |
| `19..20` | `land-surface-energy.ofe-1.open.surface-enthalpy` | `land_surface_energy` | 1 | `J m^-2 tile` | owning heat-capacity map / inverse through open-surface temperature | open ordinary-surface heat-storage branch | energy closure | prescribed radiation/atmosphere | sensible, vapor and soil conduction | open `TileState.surface_enthalpy` -> ending open tile state |
| `20..21` | `surface-liquid.wb14.ofe-1.cumulative-infiltration` | `surface_liquid` | 1 | `m` | identity / identity | remaining-storage/Green--Ampt branch | water closure | generated interval supply | interval excess/runoff | parent working continuation -> ending continuation |

The open ordinary LSE column can change all six soil-layer enthalpies through
its seven-temperature solve; they are not followers. The covered tile's LSE
surface enthalpy is preserved on the Stage-3-covered branch, while its warm
start and identities remain follower/custody fields. Hydrology canonical owner
custody additionally contains aggregate `soil_water_m` and cumulative
`infiltration_m`, `runoff_m`, `evapotranspiration_m`, `drainage_m`, and
`lateral_flow_m`. The WB14 cumulative-supply field is an exact accumulator of
generated interval supply and therefore belongs in `p`, not prescribed-input
`q`; it is needed for half-2 bounds and chronology. Whether the remaining
hydrology cumulative fields are independent dynamic coordinates, diagnostic
integrals, or derived state cannot be decided from the endpoint transaction
without defining the missing continuous equations. Therefore the known
candidate-active range is `0..21`; no complete `x` cardinality is claimed.

### Algebraic variables `z`

The actual fixture has a 29-row covered residual and a separate seven-row open
ordinary-surface residual, for 36 exposed algebraic rows:

| Range | Canonical IDs | Count | Units / map | Source -> destination |
| --- | --- | ---: | --- | --- |
| `0..10` | upper occupancy `psi_sun, psi_shade, psi_stem, psi_root, beta_sun, beta_shade, T_sun, T_shade, T_wet, T_stem` | 10 | `mm`, dimensionless, `K`; identity | immutable covered-column problem -> accepted upper occupancy |
| `10..20` | lower occupancy, same order | 10 | same | immutable covered-column problem -> accepted lower occupancy |
| `20..22` | covered canopy-air temperature and specific humidity | 2 | `K`, `kg kg^-1`; identity | atmospheric/lower-boundary closure -> accepted canopy-air state |
| `22..23` | V11 ground/lower temperature | 1 | `K`; exact identity to Stage-3 snow temperature | Stage-3 lower boundary -> LSE diagnostic follower |
| `23..29` | six LSE soil temperatures in canonical layer order | 6 | `K`; exact identity to beginning temperatures | beginning soil snapshot -> LSE diagnostic followers |
| `29..36` | open tile surface temperature followed by six soil temperatures | 7 | `K`; surface enthalpy inverse plus six soil enthalpy inverses | open LSE/soil beginnings -> open LSE surface and six aggregated soil candidates |

The covered nonlinear vector SCC is `0..22`; covered rows `22..29` are exact
boundary/follower identities. The open solver has its own seven-variable
nonlinear block at `29..36`. Four additional leaf intercellular CO2 roots
(sun/shade for each occupancy) are currently eliminated by private
branch-heavy scalar root solves. They have no current vector offsets. Exact AD
therefore requires either implicit derivatives of those roots or promotion to
four new algebraic coordinates; neither exists in the actual residual.

All algebraic rows use the frozen real branches: root/wet water branch, gas
root, zero-area substitution, wet/dry/drainage posture, and Stage-3 covered
topology. Absolute/relative scales are the owning LSE hydraulic, beta,
temperature, humidity, water, and energy closure scales. No component
temperature has a heat-capacity storage map; the earlier inventory's
`vegetation.temperature` dynamic classification was false.

### Prescribed cumulative inputs `q` and exact followers `p`

`q` is the one lane's sealed precipitation mass/advection and provider
radiation amounts plus tick-evaluable atmospheric boundary series. The real
fixture has zero terminal-liquid/external-liquid ingress. Exact support amounts
are never divided by duration or re-quadratured.

`p` contains the active-set/topology tags; the WB14 cumulative-supply and
receipt-chain continuation; V11 covered LSE ground/soil identities;
vegetation persistent/T10/carbon/material
transition; exact BGC follower; open-snow tile; canonical seven-owner bytes;
generated transfer receipts; and all receipt/digest chronology. The complete
vegetation follower must be projected between half 1 and half 2 because its
ending becomes the next beginning even though it does not feed the same
support's algebraic solve.

## Actual graph and SCC correction

Static source inspection and the retained real regression prove that one live
provider call is feed-forward:

```text
immutable six-owner beginning + trial-start snow + prescribed forcing
  -> projection and radiation
  -> covered LSE/vegetation potential solve
  -> hydrology authorization
  -> covered fixed-cap LSE/vegetation solve
  -> hydrology/surface endpoint
  -> vegetation persistent projection
  -> BGC follower
  -> corrected carrier boundary + separate snow/top-soil CN credit
  -> Stage-3 snow endpoint
```

Every node in that coarse orchestration-call graph is a singleton SCC after
each owner-local nonlinear solve is collapsed to one call node. Internally,
the covered 22-variable and open seven-variable LSE blocks above remain
nonlinear SCCs. The supposed outer carrier feedback is absent:
`ending_snow_hint` is not read by the carrier, and
the existing regression proves coupling iterations 0 and 1 have identical
physical transitions, ending joints, precipitation, lower boundaries, carrier
sources, LSE states, soil candidates, and WB14 evidence.

A future continuous DAE would introduce two real feedback SCCs—snow/LSE carrier
and root-demand/hydrology authorization—with generated canopy/surface transfers
between them. That is a proposed model graph, not the graph executed by the
current physical implementation. The 11-node SCC in
`terminal-scc-inventory-v3` is therefore planning evidence only and is
superseded for source-truth claims by this artifact.

## Defect-shaped stop condition

`CHILD1-REAL-DAE-001`: the repository has no complete real
`dS(x)/dt=f(t,x,z,q), g(t,x,z,q)=0` residual from which the requested CN,
collocation, and AD Jacobians can be constructed without adding physical model
equations.

The missing pieces are exact and source-local:

1. Terminal Stage 3 consumes a support-integrated `TerminalFluxIntegral` and a
   piecewise endpoint `terminal_transition`; it exposes no fixed-active-set
   instantaneous snow rate.
2. E04 canopy interception/drainage and vapor authorization embed the whole
   `interval_s` and preliminary cumulative store in the algebraic transaction;
   no owner-local continuous storage residual exists at an arbitrary Gauss
   tick.
3. Hydrology authorization and WB14/surface routing are full-support endpoint
   transactions. Their six storage maps/rates and exact derivatives are not
   exposed to the carrier.
4. The four leaf `ci` equations are hidden scalar root algorithms rather than
   coordinates or differentiable residuals.
5. Current projected forcing overlap uses binary64 duration ratios; there is no
   complete exact cumulative forcing evaluator for arbitrary collocation ticks.

Refactoring existing scalar formulas is authorized. Defining the missing
continuous snow, canopy-storage, hydrology, authorization, and routing
equations is not a visibility or scalar-generic refactor; it selects a new
physical temporal model. That crosses the immutable prohibition on changed
physical equations. It also cannot be justified by relabeling the draft
v20/v21 inventory: that inventory misclassifies temperatures and cardinalities
and asserts a feedback edge absent from the live consumer.

Consequently AD cannot yet be compared to the f64 complete residual; Gauss and
Radau cannot solve the real DAE; no real CN/reference receipts, Hermite
effectivity matrix, enclosure, or successor contracts can truthfully be
produced. This is the user-named stop condition "expressing the real residual
requires a changed physical equation," not an ordinary compiler, visibility,
DTO, scaling, or convergence problem.

## Noninterference and next authority need

Production remains `BelowCarrierDomain`; current receipts are not candidate
receipts. No temporal operator, Batch V2, public API, dependency, production
tolerance/controller, chronology, owner mutation/publication, restart,
receiver, runner, selector/default, Stage-3 activation, CoE, Child 3, or Child
4 change is present. No v22/v12/v140/v7 candidate is authored.

The first lift step is to close `CHILD1-REAL-DAE-001` with canonical authority
for the exact owner-local continuous residuals and corrected scalar layout,
including leaf `ci`, canopy cumulative forcing/storage, root authorization,
hydrology/WB14 rates, and their fixed-active-set derivative semantics. Only
then can the authorized test-only AD/CN/Gauss/Radau/effectivity implementation
continue without inventing physics.
