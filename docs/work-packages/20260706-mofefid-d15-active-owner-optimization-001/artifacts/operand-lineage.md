# Operand Lineage (D15A-P0)

Status: **EXECUTED** (authored before production edits, per the
Conservation/Publication Acceptance Rule).

Evidence mode: Static (source read; file:line anchors at the pre-implementation
tree `9f536aad` + this package's S4 optimization diff).

Legend: RUN = `crates/openwepp-runner/src/hillslope/`,
ORCH = `crates/openwepp-hillslope-orchestrator/src/direct_runtime/`,
OFE = `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`.

## Routed source series (per active lane-day)

| Operand | Units / basis | Producer (source authority) | In-memory object | Active consumer | Old path bypassed |
|---|---|---|---|---|---|
| `wb14_hourly_excess_m[h]` | m depth per hour slot, lane-local | R4K WB14 infiltration (`SC-RUNOFFPART-001`) | `DirectDayFrame.wb14_hourly_excess_m` (ORCH `00_core_frames.rs:1096`) | D12-limb input to `dc01_surface_runoff_hourly_weights` (ORCH `runoff.rs:1405`) → active source series | none (same limb authority as DC01/shadow) |
| `hourly_saturation_carry_m[h]` (`ui_SCrunf` lineage, `SC-SUBHYD-001#INV-SUBHYD-023`) | m depth per hour slot, lane-local | R4O subsurface compute | `DirectDayFrame.subsurface_compute_shadow_projection.hourly_saturation_carry_m` (ORCH `subsurface.rs:1019`) | same weights function → active source series (the GAP-006 exfiltration limb) | none |
| `snow.hourly_routed_melt_m[h]` (D12 limb, `SC-RUNOFFPART-001#INV-RUNOFFPART-022`) | m depth per hour slot | R4G snow coupling | `DirectDayFrame.snow_coupling_downstream_operands.hourly_routed_melt_m` (ORCH `storage.rs:1432`) | same weights function → active source series | none |
| `q_runoff_m` | m depth, lane-local (rainfall-only supply in active mode) | R4A runoff partition | `DirectDayFrame.runoff_shadow_projection.q_runoff_m` (ORCH `runoff.rs:2344-2355`) | day total for weights×total source reconstruction; erosion weight validation pair | in DC01 mode this depth included admitted surface-runon effects; active mode supersedes (INV-OFEROUTE-009) |
| source depth series | `depths[h] = weights[h] × q_runoff_m`; Σ = `q_runoff_m` (supply-reconstruction hard-fail ≤ 1e-9 rel) | ADR-0036 weights-times-total (rev-15 publication-side seam form, now consumed pre-publication) | built in the executor active routing step | `seam_source_rates_from_hourly_depths` (OFE `seam.rs:69`, the recorded `/3600` helper) → KWE forcing | shadow reconstructed the same series post-publication from rows; active mode consumes the live frame |

## Rev-21 friction operands (active consumption — same sources as the shadow)

| Operand | Units | Source (authority) | Active path |
|---|---|---|---|
| `k_o`, form `C_d`, `D_r`, `lambda`, veg `C_d` | −, −, m, −, − | native management `routing_coefficients` extension (rev 20), fail-closed if missing (RUN `00_builders_and_authority.rs:31-39`) | per-lane static in the active config the runner builds with the SAME builder the shadow uses |
| `I_h` | m s⁻¹ | `DirectDayFrame.wb14_hourly_rainfall_m[h] / 3600 s` (rev 21; ORCH `00_core_frames.rs:1101`) | executor routing step builds the intensity series from the live frame |
| `LAI` | m² m⁻² | post-growth `DirectDayFrame.evapotranspiration_compute_inputs.leaf_area_index` (ORCH `evapotranspiration.rs:632`) | read in the executor routing step, post-growth by pipeline order |
| `h_c` | m | typed-management `canhgt` on the runner lane authority (RUN `00c_day_input_builder_impl.rs:369`); hard fail when `LAI > 0` and `h_c` missing/non-positive | per-lane static in the active config; the executor re-validates the vegetated-lane guard per day |
| slope geometry (`slplen_m`, `field_width_m`, mean gradient) | m, m, m m⁻¹ | Wave-1 operand seed (RUN `00_builders_and_authority.rs:40-53`) | per-lane static in the active config (same extraction as `laned_shadow_geometry`) |

## Routed water handoff (inter-OFE, active)

| Operand | Units / basis | Producer | Consumer | Old path bypassed |
|---|---|---|---|---|
| outlet conservative bin series (`outlet_bin_outflow_m2`, `outlet_bin_spans_s`, `outlet_bin_dt_s`) | m² per unit width per bin (× lane width ⇒ m³) | lane `i` solver run (rev-24 conservative handoff) | lane `i+1` solver upstream integral, width-scaled `w_i/w_{i+1}` (same rule as OFE `cascade.rs:207-235`) | DC01 daily-lump surface runon: `transfer.surface_carry_m[0] = q_runoff_m` (ORCH `03_executor.rs:732`) → R4J `runon_input_m` (ORCH `runoff.rs:557-566`) → R4K supply admission (ORCH `runoff.rs:638-653`) — DISABLED for active lanes (surface only) |
| `lateral_carry_m[h]` (subsurface inter-OFE carry, `ui_LfCrf` lineage) | m per hour slot, area-ratio scaled at R4J | R4O subsurface | UNCHANGED — stays on the DC01 lateral admission (the router supersedes SURFACE runon only, GAP-006) | none (explicitly NOT bypassed) |

## Erosion shape (active)

| Operand | Units | Producer | Consumer | Old path bypassed |
|---|---|---|---|---|
| `routed_hydrograph_runoff_fraction[24]` | unit-sum weights (dry days all-zero) | executor routing step: hour-aligned mass sums of the lane's outlet bin series; tail (> hour 24) folds into hour 24 (rev-27 rule; surfaced `routed_tail_fold_m3`) | `DirectErosionInputs.hydrograph_shape_authority = RoutedHydrograph` → `r7d8_surface_hourly_weights` (ORCH `erosion.rs:402-416`) → `wave1_hourly_weights` (`erosion.rs:576`) → Wave-1 hourly plan, `publication.hourly_runoff_fraction` (`erosion.rs:706`), and the E.3 erosion-inflow publisher (ORCH `03_executor.rs:584`) | `r7d8_dc01_surface_hourly_weights` (DC01 shape authority) — not called for active lanes |

## Closure operands (per active day, hillslope level)

All converted to m³ with the lane area (`A_i` from the run-lane state; the day
frame itself carries `upstream_area_ratio` only).

| Term | Source surface | Units in source | Basis note |
|---|---|---|---|
| precip input | `storage_downstream_operands.precip_input_m` (R4B; ORCH `storage.rs:1609-1623`) | m | × `A_i` |
| snow coupling, frost liquid delta, interception, runon input (lateral-only in active), ET net (`evapotranspiration_m` − `…_storage_return_m`), deep seepage, subsurface loss | same R4B operand struct | m | × `A_i`; per-lane R4B closure (`closure_residual_m`) already hard-guards the lane identity |
| ΔS soil | `storage_reconciled_m − storage_initial_m` (R4B) | m | × `A_i` |
| lateral export (`latqcc` bypass) | outlet lane's subsurface lateral flow (`subsurface_compute_shadow_projection.lateral_flow_m`, ORCH `subsurface.rs:1012`) | m (depth; the mm→m³ helper in the contract's unit table applies to the PUBLISHED mm surface — the runtime uses the in-frame depth × `A_outlet`) | bypasses the router (GAP-006 D3). **Booking (QA-M2 corrected):** in the enforced day identity the lateral export rides INSIDE the per-lane `subsurface_loss_m` operands — an explicit extra `− latqcc` subtraction would double-count. It is SURFACED as its own recorded evidence operand (`total_latqcc_outlet_m3`, all days), and reconstructs the published parquet `sbrunv` sum to 1 ulp. |
| routed injection | Σ `depths[h] × A_i` per lane | m³ | equals Σ `q_runoff_m × A_i` by supply reconstruction |
| routed outlet | terminal lane's booked `outflow_m2 × width` | m³ | the scheme-actual boundary flux (rev 24) |
| router mesh ΔS | Σ lanes' solver `storage_change_m2 × width` | m³ | booked in the day's ΔS; reset next day (rev-27 window row); surfaced `routed_end_window_storage_m3` |
| positivity clamp | Σ lanes' solver `positivity_clamp_m2 × width` | m³ | the INV-OFEROUTE-006-surfaced scheme injection; booked EXPLICITLY on the input side of the day identity (baseline measured ≥ 3.8 % of source on H2637 — see `baseline-profile.md`) |

Day identities (rev-27 runtime form, QA-review-reconciled — the enforced set):
(a) supply reconstruction `|Σ depths − q_runoff| ≤ 1e-9` rel (per lane-day);
(b) ROUTER-INTERNAL day residual `injected + clamp − outlet − ΔS_mesh` ≤ 1e-9
rel (solver-family books: per-lane conservation + handoff telescoping — NOT a
soil-seam check); (c) SEAM cross-ledger `|injected_solver − Σ q_runoff·A_i|`
≤ 1e-9 rel — INDEPENDENT ledgers (solver booking vs soil release), made exact
by the hourly forcing breakpoints + the recorded mesh-basis conversion
`A_lane/(slplen·width)`; (d) the assembled day identity
`Σ_i A_i·(IN_i − OUT_i − ΔS_i) + Σ q_runoff·A_i + clamp − Q_routed_out −
ΔS_mesh` ≤ 1e-6 × max operand — with the honesty note that the per-lane R4B
residual is zero by construction, so (d)'s content is the (b)+(c)
composition; the lateral export is inside `subsurface_loss` (no separate
subtraction). Measured maxima (seam-fixed run): 7.3e-16 / 2.5e-13 / 5.0e-14 /
2.5e-13. Diagnostic vs authoritative: manifest `laned_active` fields are
evidence surfaces; the authoritative water surfaces remain the per-lane WB
publications (SC-RUNOFFPART-001) and the typed closure guards.
`total_source_m3` (manifest) is the SOLVER-BOOKED injection — equal to the
soil release within the seam tolerance by check (c), not merely by
description (QA-L2 resolved).

## Anti-alias notes (what the expected values must differ from)

- The active source series total closes to `q_runoff_m` (rainfall-only), which
  on multi-OFE lanes DIFFERS from the DC01-mode `runvol/area` the shadow used —
  a test asserting equality with the DC01-mode value would be wrong.
- The routed erosion weights must differ from the DC01 source weights on days
  where routing shifts timing (storage delay) — the D13 tests already pin the
  supersede direction; the active tests must not alias the two.
- `QOFE`/`Q` publication scaling (RUN `05_runner_execution_and_outputs.rs:203-204`)
  does NOT read `transfer.surface_carry_m` — zeroing the surface transfer for
  active lanes does not touch the Q/QOFE publication math (verified by the
  transfer-consumer sweep; consumers are R4J, DC01 admission shapes, R5B/R3A
  input accounting only).
