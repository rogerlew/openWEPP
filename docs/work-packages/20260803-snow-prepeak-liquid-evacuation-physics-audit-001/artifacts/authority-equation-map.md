# Authority And Equation Map

Status: `executed`

Evidence mode: `Static + Ran`

The audit uses the canonical contracts as implementation obligations, the
WEPP Chapter 3 handbook and pinned `wepp-forest` source as provenance, and
first-principles dimensions and conservation as independent checks. Agreement
with legacy behavior is a diagnostic flag, not correctness authority.

## CoE Melt Boundary

Current Rust computes four signed empirical terms in legacy inches per hour and
then applies one `0.0254 m/in` conversion:

`M_raw = 0.0254 (A + B + C + D)`.

| Term | Current equation | Inputs and units | Authority and consumer |
|---|---|---|---|
| `A`, shortwave | `0.0607 R_h f_abs (1-cancov)` | `R_h`, `MJ m^-2 h^-1`; signed result, legacy inches per hour | Rust `infiltration_reconciliation.rs:573-585,747-750`; pinned `melt.for:142-148`; `SC-SNOWFREEZE-001` INV-052/053. The coefficient already embeds about 0.5 legacy albedo. |
| `B`, temperature/clear-sky | `(0.025/24) T_F - 0.84(1-cloud)(1-cancov)/24` | `T_F = 1.8 T_C`, a temperature departure from freezing; cloud fraction | Rust `:751-752`; pinned `melt.for:153-180`. The term is signed and is not a separable physical sensible/longwave flux. |
| `C`, wind/dewpoint | `(0.0084/24) V_mph (1-0.8 cancov)(0.22 T_F+0.78 Td_F) adj + 0.8 cancov (0.045/24)T_F` | Daily wind converted once from `m s^-1` to mph; daily dewpoint departure; `adj=1.57(10 m)^(-1/6)` | Rust `:754-775`; pinned `melt.for:209-229`. Empirical aerodynamic term, signed. |
| `D`, rain heat | `0.007 rain_in (Td_F if Td_F>0 else T_F)` | Hourly rain in legacy inches; temperature departure | Rust `:777-790`; pinned `melt.for:243-275`. |
| Applied melt | positive result capped to available pack; negative result retained in `raw` diagnostics | `m water equivalent h^-1` | Rust `:793-860,1488-1507`; `SC-SNOWFREEZE-001` INV-001/015/019. |

The Rust equations and unit conversions match pinned legacy commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. No missing `/24`, sign inversion,
or duplicate inch/metre conversion was found. The handbook differs from the
post-2007 implementation in the written `A-B+C+D` sign presentation,
cold-hour shortwave attenuation, roughness/displacement treatment, and parts of
the rain/temperature term. That is an unresolved authority/provenance tension,
not a Rust transcription defect.

The active `coe_liquid_holding_capacity_v1` path supplies
`shortwave_absorbed_fraction=1` at `infiltration_reconciliation.rs:1467-1485`.
This preserves the fixed albedo already baked into `0.0607`; it does not mean
100% of incident shortwave is physically absorbed. The separate
`coe_shortwave_albedo_v1` contract multiplies the same albedo-bearing
coefficient by a dynamic absorbed fraction, creating a static authority concern
outside the active configuration.

## Climate And Surface Inputs

SIMIMPL converts daily radiation once and distributes it hourly at
`runtime_inputs/06_simimpl28_hourly_forcing.rs:207-296`, consistent with
`SC-CLIMATE-001` INV-013. Cloud is a same-day solar-transmissivity proxy, not an
independent cloud observation:

`cloud = clamp((0.3 + 0.7*0.75^ms - radly/r3) /
[0.7*(0.75^ms - 0.4^ms)], 0, 1)`

at `06_simimpl28_hourly_forcing.rs:564-575`. The runner projects hourly
radiation, air temperature, and cloud plus daily wind/dewpoint at
`00a_snow_frost_authority_impl.rs:406-461`.

The checksum-bound retained cloud check found Snowbird wet-winter ERA cloud
`0.769` versus proxy `0.913`, correlation `0.230`. This supports a proxy
mismatch but does not quantify a stateful SWE response. The retained ERA5
horizontal-daily shortwave diagnostic was about 28.5% higher than the forcing,
which has the wrong sign for delaying `A` on that surface. Neither diagnostic
is a slope-aware hourly replay, and ERA remains diagnostic only. Exact upstream
artifact and tool hashes are bound in `audit-freeze-v3.json`.

## Liquid Capacity And Snow Mass

The active holding capacity is

`C_liq = 0.01 (1-rho_s/917) D_s`,

with metres of liquid over the snow column. It is produced at
`infiltration_reconciliation.rs:311-320`, filled and drained at `:1678-1723`,
and contracted/released at `:1799-1838`. `SC-SNOWFREEZE-001` INV-067 authorizes
the opt-in path, and INV-072 activates it as part of the default bundle. At
`rho=350 kg m^-3`, capacity is about 1.77% of pack SWE: a timing buffer rather
than seasonal storage.

The primitive authoritative daily mass identity is

`SWE_after - SWE_before = snowfall_SWE + rain_retained - pack_loss - sublimation`,

implemented and guarded at `runoff_reconciliation.rs:436-452`. The published
liquid alias is `routed_melt = pack_loss + rain_released`, established at
`runoff_reconciliation.rs:225-264`. Neither routed melt nor released rain is a
second snow sink.

## Density Compaction

The density driver currently passes

`liquid_for_compaction = pack_loss + routed_melt`

at `runoff_reconciliation.rs:2160-2188`. Because the mass ledger establishes
`routed_melt = pack_loss + rain_released`, the actual input is

`2 pack_loss + rain_released`.

The value is converted to `kg m^-2` and distributed over layers at
`09_snow_density.rs:805-813,1023-1058`; wet compaction consumes the liquid/mass
ratio at `:1499-1525`. The trace proves the duplicate data-flow alias. INV-068's
apply-once language governs a different selector, and current multilayer
authority does not prove that `routed_melt` alone is the correct complete wet-
compaction driver. The physical-defect verdict therefore remains unresolved.
Physical density is kept separate from the CoE boundary at
`infiltration_reconciliation.rs:1061-1076`, so the alias cannot directly change
authoritative SWE loss.

## Stage-3 Thermal Boundary

Stage 3 diagnoses layer cold content as

`CC = SWE rho_w c_i (0-T)`, with `c_i=2100 J kg^-1 K^-1`,

at `runoff_reconciliation.rs:1239-1250`. Refreeze capacity is

`L_refreeze = min(L_in, CC/(rho_w L_f))`, with
`L_f=333550 J kg^-1`,

at `:1126-1159`. Its valid implemented energy identity is

`Q_surface + Q_internal_conduction + Q_refreeze + CC_export
= CC_before - CC_after`

at `:788-810`. Positive excess energy is explicitly unused; it does not create
a second melt sink. The reference selector disables explicit longwave and
latent/sublimation, while sensible, basal, and advected heat are zero. This is
a bounded post-CoE diagnostic carrier, not a complete snow energy balance.
Independent reconstruction over the accepted reference traces closes this
bounded identity within `1.87e-8 J m^-2`.

`SC-SNOWFREEZE-001` INV-080/081 makes CoE SWE and routed liquid authoritative
and Stage 3 snow-neutral. The runtime diagnostic owns the exact Stage-3 liquid
operands, but the JSONL serializer at
`00c_day_input_builder_impl.rs:1332-1400` omits incoming, routed, retained, and
liquid-residual fields. Therefore independent Stage-3 liquid closure is not
available from the frozen real-consumer evidence.
