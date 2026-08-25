# Terminal batch residual and state inventory V1

Status: `Static / candidate-authority input`

Source identity: `68897d9488d85430cbf2b11cf1a9839670a3c044`

This inventory defines numerical surfaces, not derivatives of canonical owner
bytes. The seven owner payloads remain opaque exact custody objects. Numerical
residuals operate only on the typed fields below; accepted typed endings are
serialized by their owning contracts and compared to the installed owner bytes
by exact receipt joins.

## Residual partition

For support `H=[t0,t1)`, `h=t1-t0`, candidate `x1`, and beginning `x0`:

```text
R = S(x1) - S(x0)
    - I_prescribed(H)
    - Q_endpoint(H,x0,x1)
    - P_deterministic(H,x0,x1)
```

`A(H,x0,x1)=0` separately denotes algebraic carrier closure;
`C(H,x0,x1)=0` denotes phase/complementarity; `D(...)` denotes exact custody,
ordering, topology, cardinality, receipt, and owner-byte predicates. `D` is
never differentiated or included in a floating-point norm.

Low BE uses `Q_endpoint=h F(t1,x1)`. High CN uses
`Q_endpoint=h/2(F(t0,x0)+F(t1,x1))`. Both use the identical sealed
`I_prescribed(H)` and deterministic projection rules. An interval total is not
divided by `h`, converted to an endpoint rate, or quadratured again.

## Participating typed surfaces

| Owner / typed field | Units | Class | Storage map `S` or projection | Forcing and shortened-support rule | BE / CN semantics | Phase/complementarity | Tolerance / exact predicate | Receipt and installed-owner join |
|---|---|---|---|---|---|---|---|---|
| snow lane ice mass, ordered layer ice/SWE | `kg m^-2` OFE-ground; layer SWE `m` | differential storage | identity mass after exact SWE conversion; ordered layer partition is discrete | sealed solid parcels are exact-support totals; vapor is endpoint flux; no parcel re-quadrature | BE/CN integrate only bounded endpoint vapor and state-dependent melt opportunity; prescribed solid mass enters identically | nonnegative ice; sublimation reserved before melt; melt bounded by available ice; terminal iff accepted high candidate has zero solid within root tolerance | LTE `a=1e-9 kg m^-2`, relative `1e-8`; closure scale-aware `max(1e-12,1e-12 sum_abs)`; layer order/count exact | lane physical ledger, bounded-vapor and precipitation receipts; high ending serialized into canonical snow owner and exact-joined |
| snow retained liquid and terminal/released liquid | `kg m^-2` OFE-ground | differential storage plus discrete destination | identity retained-liquid storage; terminal parcel is discrete custody, not storage derivative | liquid precipitation/release parcels are exact-support totals with route/enthalpy identity | external totals identical in both arms; endpoint-dependent refreeze/retention uses each arm's state | retained/refrozen/routed split is exclusive; terminal parcel exists only after high accepted terminal endpoint | LTE `a=1e-9 kg m^-2`; exact parcel cardinality/posture/destination | precipitation set, phase/liquid receipt, outcome ledger, V4 pending-parcel map; installed snow owner exact |
| snow cold content/material enthalpy | `J m^-2` | differential storage | `S_E=-Q_cc + L_f m_liquid` under existing positive-into-snow convention; layer enthalpies sum exactly | precipitation advection and prescribed radiation totals enter once; state-dependent longwave, sensible, latent, snow--soil and interlayer fluxes are endpoint terms | BE endpoint flux versus CN beginning/ending average; each component retained separately; no `Q_complete` residual substitution | cold-content-first; melt only after cold deficit exhausted; refreeze uses `L_f m_refrozen`; unallocated energy only at genuine exhaustion | LTE `a=1e-6 J m^-2`, relative `1e-8`; independent energy closure `max(1e-6,1e-12 sum_abs)` | ordered component receipts, snow--soil receipt, phase receipt and independent physical ledger join high snow bytes |
| snow surface temperature, depth, density, layer cardinality/order | `K`, `m`, `kg m^-3`, discrete | algebraic/diagnostic plus discrete lifecycle | reconstructed deterministically from accepted mass, cold content, layer mechanics and authoritative thermal partition; not separately integrated | shortened support changes the physical state feeding reconstruction, not its laws | each arm reconstructs its own endpoint; values enter state-dependent endpoint fluxes | resolved/terminal active-set and exact layer-collapse rules; no blended layers | `1e-8 K`, `1e-9 m`; density bits and structural/count fields exact under existing fixed-point policy | candidate fingerprint, layer/thermal receipts and exact canonical snow-owner encoding |
| vegetation interception stores per occupancy | `kg m^-2 tile-ground` | differential storage owned by vegetation | exact `store_end-store_begin` from authoritative V8 balance | precipitation/condensation inputs and released throughfall/drainage/stemflow are exact-support totals | each arm executes the authoritative support transition with its own algebraic endpoint carrier; no derivative of vegetation bytes | nonnegative storage and exact route exclusivity | existing vegetation mass tolerance; occupancy/order/configuration exact | V11 water/energy receipts; ending vegetation owner bytes exact-joined once |
| vegetation component temperatures and shared canopy-air node | `K`, `kg kg^-1` | algebraic carrier state | no storage derivative in this successor; solve existing component energy and shared-air residuals | sealed atmosphere/exposure and support-specific radiation projection | BE solves algebraic closure at ending state; CN uses beginning and ending state-dependent physical fluxes but ending algebraic closure remains exact | wet/dry surface and condensation/evaporation branches use existing active sets | existing fixed-point unit bounds; topology/component identity exact | component-carrier and reciprocal-longwave receipts; ending vegetation/LSE owners joined once |
| land-surface-energy tile temperatures/flux terms | `K`, `W m^-2` and exact `J m^-2` receipts | algebraic carrier plus exact-support integral | existing LSE-V8 physical solve; no serialization-byte derivative | prescribed interval radiation remains sealed `J m^-2`; endpoint-dependent longwave/sensible/latent/soil fluxes expose typed `W m^-2` endpoints | both arms solve same LSE equations on same support; BE/CN differ only in endpoint-flux quadrature named by the owning residual | snow-present terminal active set; positive support must be `>=600000000 ns` | existing LSE Newton/fixed-point guards; physical fields unit-specific; identities exact | ordered tile/component boundary receipts and ending LSE owner exact-joined once |
| soil-thermal top node/layer enthalpy participating in snow boundary | `J m^-2` OFE-ground; temperature `K` | differential storage | authoritative soil heat-capacity/enthalpy map; temperature is its invertible projection inside domain | deeper/bottom prescribed terms retain owning representation; snow-top heat is state-dependent endpoint flux | BE/CN use equal-and-opposite snow--soil endpoint flux on same support and independently close soil enthalpy | no phase normalization beyond owning frost/soil contract; invalid inversion typed | `1e-6 J m^-2` for energy residual; existing soil temperature/frost guards; node identity exact | terminal snow--soil trial/accepted receipt binds both endings; ending soil owner exact-joined once |
| surface-liquid per-OFE storage and queued parcels | `kg m^-2`, `J m^-2` | discrete/exact-support transition | existing WB14/surface ingress deterministic transition; not an inferred rate | exact timed supply parcels and enthalpy over `H`; shorten by provider parcel-support intersection only | same sealed parcel set enters both arms; candidate-specific hydrology may differ through endpoint state but no parcel re-quadrature | storage/infiltration/overflow routes exclusive and nonnegative | existing WB14 closure; parcel/config/order exact | topology-ordered ingress receipts and ending surface-liquid owner exact-joined once |
| hydrology soil water, infiltration/runoff/runon cumulative state | owning depth/mass units | deterministic support transition with stored scalars | existing hydrology kernel transition from beginning plus exact boundary supplies; canonical bytes not differentiated | timed supply/support intersection is authoritative | run once per complete BE arm and once per complete CN arm from the same beginning; all lanes participate; accepted high arm supplies every ending | existing Green--Ampt/frozen/domain branches; no cross-lane post-hoc merge | owning kernel tolerances; lane/OFE/config/model identity exact | per-lane hydrology endings, ordered WB14 receipts and one ending hydrology owner join |
| biogeochemistry mineral pools and vegetation debits | `kg N` or configured area basis | deterministic support transition / exact debit custody | existing BGC pool transition; no new temporal derivative is defined here | exact accepted vegetation use/debit set for the candidate support | each complete arm produces and validates its own debit set; shared BGC owner advances once, never per lane | configured exact-one-bearing-OFE and stratum/layer/species rules | existing pool/debit closure; identities/order exact | scoped debit/transition receipts and ending BGC owner exact-joined once |
| canonical owner bytes, digests, topology, ordinals, receipt chains | bytes/digests/integers | discrete exact predicate | none | none | no BE/CN arithmetic | exact chronology and posture only | bitwise equality, canonical ordering/cardinality, SHA-256 reconstruction | complete owner-set, joint-trial, batch, group, event, parcel and parent joins |

## Prescribed totals versus endpoint fluxes

| Exact-support prescribed integral `I_prescribed(H)` | State-dependent endpoint flux `F(t,x)` | Algebraic/deterministic/discrete |
|---|---|---|
| sealed solid/liquid precipitation parcel mass; precipitation-advected heat; provider-integrated radiation when the provider contract supplies energy; timed surface-liquid supply parcels | snow/canopy longwave exchange; sensible and vapor exchange; latent energy from the exact bounded vapor transfer; snow--soil conduction; interlayer conduction; state-dependent emitted radiation | shared canopy-air closure; LSE/vegetation component closure; layer lifecycle; phase complementarity; hydrology/BGC deterministic transitions; owner/receipt/hash predicates |

If an upstream receipt supplies only a support total, it remains in the first
column. A successor may use endpoint quadrature only when the owning contract
and receipt expose authoritative endpoint flux values with the same forcing,
topology, and model identity.

## 1.875-second blocker receipt inventory

Ran evidence fixes support `1.875 s`, half supports `0.9375 s`, complete-energy
difference `27.2131278332233 J m^-2`, scaled LTE
`1.9181115296775517e6`, and next prohibited half support `0.46875 s`.

The current rejection path does not return the coarse/fine
`DirectSnowTerminalEventResult`, so the exact component values and receipt
digests are not externally retained after `BelowCarrierDomain`. This is an
evidence-surface gap, not permission to infer components from `Q_complete`.
The successor contract requires a read-only rejected-trial evidence record for
each admitted call containing:

- exact support, role and attempt/coupling ordinals;
- beginning joint and complete-owner-set digests;
- ordered component/destination/lane carrier receipt digests;
- prescribed parcel/radiation total digests and values;
- endpoint shortwave, longwave, sensible, vapor/latent and snow--soil operands;
- each arm's ice, liquid, cold content, complete component energies,
  unallocated energy, ending joint and hydrology receipt;
- LSE admission receipt and proof that no call below `600 ms` occurred.

Until a contract-derived characterization test retains those existing
authoritative receipts, only the already recorded total discrepancy is claimed.
No component value is fabricated in this inventory.
