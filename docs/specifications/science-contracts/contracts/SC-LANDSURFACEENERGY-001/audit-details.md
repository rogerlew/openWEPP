[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | always | shared authority | whole chapter |

<a id="audit-details"></a>
# Authority and enforcement audit details

All sections remain normative. Read the relevant complete sections for source provenance, enforcement-path, historical applicability or promotability claims. Current physical rules and unresolved authority limits remain in interface and common-details; this relocation does not resolve or demote any gap.

<a id="purpose"></a>
## Purpose

Define the first-class control-volume, conservation, custody, constitutive and
failure authority for the default-off `OPENWEPP_SNOW_FREE_LSE_V1` model.
Version 3 releases implementation authority for snow-free bare-mineral and
forest-litter surfaces coupled to `OPENWEPP_C3_WOODY_V8`; it authorizes no
production selector, cutover, calibration or snow-terminal recipient.

Sections from "Scientific Scope" through the gap register preserve the
version-2 conservation and missing-authority baseline as historical context.
Where those sections say `future`, `missing`, `gap`, `NON_PROMOTABLE`, or
describe `M_l` as LSE-mutated, the named Version-3 constitutive authority below
prospectively supersedes that statement for `OPENWEPP_SNOW_FREE_LSE_V1` only.
All exact-one conservation, failure, adjacent-owner and no-real-consumer rules
not expressly superseded remain binding.

<a id="scope"></a>
<a id="scientific-scope-and-explicit-out-of-scope-boundaries"></a>
## Scientific Scope and Explicit Out-of-Scope Boundaries

In scope is one horizontal-area-normalized surface control volume over one
explicit interval. It owns the surface temperature/energy state, surface
liquid energy state, flux sign convention, exact-one component lineage, and
the coupled water/energy bookkeeping at its boundary.

Out of scope in version 1:

- production Rust, scheduling, selectors, defaults, publication, or cutover;
- any snow-present thermodynamics, which remain owned by
  `SC-SNOWENERGY-001` and `SC-SNOWFREEZE-001`;
- consumption of schema-v8 terminal liquid, energy, or unevaluated time;
- recomputation of ET, infiltration, runoff, percolation, soil-water
  withdrawal, frost fronts, or subsurface phase change;
- a new empirical parameter, calibration result, remote-sensing input, or
  wepppy-owned climate/GIS/run-state concern; and
- provisional, surrogate, heuristic, or comparator-targeted physics.

<a id="sources"></a>
<a id="authority-anchors-with-top-down-citations"></a>
## Authority Anchors with Top-Down Citations

| Anchor | Source | Binding use | Evidence |
|---|---|---|---|
| `REF-LANDSURFACEENERGY-001` | `references/50201000/chap5.pdf`, §§5.1-5.3 | Daily water closure and modified-Ritchie ET context; it does not specify a complete prognostic surface-energy solver. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-002` | `references/50201000/chap4.pdf`, §4.2 | Green-Ampt/Mein-Larson infiltration and rainfall-excess ownership boundary. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-003` | pinned `dac3c950...:src/contin.for`, lines 839-882 and 907-922 | Winter branch produces `wmelt`; rain plus melt updates the `r5` antecedent-history lineage. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-004` | pinned `dac3c950...:src/watbal.for`, lines 331-344, 431-498 | Liquid ingress precedes percolation and ET; water state is mutated by the owning hydrology routines. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-005` | pinned `dac3c950...:src/evap.for`, lines 171-360 and 428-680; `src/evappm.for`, lines 178-430 | Legacy radiation/ET demand and soil/residue/plant water withdrawal; not an hourly surface enthalpy solver. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-006` | pinned `dac3c950...:src/frostn.for`, lines 383-686; `src/frzng.for`, lines 331-381; `src/frznw.for`, lines 62-113 | Soil/residue/snow thermal resistance and frozen-soil phase mechanics; `surtmp` is an input rather than a coupled solved surface state. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-007` | `SC-CLIMATE-001`, `SC-EVAP-001`, `SC-WATBAL-001`, `SC-RUNOFFPART-001`, `SC-SOIL-001`, `SC-SUBHYD-001` | Existing forcing, ET, water, runoff, soil, and subsurface ownership. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-008` | `SC-SNOWENERGY-001#INV-SNOWENERGY-034` and `GAP-SNOWENERGY-011` | Schema-v8 terminal payload is censored and has no receiving-surface authority. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-009` | physical conservation of mass and first-law energy accounting | Exact ledger closure and equal/opposite shared-boundary fluxes. | `[INFERENCE][Static]` |
| `REF-LANDSURFACEENERGY-010` | `crates/openwepp-meteorology/src/surface_energy.rs` | Existing typed flux algebra is reusable mechanics only; it is not proof of a complete runtime owner or consumer. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-011` | IEEE 754 binary64 round-to-nearest, ties-to-even; exact integer arithmetic over finite-binary64 dyadics; `SC-SURFACELIQUID-001#INV-SURFACELIQUID-022` | Receiver-owned exact soil-layer enthalpy total, one correctly rounded high term, canonical signed-dyadic carry, and exact accepted-credit custody. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-LANDSURFACEENERGY-012` | IEEE 754 binary64 round-to-nearest, ties-to-even; exact integer arithmetic over finite-binary64 dyadics; physical conservation of accepted surface-energy operands; `SC-SURFACELIQUID-001#INV-SURFACELIQUID-023` | LSE-owned exact per-tile surface enthalpy, immutable high-term mirrors, exact retained-ingress credit custody, and successor restart/rollback. | `[DIRECT][Static] + [INFERENCE][Static]` |

All pinned citations mean `git show
dac3c950d8b16cc73774bf5ce2e7e11f80baac70:<path>`; the mutable checkout
HEAD is not normative.

<a id="enforcement"></a>
<a id="invariants-and-invariant-guard-map"></a>
## Invariants and Invariant Guard Map

| Binding reference | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|

Guard-map enforcement in version 1 is the contract-derived integration test
and package review. Runtime mappings are intentionally future obligations; an
implementation package must replace each future mapping with a typed path and
evidence artifact before promotion.

<a id="invariant-guard-map"></a>
### Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-LANDSURFACEENERGY-001` | focused contract schema/unit assertions | test | blocked promotion | package contract-test evidence |
| `INV-LANDSURFACEENERGY-002` | future LSE input validator | runtime | `LSEB-E-001`; currently `HOLD` | `GAP-LANDSURFACEENERGY-003/004` |
| `INV-LANDSURFACEENERGY-010` | future independent energy reconstruction | runtime | `LSEB-E-011`; currently `HOLD` | `GAP-LANDSURFACEENERGY-001/004` |
| `INV-LANDSURFACEENERGY-011` | future independent water reconstruction | runtime | `LSEB-E-012`; currently `HOLD` | `GAP-LANDSURFACEENERGY-004` |
| `INV-LANDSURFACEENERGY-012` | future latent mass/energy identity | runtime | `LSEB-E-013`; currently `HOLD` | `GAP-LANDSURFACEENERGY-002/003` |
| `INV-LANDSURFACEENERGY-013` | future equal/opposite ground-flux consumer | runtime | `LSEB-E-014`; currently `HOLD` | `GAP-LANDSURFACEENERGY-002/004` |
| `INV-LANDSURFACEENERGY-014` | future liquid mass/advected-energy lineage join | runtime | `LSEB-E-010/011`; currently `HOLD` | `GAP-LANDSURFACEENERGY-002/003` |
| `INV-LANDSURFACEENERGY-015` | future end-storage domain validator | runtime | `LSEB-E-015`; currently `HOLD` | `GAP-LANDSURFACEENERGY-003/004` |
| `INV-LANDSURFACEENERGY-020` | future branch selector plus poison vectors | runtime | `LSEB-E-020/021`; currently `HOLD` | `GAP-LANDSURFACEENERGY-004/005` |
| `INV-LANDSURFACEENERGY-021` | terminal-field rejection assertion | test | `LSEB-E-021` / blocked promotion | focused test + snow contract |
| `INV-LANDSURFACEENERGY-022` | scheduler-order and real-consumer gate | governance | blocked promotion | `GAP-LANDSURFACEENERGY-004` |
| `INV-LANDSURFACEENERGY-030` | owner-boundary assertions | test | blocked promotion | focused test + adjacent contracts |
| `INV-LANDSURFACEENERGY-031` | dual science review and owner assertions | governance | blocked promotion | package review artifacts |
| `INV-LANDSURFACEENERGY-032` | real-consumer reachability gate | governance | blocked promotion | `GAP-LANDSURFACEENERGY-004` |
| `INV-LANDSURFACEENERGY-040` | gap-label assertion and package disposition | governance | `NON_PROMOTABLE` | focused test + disposition |
| `INV-LANDSURFACEENERGY-114` | terminal support/operand validator and receiver selection | default-off runtime/test | typed reject/rollback | terminal handoff package |
| `INV-LANDSURFACEENERGY-115` | liquid-energy join and atomic envelope | default-off runtime/test | typed reject/rollback | terminal handoff package |
| `INV-LANDSURFACEENERGY-130` | covered-liquid finalization before ledger materialization | runtime/test | exact reference-state canonicalization or existing typed reject | adaptive microstepping package |
| `INV-LANDSURFACEENERGY-138` | covered-column Jacobian probe-domain validator | runtime/test | centered interior, unique inward closed-bound derivative, or typed reject | covered solver contract/unit/runtime vectors |
| `INV-LANDSURFACEENERGY-139` | covered-column first-domain-valid-halving no-update witness | runtime/test | after the full trial fails the existing no-update witness by domain or governed-step refusal, accept the unchanged current iterate only when the complete current residual vector and every governed first-halved prospective step pass; otherwise retain strict-decrease update or typed numerical rejection | covered solver contract/unit/runtime vectors plus interior-terminal real consumer |
| `INV-LANDSURFACEENERGY-150` | `SoilThermalOwnerEnvelopeV2`, exact aggregation/rounding, typed energy-credit receipt, restart/checkpoint replay, complete-owner join, and real consumers | runtime/test/governance | canonical finite exact total and atomic install, or `LSEB-E-049`; blocked promotion until WAT5, `p61`, and native-forest consumers pass | exact-carry focused vectors, split-restart equivalence, rollback hashes, and three real-consumer gates |
| `INV-LANDSURFACEENERGY-151` | `LseSurfaceEnthalpyOwnerEnvelopeV1`, exact surface-energy operand aggregation, immutable V2/V3 high-mirror join, restart/checkpoint replay, `SurfaceLiquidCompleteOwnerProjectionV4`, and real consumers | runtime/test/governance | canonical finite exact total and atomic install, or `LSEB-E-050`; blocked promotion until `p61` and native-forest consumers pass | exact surface-carry vectors, split-restart equivalence, rollback hashes, and two real-consumer gates |
| `INV-LANDSURFACEENERGY-153` | exact-surface receipt ending-posture and parent/child-support validator plus V3/V2 mirror join | runtime/test/governance | deterministic partial/final lineage or `LSEB-E-050` / `SURFACELIQUID-E-012` with byte-exact rollback | first/middle/final child, mid-parent restart, wrong posture/bounds/mixed marker/early advance, rollback |
| `INV-LANDSURFACEENERGY-154` | immutable represented-snow classifier, standard Stage-3 covered-column charge, native identity/receipt join, inactive-litter custody validator, and terminal split | runtime/test/governance | one charged standard map or typed rejection with byte-exact owner rollback | typed classification, zero litter-phase calls, exact optical/lower-boundary retention, no second envelope, unchanged V3/V4 bytes, snow-free transition |
| `INV-LANDSURFACEENERGY-155` | unpublished-continuation constructor, typed V3 candidate-only soil-beginning branch, non-owner projection guard, complete outer replay, and single owner/restart install | runtime/test/governance | authenticated private read or `LSEB-E-049`; no intermediate owner; byte-exact rollback | contiguous support, predecessor-trial substitution, owner/restart-byte absence, exact final replay, one install, rollback |
| `INV-LANDSURFACEENERGY-156` | bounded-phase raw/retained/spill splitter, typed phase-source receipt, V3 current-ingress handoff, named exact-surface debit operand, and independent mass/enthalpy replay | runtime/test/governance | exact spill and one WB14 handoff or typed `LSEB-E-047/048/050`; no owner mutation on failure | zero/below/at/above capacity, melt-created spill, mass/enthalpy and area-basis closure, receipt/support/transaction/key substitution, no-resolve, one ingress, rollback |
| `INV-LANDSURFACEENERGY-157` | typed native-phase versus ordinary-finalized-use partition, phase-adjusted V2 resource join, canonical ordinary debit, and single current-ingress continuation | runtime/test/governance | every finalized row consumed exactly once under its original custody or typed rejection; no resource replacement or owner mutation on failure | heterogeneous open/covered batch, zero/nonzero ordinary rows, native-row replay, duplicate/omitted/foreign row, canonical order, one ingress, rollback |
| `INV-LANDSURFACEENERGY-158` | topology-ranked V16 exact-surface owner records and accepted operand groups at the authenticated configuration join | runtime/test/governance | accept opaque multi-digit OFE identities in configuration order; reject duplicate/omitted/substituted/stale/reordered custody with rollback | topology `ofe-9 -> ofe-10`, reverse/nonmonotone IDs, within-OFE order, duplicate/omission/substitution, stale digest, rollback |
| `INV-LANDSURFACEENERGY-159` | private immutable validated resident/resource handoff, parent-static structural plan, per-map forcing proof, and resident-revision-sourced native proof with exact pointer, revision, generation, configuration, topology, and lineage binding | runtime/test/governance | join each proof only at the validation position it replaces; every map freshly validates exact forcing once and all map-dynamic surfaces; every resident successor and every restart/external/durable/untrusted boundary receives full validation | O(1) resident install, append-tail validation, one parent-static plus one forcing/dynamic validation per map, native proof consumption only in native regimes, structural/native object poisons, proof reuse/transfer, combined first-error poisons, restart replay, rollback |
| `INV-LANDSURFACEENERGY-160` | private snow-free provisional-physical ending consumed by the exact final accepted-slab reseal | runtime/test/governance | execute energy and coupled owner physics once; reseal only final receipt identities or reject atomically | exact direct-versus-reuse owner bytes, one provider/phase/ingress/WB14/soil call, non-slab poisons, single-use/restart rejection, rollback |
| `INV-LANDSURFACEENERGY-162` | represented-snow ground/soil anchor classifier and exact probe-residual assembler | runtime/test | bit-identical full-evaluator residual vectors and Jacobian or complete-evaluator path; typed existing domain/error precedence | full-evaluator differential oracle, centered/inward stencil vectors, dependency poisons, evaluation-call counts, authentic runner parity |
| `INV-LANDSURFACEENERGY-163` | private covered-leaf maximum-demand exact-reuse classifier | runtime/test | reuse only an identical successful leaf result or execute the unchanged complete beta-one call | per-branch and exact-beta differential oracle, call counts, boundary probes, typed-error precedence, authentic runner parity |
| `INV-LANDSURFACEENERGY-164` | versioned/hashed topology-generic component-temperature graph, immutable sweep base and single-use signed-probe replay through one shared canonical evaluator tail | runtime/test | select complete evaluation before replay on ordinary ineligibility; otherwise replay reachable nodes in source order or fail directly on integrity/post-start error | forced-complete oracle, exact direct-edge graph, normative fallibility/crossability matrix, source-real error/rollback corpus, scoped/aggregate buckets, reciprocal-longwave, duplicated wet routing, terminal descendants, full-solve/release parity |
| `INV-LANDSURFACEENERGY-041` | provenance/no-proxy review | governance | blocked promotion | baseline map + reviews |
| `INV-LANDSURFACEENERGY-042` | future recipient-specific radiation ledger and poison vectors | runtime + test | blocked promotion on omitted, duplicated, or aliased recipient | vegetation/LSE integration package |
| `INV-LANDSURFACEENERGY-043` | future latent mass-energy lineage join | runtime + test | blocked promotion on missing/mismatched `h_v`, duplicate debit, or amount/rate basis mismatch | vegetation/LSE integration package |

<a id="producer-obligations-and-consumer-obligations"></a>
## Producer Obligations and Consumer Obligations


<a id="gaps"></a>
<a id="gap-register-and-promotability-labels"></a>
## Gap Register and Promotability Labels

| Gap ID | Gap | Required closure | Label |
|---|---|---|---|
| `GAP-LANDSURFACEENERGY-001` | V1/v2 lacked a complete snow-free surface-temperature and coupled energy-storage algorithm. | Version 3 named model and independent vectors. | `AUTHORITY_ADMITTED`, implementation pending |
| `GAP-LANDSURFACEENERGY-002` | V1/v2 lacked jointly authorized sensible, latent, ground, liquid advection, and storage families. | Version 3 exact owner/source equations. | `AUTHORITY_ADMITTED`, implementation pending |
| `GAP-LANDSURFACEENERGY-003` | V1/v2 lacked complete latent heat, storage, resistance, substrate, and tolerance authority. | Version 3 strict configuration and numerical contract. | `AUTHORITY_ADMITTED`, implementation pending |
| `GAP-LANDSURFACEENERGY-004` | No first-class runtime state, ledger, domain error, scheduler span, or real downstream consumer exists. | Later scoped implementation plus real-consumer proof. | `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-LANDSURFACEENERGY-005` | Schema-v8 snow terminal liquid, energy, and remaining time are censored. | Atomic two-contract cutover with exact-one custody, rollback/defaults, and receiving-surface closure. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-LANDSURFACEENERGY-006` | Legacy daily ET and frost mechanics are not complete LSE authority. | Version 3 uses the selected external stack; legacy remains unchanged comparator behavior. | authority portion admitted; runtime/cutover pending |
| `GAP-LANDSURFACEENERGY-007` | V1 soil-thermal binary64 state cannot retain accepted energy below the high-term ULP. | Version 15 V2 owner/receipt/restart/checkpoint exact carry, exact reconstruction, rollback, and real WAT5/`p61`/native-forest adoption. | `AUTHORITY_ADMITTED`, implementation pending; `NON_PROMOTABLE` |
| `GAP-LANDSURFACEENERGY-008` | Frozen LSE V3 and surface-owner V2 binary64 surface enthalpy cannot retain an accepted per-tile energy credit below the high-term ULP. | Version 16 LSE exact-surface owner/receipt/restart/checkpoint, immutable high-mirror join, exact reconstruction, rollback, and real `p61`/native-forest adoption. | `AUTHORITY_ADMITTED`, implementation pending; `NON_PROMOTABLE` |

The first safe later implementation slice is a default-off, snow-free-only
typed request/state/component-ledger/result using only admitted operands,
explicit duration, atomic validation, and an independent internal ledger
consumer. It must reject snow and terminal handoffs and must not mutate ET,
runoff, infiltration, soil, or frost. That slice is evaluation mechanics only;
production coupling requires later authority and a real process consumer.

The immutable definition identity is
`sha256:e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`
for
`docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_snow_free_lse_v1_definition.json`.


