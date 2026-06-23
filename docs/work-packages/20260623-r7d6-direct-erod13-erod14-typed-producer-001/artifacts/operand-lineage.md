# Operand Lineage

Status: executed-held.

## Direct Erosion Operands

| Operand | Units / basis | Direct source authority | Direct producer target | Publication consumer |
|---|---|---|---|---|
| `Q` | event runoff depth / contract boundary symbol | Direct R4A/R4PQZ runoff projection; `SC-SED-001` `INV-SED-004` hydrologic input | `DirectErosionWave1Inputs.q` / Wave-1 forcing | EROD13 continuity and EROD14 `qout` derivation |
| `peakro` | `m^3 s^-1` | Direct runoff peak-duration producer or R7D6 typed WB16-compatible direct producer; `SC-SED-001` EROD13 Wave-1 activation | `DirectErosionWave1Inputs.peakro` | HBP peak and EROD13/EROD14 hydrologic forcing |
| `watdur` | `s` | Direct runoff duration producer; must satisfy `watdur = Q / peakro` within `TOL-SED-001` when active | `DirectErosionWave1Inputs.watdur` | HBP duration and EROD13 forcing |
| `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | branch diagnostics | R7D6 direct WB16 peak-duration span, ported from the existing hydrology kernel WB16 branch logic | `DirectPeakRunoffState` and shadow projection | Direct publication peak metadata and erosion hydrologic forcing audit |
| `Ie`, `te` | `m s^-1`, `s` | Direct rainfall/hyetograph effective intensity and duration from direct WB14/R4K inputs | `DirectErosionWave1Inputs.effective_rainfall` | EROD13 interrill delivery |
| `fs`, `ft`, `taufe`, `q` | fraction-like shear partition, `Pa`, `m^2 s^-1` | `SC-HYDRAULICS-001` / `SC-SED-001` EROD13 coupling; direct typed seed extraction required | `DirectErosionWave1Inputs.shear` | EROD13 detachment/deposition branch |
| `G`, `Di`, `beta`, `vf`, `dGdx` | sediment load/rate branch inputs per `SC-SED-001` | Direct typed erosion seed/carry; no zero fallback when Wave-1 enabled | `DirectErosionWave1Inputs.branch` | EROD13 continuity and branch outputs |
| `cntlen`, `kr`, `kradjf`, `tcadjf`, `shrsol`, `tcend`, `shcrit`, `detinr`, `effdrr`, `effdrn`, `veleff`, `pkro` | mixed Chapter-11 normalized parameter basis | Direct typed erosion seed/carry from parsed/static runtime authority; `SC-SED-001` `INV-SED-007` | `DirectErosionWave1Inputs.normalization` | EROD13 `eta`, `taucn`, `theta`, `phi` |
| `erod13_tc_k`, `erod13_tc_m` | transport-capacity coefficients | Direct typed seed; `SC-SED-001` `INV-SED-006` | `DirectErosionWave1Inputs.transport_capacity` | EROD13 `Tc` |
| `Dc`, `Tc`, `Df`, `eta`, `taucn`, `theta`, `phi` | EROD13 core outputs; signed `Df` | Direct EROD13 span | `DirectErosionWave1State` and downstream operands | EROD14 `theta` and diagnostics |
| `erod14_case`, `erod14_Qj_minus_1`, `erod14_Vj`, `erod14_Qj`, `erod14_Fh`, `erod14_Fp` | case/branch scalars | Direct typed EROD14 input/carry; `SC-SED-001` `INV-SED-008` | `DirectErosionWave2Inputs.case` | EROD14 branch classifier |
| `erod14_xtop`, `erod14_xbot`, `erod14_xdetst`, `erod14_ldtop`, `erod14_ldbot`, `erod14_lddend`, `erod14_qout`, `erod14_qin`, `erod14_qostar`, `erod14_slplen` | geometry/load/flow transition basis | Direct typed EROD14 input/carry; downstream `qin` cannot be accepted from water transfer alone under `INV-SED-012` | `DirectErosionWave2Inputs.transition` | EROD14 deposition/enrichment |
| `erod14_class_count`, `erod14_ktrato`, `erod14_ainftc`, `erod14_binftc`, `erod14_cinftc`, `erod14_beta`, `theta`, class `fall/frcflw/frac/fidel/tcf1` | class transport/enrichment basis | Direct typed EROD14 input and carried class state | `DirectErosionWave2Inputs.class_state` | EROD14 class updater |
| `erod14_gend_*`, `erod14_sedmax_*`, `sed_frac_*`, `erod14_sumg`, `ER` | class output mass/fraction and enrichment ratio | Direct EROD14 span; `SC-SED-001` `INV-SED-009` | `DirectErosionWave2State` / downstream operands | EROD15 export and diagnostics |
| `total_detachment_kg` | `kg`; event total | `max(erod14_sumg, 0)` per `SC-SED-001` EROD15 rule | `DirectPublicationErosionOperands.total_detachment_kg` and HBP alias | HBP event payload; PASS `tdet` |
| `total_deposition_kg` | `kg`; event total | `max(erod14_lddend, 0)` per `SC-SED-001` EROD15 rule | `DirectPublicationErosionOperands.total_deposition_kg` and HBP alias | HBP event payload; PASS `tdep` |
| `particle_class_count` | count | `erod14_class_count` per EROD15 | direct erosion publication metadata | HBP/PASS class validation |
| `sediment_concentration_kg_m3[class]` | `kg m^-3`; class concentration | `erod14_gend[class] / erod14_qout` when `qout > 0`, else `0` per EROD15 | `DirectPublicationErosionOperands.sediment_concentration_kg_m3` | HBP class concentration; PASS `sedcon_1..5` |
| `particle_flow_fraction[class]` | fraction | `sed_frac[class]` per EROD15 | direct erosion downstream/publication operands | routing/HBP payload validation |

## Anti-Alias Candidates

- `total_detachment_kg` must not alias `total_deposition_kg`, `erod14_lddend`,
  `sediment_concentration_kg_m3_0001`, runoff volume, or zero defaults.
- `total_deposition_kg` must not alias `erod14_sumg`, `total_detachment_kg`,
  runoff/infiltration/storage deltas, or zero defaults.
- `sediment_concentration_kg_m3[class]` must not alias particle-flow fraction,
  class load `gend[class]`, detached/deposited mass totals, HBP scalar class 1
  when another class is nonzero, water-transfer fields, or runtime aliases.
- `erod14_qin` must not be accepted from public WAT/WB13 `UpStrmQ`, aggregate
  runoff, or R7D4 water-transfer evidence alone. `INV-SED-012` requires
  prior-OFE erosion `qout` plus particle/class-fraction handoff before claiming
  sediment-coupled downstream `qin`.
- `peakro`/`watdur` used by erosion must not alias publication floors used for
  near-zero HBP formatting unless the direct producer proves the active
  zero-runoff branch.

## Authority Decision

- R7D6 may reuse baseline-authoritative EROD math only after separating it
  from compatibility `HillslopeKernelRequest`/WB13/runtime-surface authority.
  A direct span must own typed inputs, state mutation, downstream operands, and
  shadow projection.
- R7D6 may extract initial seed values from parsed/static lane seed authority
  only after each seed is copied into typed direct input structs with unit and
  finite/domain guards. Passing the whole compatibility-shaped runtime surface
  into the EROD14 kernel is not acceptable direct authority.
- R7D6 must keep the R7D5 fail-closed guard until
  `DirectPublicationErosionOperands` are populated from the typed producer.
- R7D6 removed the fabricated MOFE03 Wave-2 `erod14_lddend = 0.3` default.
  Missing direct EROD14 deposition authority now seeds `erod14_lddend = 0.0`
  unless a real seed symbol overrides it; otherwise direct publication creates
  non-parity deposition mass with no operand lineage.
- R7D6 does not resolve whether compatibility PASS/HBP `peakro = 0.0` is
  correct. Direct WB16 peak-duration operands are retained as producer
  authority, and R7D7 must adjudicate the remaining publication residual.
