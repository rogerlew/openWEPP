---
contract_id: SC-GWBASEFLOW-001
title: Groundwater Reservoir Baseflow Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 0.1.2
producer_scope:
  - groundwater linear-reservoir storage update
  - hillslope groundwater baseflow and deep-seepage export
consumer_scope:
  - hillslope pass and watershed/channel baseflow consumers
  - Lane D active-router water-balance export boundary
evidence_level: static
last_reviewed: 2026-07-13
supersedes: []
superseded_by: []
---

# SC-GWBASEFLOW-001 Groundwater Reservoir Baseflow Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`
Contract version: `0.1.2`

## Purpose

This contract defines openWEPP's groundwater-reservoir baseflow process
authority. It binds the Srivastava linear-reservoir groundwater/baseflow
lineage to canonical `SC-*` authority before production implementation.

The contract owns daily groundwater storage, groundwater baseflow, and deep
seepage computed from WEPP deep-percolation recharge when `gwcoeff.txt` enables
the linear-reservoir branch. It also defines the handoff obligations for
single-OFE, Lane D MOFE, hillslope pass, and watershed/channel consumers.

## Scientific Scope And Boundaries

In scope:

- linear-reservoir groundwater storage update driven by deep percolation;
- baseflow and deep seepage as fractions of groundwater reservoir storage;
- `gwcoeff.txt` process coefficients after successful parser projection;
- pass/export surfaces for groundwater baseflow and deep seepage;
- watershed/channel consumption of generated groundwater baseflow;
- Lane D active routing boundary rules that prevent loss or double counting of
  non-surface groundwater/baseflow export.

Out of scope:

- parsing grammar for `gwcoeff.txt`, owned by `SC-INFILE-GWCOEFF-001`;
- lateral subsurface export (`latqcc`) physics, owned by `SC-SUBHYD-001`;
- surface return flow / saturation-excess routing, owned at the coupling seam
  by `SC-OFEROUTE-001`;
- `chan.inp` unit-area channel baseflow coefficient `cbase`, owned by
  `SC-INFILE-CHANINP-001` and routing/channel contracts;
- nonlinear Wittenberg-style baseflow algorithms from later Srivastava lineage;
- coefficient fitting, calibration, or inferred defaults.

## Authority Anchors

| Anchor ID | Source | Contract use | Evidence |
|---|---|---|---|
| `REF-GWBASEFLOW-SRIVASTAVA-DISS-CH2` | Srivastava (2013) dissertation, Chapter 2, groundwater/baseflow equations and Priest River evaluation. | Primary equation authority for `Qb = kb * S`, `Qs = ks * S`, and daily storage recurrence using WEPP deep-percolation recharge. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-SRIVASTAVA-DISS-APP-C` | Srivastava (2013) dissertation, Appendix C baseflow code listing. | Lineage confirmation for fitted initial storage, baseflow coefficient, deep-seepage coefficient, and daily vector update shape. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-ASABE-2013` | Srivastava et al. (2013), *Transactions of the ASABE* 56(2), 603-611. | Peer-reviewed companion authority for linear-reservoir baseflow coupled to WEPP deep percolation and streamflow components. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-ASABE-2017` | Srivastava et al. (2017), *Transactions of the ASABE* 60(4), 1171-1187. | Later nonlinear/baseflow lineage and terminology context; not authority for the current `gwcoeff.txt` linear process. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-GWBASEFLOW-DUN-2009` | Dun et al. (2009), *Journal of Hydrology* 366, 46-54. | Forest subsurface/deep-percolation and lateral-flow context; distinguishes WEPP forest subsurface flow from groundwater reservoir baseflow. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-GWBASEFLOW-LEGACY-CCHRT1` | `/workdir/wepp-forest_260430_baseline/src/cchrt1.inc:7-17,31-52` at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. | Legacy symbols, units, and branch meanings for `lr_bf`, `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`, `gwbfv`, and `gwdsv`. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-LEGACY-MAIN` | `/workdir/wepp-forest_260430_baseline/src/main.for:120-136,450-465` at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. | `gwcoeff.txt` branch selection: present file sets `lr_bf=1` and reads four coefficients; missing file sets `lr_bf=0`. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-LEGACY-CONTIN` | `/workdir/wepp-forest_260430_baseline/src/contin.for:1088-1120` at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. | Daily recharge accumulation from deep percolation and reservoir/baseflow/deep-seepage update order. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-LEGACY-PASS` | `/workdir/wepp-forest_260430_baseline/src/wshpas.for:220-227,236-245,255-265,386-414,466-505,530-532`; `/workdir/wepp-forest_260430_baseline/src/wshdrv.for:515-520,845-875`; `/workdir/wepp-forest_260430_baseline/src/cstore2.inc:7-15,29-32`. | Hillslope pass and watershed-driver temporary storage of groundwater baseflow/deep seepage. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-LEGACY-CHANNEL` | `/workdir/wepp-forest_260430_baseline/src/wshcqi.for:86-159,199-207`; `/workdir/wepp-forest_260430_baseline/src/wshchr.for:133-148,183-189,205-225,260-262,696-704`. | Watershed/channel branch behavior: `lr_bf=0` consumes `cbase`; `lr_bf=1` consumes generated `tmpgwbfv`, applies `bftharea`, and prevents channel-water-balance duplication. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-LEGACY-WATBAL` | `/workdir/wepp-forest_260430_baseline/src/watbalprint.for:87-96,101-124` at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. | Water-balance publication branch: legacy `Baseflow` column is zero under `lr_bf=1` because groundwater baseflow is carried in runoff/streamflow instead. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-INFILE-GWCOEFF` | `SC-INFILE-GWCOEFF-001`. | Parser-to-process handoff for canonical symbols and malformed/absent sidecar behavior. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-INFILE-CHANINP` | `SC-INFILE-CHANINP-001`. | Namespace separation from `chan.inp` `cbase`. | `[DIRECT][Static]` |
| `REF-GWBASEFLOW-OFEROUTE` | `SC-OFEROUTE-001#INV-OFEROUTE-012`. | Active-router boundary: non-exfiltrated subsurface/baseflow export must not be silently lost or double counted. | `[DIRECT][Static]` |

## Variables And Units

| Symbol | Meaning | Units | Domain | Owner / provenance |
|---|---|---|---|---|
| `lr_bf` | linear-reservoir baseflow branch flag | flag (`0` or `1`) | exactly `0` or `1` | `SC-INFILE-GWCOEFF-001`, `REF-GWBASEFLOW-LEGACY-MAIN` |
| `igwstrd` | initial groundwater storage depth | `mm` | finite, `>= 0` | `SC-INFILE-GWCOEFF-001`, `REF-GWBASEFLOW-LEGACY-CCHRT1` |
| `bfcoeff` / `kb` | daily baseflow coefficient | `d^-1` | finite, `>= 0` | Srivastava linear-reservoir authority, `REF-GWBASEFLOW-LEGACY-CCHRT1` |
| `dscoeff` / `ks` | daily deep-seepage coefficient | `d^-1` | finite, `>= 0` | Srivastava linear-reservoir authority, `REF-GWBASEFLOW-LEGACY-CCHRT1` |
| `bftharea` | watershed/channel baseflow threshold area | `ha` | finite, `>= 0` | `SC-INFILE-GWCOEFF-001`, `REF-GWBASEFLOW-LEGACY-CHANNEL` |
| `D_i` / `gwstrv2` | groundwater recharge volume over the current daily timestep | `m^3` | finite, `>= 0` | `SC-PERC-001`/deep-percolation producer plus `REF-GWBASEFLOW-LEGACY-CONTIN` |
| `S_i` / `gwstrv3` | groundwater reservoir storage after recharge/debit recurrence | `m^3` | finite, `>= 0` | `REF-GWBASEFLOW-SRIVASTAVA-DISS-CH2`, `REF-GWBASEFLOW-LEGACY-CONTIN` |
| `Qb_i` / `gwbfv` | groundwater reservoir baseflow volume over the current daily timestep | `m^3` | finite, `>= 0` | `REF-GWBASEFLOW-SRIVASTAVA-DISS-CH2`, `REF-GWBASEFLOW-LEGACY-CONTIN` |
| `Qs_i` / `gwdsv` | deep-seepage volume out of the groundwater reservoir over the current daily timestep | `m^3` | finite, `>= 0` | `REF-GWBASEFLOW-SRIVASTAVA-DISS-CH2`, `REF-GWBASEFLOW-LEGACY-CONTIN` |
| `tmpgwbfv` | watershed-driver temporary baseflow volume over the current daily timestep | `m^3` | same as `gwbfv` | `REF-GWBASEFLOW-LEGACY-PASS` |
| `tmpgwdsv` | watershed-driver temporary deep-seepage volume over the current daily timestep | `m^3` | same as `gwdsv` | `REF-GWBASEFLOW-LEGACY-PASS` |
| `cbase` | `chan.inp` unit-area channel baseflow coefficient | `m^3 s^-1 m^-2` | out of this contract's process scope | `SC-INFILE-CHANINP-001`; namespace-separated |
| `latqcc` | lateral subsurface export | `mm` publication depth unless owning contract says otherwise | out of this contract's process scope | `SC-SUBHYD-001`; namespace-separated |

## Algorithm State Surfaces

| Surface | Required inputs | Required outputs | Mutated state |
|---|---|---|---|
| Parser/process handoff | `lr_bf`, `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea` from `SC-INFILE-GWCOEFF-001` | groundwater process mode | none; parsed coefficients are immutable after parser finalization |
| Daily groundwater recurrence | day/hillslope deep-percolation recharge `D_i`, prior storage `S_{i-1}`, prior `Qb_{i-1}`, prior `Qs_{i-1}` | `S_i`, `Qb_i`, `Qs_i` | groundwater storage carry |
| Hillslope pass / HBP handoff | `Qb_i`, `Qs_i`, `lr_bf` | pass fields or explicit absent/disabled state | pass inventory only |
| Watershed/channel consumption | `tmpgwbfv`, topology side/top hillslope mapping, `bftharea`, `lr_bf` | channel inflow contribution, `qBase`, diagnostics | channel baseflow carry arrays |
| Publication | generated baseflow/deep seepage, runtime branch state | water-balance rows with no duplication; run-level initial storage, terminal `S_N`, and terminal `Qb_N`/`Qs_N` recurrence operands | publication only |

## Algorithm Specification

### Branch Selection

1. If `gwcoeff.txt` is absent, `SC-INFILE-GWCOEFF-001` emits the explicit
   missing branch and `lr_bf=0`. The groundwater reservoir process is disabled.
   Runtime must not synthesize `igwstrd`, `bfcoeff`, `dscoeff`, or `bftharea`
   defaults.
2. If `gwcoeff.txt` is present and parsed successfully, `lr_bf=1` and the
   normalized coefficients become immutable inputs to this contract.
3. If `gwcoeff.txt` is present but malformed, out of domain, mixed across Lane D
   lanes, or unavailable to a consumer that requires it, runtime must fail
   closed before groundwater/baseflow outputs are produced.

### Daily Linear-Reservoir Recurrence

For each hillslope day under `lr_bf=1`:

1. Convert initial groundwater storage depth to volume for the hillslope area:

   `S_0 = (igwstrd / 1000) * hillslope_width_m * hillslope_length_m`

2. Accumulate daily recharge from deep percolation over OFEs:

   `D_i = sum_o(sep_o_m * width_o_m * length_o_m)`

3. Compute current storage from prior storage plus current recharge minus prior
   outflows:

   `S_i = S_{i-1} + D_i - Qb_{i-1} - Qs_{i-1}`

4. Compute current day exports:

   `Qb_i = bfcoeff * S_i`

   `Qs_i = dscoeff * S_i`

5. Carry `S_i`, `Qb_i`, and `Qs_i` for the next day and export `Qb_i`/`Qs_i`
   to the hillslope pass/watershed handoff as daily timestep volumes in `m^3`.
   Channel consumers that need a flow rate perform the single conversion from
   `m^3` per daily timestep to `m^3 s^-1` by dividing by `86400 s d^-1`.

For `lr_bf=0`, runtime emits an explicit disabled state and `Qb_i = Qs_i = 0`
for this reservoir process. The `cbase` channel branch remains a separate
channel-routing behavior and is not a groundwater-reservoir default.

### Watershed And Channel Consumption

1. When `lr_bf=0`, watershed/channel routing may consume `chan.inp` `cbase`
   according to `SC-INFILE-CHANINP-001` and `SC-ROUTE-001`; generated
   groundwater reservoir baseflow is absent.
2. When `lr_bf=1`, watershed/channel routing consumes generated
   `tmpgwbfv`/`gwbfv` volumes from side and top hillslopes instead of `cbase`.
3. The `bftharea` branch is evaluated against watershed contributing area in
   hectares. Contributions below the threshold follow the legacy ephemeral
   carry/suppression branch and must not be silently converted to `cbase`.
4. Channel water-balance/publication must not double count generated baseflow.
   If legacy `watbalPrint` surface semantics are retained, the `Baseflow` column
   is zero under `lr_bf=1` because generated groundwater baseflow is carried in
   runoff/streamflow surfaces.

### Run-Level Recurrence Publication

When `lr_bf=1`, run execution provenance MUST publish the authoritative first
day `storage_before_m3` (`S_0`), final day `storage_after_m3` (`S_N`), final
day `baseflow_m3` (`Qb_N`), and final day `deep_seepage_m3` (`Qs_N`) alongside
the cumulative recharge and export totals. These are existing recurrence
operands, not a second groundwater state or a post-export surrogate. They make
both exact timing identities independently reconstructable:

`S_N = S_0 + sum(D_i) - (sum(Qb_i) - Qb_N) - (sum(Qs_i) - Qs_N)`

`S_N - Qb_N - Qs_N = S_0 + sum(D_i) - sum(Qb_i) - sum(Qs_i)`

When the reservoir branch is disabled, these run-level storage/terminal-export
operands MUST remain explicitly absent rather than zero-filled. Daily HBP/pass
schema and downstream flow consumption are unchanged by this observability
surface.

### Lane D MOFE Boundary

Lane D active surface routing does not consume groundwater reservoir baseflow as
a surface source. M-T2B must export non-exfiltrated groundwater/baseflow and
deep seepage alongside the active surface-water ledger, and must prove the
following:

1. deep-percolation recharge is not routed as surface runoff;
2. `ui_SCrunf` / return-flow exfiltration remains the only subsurface-to-surface
   active-router source term under `SC-OFEROUTE-001`;
3. `latqcc` lateral export remains `SC-SUBHYD-001` owned and is not collapsed
   into `Qb_i`;
4. missing or mixed `gwcoeff` authority across active MOFE lanes fails closed;
5. the hillslope water-balance ledger includes surface outflow, lateral export,
   groundwater baseflow export, deep seepage, ET, and storage change without
   double subtraction.

## Branch And Guard Table

| Branch / trigger | Guard class | Required behavior | Failure behavior |
|---|---|---|---|
| `lr_bf=0` explicit missing branch | runtime + governance | reservoir process disabled; `Qb_i=Qs_i=0`; no coefficient defaults inferred | typed missing-optional observability; no model-state emission for coefficients |
| `lr_bf=1` parsed branch | runtime | coefficients finite and in domain; recurrence executes | typed `GWBASEFLOW-E-001` on missing/malformed/domain failure |
| coefficient domain | runtime | `igwstrd>=0`, `bfcoeff>=0`, `dscoeff>=0`, `bftharea>=0`; no parser/baseline upper-bound guard is inferred | typed `GWBASEFLOW-E-002` |
| storage recurrence | runtime | `S_i`, `Qb_i`, and `Qs_i` finite/non-negative; exports cannot exceed accepted storage without explicit recharge/carry evidence | typed `GWBASEFLOW-E-003` |
| `bftharea` threshold | runtime + test | threshold evaluated in hectares against watershed area | typed `GWBASEFLOW-E-004` or hold if topology area unavailable |
| `cbase` namespace | runtime + governance | `cbase` only in `lr_bf=0` channel branch; never used as `bfcoeff` | typed `GWBASEFLOW-E-005` |
| `latqcc` namespace | runtime + governance | lateral export remains separate from groundwater reservoir baseflow | typed `GWBASEFLOW-E-006` |
| Lane D mixed authority | runtime | all lanes in a coupled active hillslope have consistent `gwcoeff` process authority or all are explicitly disabled | typed `GWBASEFLOW-E-007` |
| pass/HBP consumer | runtime + test | generated `gwbfv`/`gwdsv` read by downstream consumer when claimed | typed `GWBASEFLOW-E-008`; no producer-only closure |

## Invariants And Invariant Guard Map

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-GWBASEFLOW-001` | Groundwater reservoir baseflow is a distinct process from lateral subsurface export (`latqcc`) and `chan.inp` unit-area baseflow (`cbase`). | `REF-GWBASEFLOW-SRIVASTAVA-DISS-CH2`, `REF-GWBASEFLOW-INFILE-CHANINP`, `SC-SUBHYD-001` | `[DIRECT][Static] + [INFERENCE][Static]` | namespace guard, alias map, implementation tests | typed hard fail / package `HOLD` on conflation |
| `INV-GWBASEFLOW-002` | Successful `gwcoeff.txt` parse is the only authority that enables the linear-reservoir branch (`lr_bf=1`). Missing sidecar disables this reservoir process without inferred coefficient defaults. | `REF-GWBASEFLOW-LEGACY-MAIN`, `REF-GWBASEFLOW-INFILE-GWCOEFF` | `[DIRECT][Static]` | parser/process handoff guard | typed hard fail for malformed present file; explicit disabled state for absence |
| `INV-GWBASEFLOW-003` | Daily recharge to the groundwater reservoir is the volume form of WEPP deep percolation accumulated over the hillslope day. | `REF-GWBASEFLOW-SRIVASTAVA-DISS-CH2`, `REF-GWBASEFLOW-LEGACY-CONTIN`, `REF-GWBASEFLOW-DUN-2009` | `[DIRECT][Static] + [INFERENCE][Static]` | recharge lineage test | typed hard fail / `HOLD` if recharge producer lineage is unavailable |
| `INV-GWBASEFLOW-004` | The accepted linear-reservoir recurrence is `S_i = S_{i-1} + D_i - Qb_{i-1} - Qs_{i-1}`, `Qb_i = bfcoeff * S_i`, `Qs_i = dscoeff * S_i`. | `REF-GWBASEFLOW-SRIVASTAVA-DISS-CH2`, `REF-GWBASEFLOW-LEGACY-CONTIN` | `[DIRECT][Static]` | contract-derived recurrence vectors | typed hard fail on non-finite or negative storage/export |
| `INV-GWBASEFLOW-005` | Generated baseflow and deep seepage must be exported to pass/watershed consumers when `lr_bf=1`; producer-only evidence is not enough for watershed/baseflow closure. | `REF-GWBASEFLOW-LEGACY-PASS`, `REF-GWBASEFLOW-LEGACY-CHANNEL` | `[DIRECT][Static]` | pass/HBP and watershed consumer proof | package `HOLD` until real consumer reads generated fields |
| `INV-GWBASEFLOW-006` | `bftharea` is evaluated as hectares of watershed contributing area and controls the legacy ephemeral/perennial baseflow branch. | `REF-GWBASEFLOW-LEGACY-CHANNEL`, `REF-GWBASEFLOW-INFILE-GWCOEFF` | `[DIRECT][Static]` | topology-area threshold fixture | typed hard fail or `HOLD` if area lineage is missing |
| `INV-GWBASEFLOW-007` | Lane D active routing must conserve and export non-exfiltrated groundwater/baseflow/deep-seepage terms without feeding them as surface-router source terms or double counting them against `latqcc`. | `REF-GWBASEFLOW-OFEROUTE`, `REF-GWBASEFLOW-LEGACY-CONTIN`, `SC-SUBHYD-001` | `[DIRECT][Static] + [INFERENCE][Static]` | active-mode ledger vector and mixed-authority guard | production activation `HOLD` until fixture-proven |
| `INV-GWBASEFLOW-008` | Publication surfaces must label generated groundwater baseflow and deep seepage so consumers can distinguish true zero, disabled process, and generated-but-carried-in-runoff legacy water-balance behavior. For an enabled run, execution provenance must additionally expose `S_0`, `S_N`, `Qb_N`, and `Qs_N` with cumulative recharge/exports so both recurrence timing identities can be independently reconstructed; disabled runs leave these operands absent, never zero-filled. | `REF-GWBASEFLOW-LEGACY-WATBAL`, `REF-GWBASEFLOW-LEGACY-PASS`, `INV-GWBASEFLOW-004` | `[DIRECT][Static] + [INFERENCE][Static]` | output metadata / publication lineage gate and real H2637 run-level reconstruction | typed hard fail or publication `HOLD` on ambiguous zero-fill or missing recurrence operands |

## Producer And Consumer Obligations

| Obligation ID | Surface | Obligation | Gate |
|---|---|---|---|
| `OBL-GWBASEFLOW-P-001` | parser handoff | consume `SC-INFILE-GWCOEFF-001` state exactly; do not infer missing coefficients | parser-derived tests |
| `OBL-GWBASEFLOW-P-002` | single-OFE recurrence | implement the daily vector recurrence and storage carry with domain guards | recurrence unit tests |
| `OBL-GWBASEFLOW-P-003` | MOFE aggregation | aggregate deep-percolation recharge per hillslope/OFE lane without losing lane provenance | Lane D MOFE accounting vector |
| `OBL-GWBASEFLOW-P-004` | pass/HBP export | publish `gwbfv`/`gwdsv` or explicit disabled/null authority to the downstream consumer path | producer + real consumer proof |
| `OBL-GWBASEFLOW-C-001` | watershed/channel | consume generated `tmpgwbfv` when `lr_bf=1`; consume `cbase` only in its own branch | watershed fixture with branch separation |
| `OBL-GWBASEFLOW-C-002` | active router | keep groundwater/baseflow export off the Lane D surface source series and inside the water-balance/export ledger | active ledger closure vector |
| `OBL-GWBASEFLOW-C-003` | publication | reject zero-fill aliases; expose generated/disabled/legacy-carried states distinctly; publish enabled-run `S_0`, `S_N`, `Qb_N`, and `Qs_N` recurrence operands in execution provenance | publication metadata and independent run-level reconstruction gate |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `lr_bf` | `groundwater.linear_reservoir_enabled` | parser/process handoff | flag; same branch value | `SC-INFILE-GWCOEFF-001` / `SC-GWBASEFLOW-001` |
| `igwstrd` | `groundwater.initial_storage_depth_mm` | parser/process handoff | same unit, `mm` | `SC-INFILE-GWCOEFF-001` / `SC-GWBASEFLOW-001` |
| `bfcoeff` | `groundwater.baseflow_coeff_per_day` | parser/process handoff | same unit, `d^-1` | `SC-INFILE-GWCOEFF-001` / `SC-GWBASEFLOW-001` |
| `dscoeff` | `groundwater.deep_seepage_coeff_per_day` | parser/process handoff | same unit, `d^-1` | `SC-INFILE-GWCOEFF-001` / `SC-GWBASEFLOW-001` |
| `bftharea` | `groundwater.baseflow_threshold_area_ha` | parser/process handoff and channel threshold | same unit, `ha` | `SC-INFILE-GWCOEFF-001` / `SC-GWBASEFLOW-001` |
| `S_i` / `gwstrv3` | `groundwater.storage_m3` | runtime state | same unit, `m^3`; registry gap | `SC-GWBASEFLOW-001` |
| `D_i` / `gwstrv2` | `groundwater.recharge_m3` | runtime state | same unit, `m^3`; daily timestep volume; registry gap | `SC-GWBASEFLOW-001` |
| `Qb_i` / `gwbfv` | `groundwater.baseflow_volume_m3` | pass/HBP/watershed | same unit, `m^3`; daily timestep volume; registry gap | `SC-GWBASEFLOW-001` |
| `Qs_i` / `gwdsv` | `groundwater.deep_seepage_volume_m3` | pass/HBP/watershed | same unit, `m^3`; daily timestep volume; registry gap | `SC-GWBASEFLOW-001` |
| `cbase` | `unit_area_baseflow_coefficient` | `chan.inp` channel branch | not same symbol; `m^3 s^-1 m^-2`; namespace guard required | `SC-INFILE-CHANINP-001` |
| `latqcc` | `hillslope_wat.latqcc` | lateral subsurface export publication | not same symbol; owned publication unit `mm` | `SC-SUBHYD-001` |

## Constants And Parameters

| Parameter | Units | Authority | Notes |
|---|---|---|---|
| `1000 mm m^-1` | conversion | unit-governance depth conversion | required for `igwstrd` depth to volume and publication depth conversions; implementation must use named helper or recorded exception |
| `86400 s d^-1` | conversion | channel routing legacy code | required when generated baseflow volume is converted to daily flow rate in channel consumers |
| `10000 m^2 ha^-1` | conversion | `bftharea` threshold code | required for watershed area-to-hectare threshold comparison |
| `bfcoeff` | `d^-1` | `gwcoeff.txt` / Srivastava | no inferred defaults |
| `dscoeff` | `d^-1` | `gwcoeff.txt` / Srivastava | current openWEPP authority excludes negative upward-recharge coefficients |

## Unit-Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `igwstrd` | `mm` | registry gap for `groundwater.initial_storage_depth_mm` | `mm_to_m` then `depth_times_area_to_volume` required | none | not published directly |
| `bfcoeff` | `d^-1` | registry gap for `groundwater.baseflow_coeff_per_day` | none | dimensioned rate coefficient scalar exception until typed coefficient wrapper exists | not published directly |
| `dscoeff` | `d^-1` | registry gap for `groundwater.deep_seepage_coeff_per_day` | none | dimensioned rate coefficient scalar exception until typed coefficient wrapper exists | not published directly |
| `bftharea` | `ha` | registry gap for `groundwater.baseflow_threshold_area_ha` | `ha_to_m2` or `m2_to_ha` required | none | not published directly |
| `D_i`, `S_i`, `Qb_i`, `Qs_i` | `m^3` | registry gaps for runtime/pass symbols | depth/area helpers required; volume-to-flow conversion only at channel consumers | none | M-T2B must register output/pass metadata before publication closure |
| `latqcc` | `mm` | registered under `SC-SUBHYD-001` | `mm_to_m3(area)` only for closure operands | none | owned by `SC-SUBHYD-001` |
| `cbase` | `m^3 s^-1 m^-2` | parser/routing registry gap | none in this contract | not this contract's scalar | owned by `SC-INFILE-CHANINP-001` / routing consumer |

## Tolerance And Numeric Notes

- Contract-derived recurrence tests should use exact arithmetic where possible
  and relative/absolute tolerances only for floating-point multiplication and
  conversion roundoff.
- Storage and export values have zero lower bounds. Negative values outside a
  small roundoff envelope are domain violations.
- Current openWEPP authority does not permit negative `dscoeff` even though
  later literature discusses upward lower-aquifer exchange. Supporting that
  process requires a separate contract amendment and parser-domain change.
- No coefficient upper bound is inferred from `SC-INFILE-GWCOEFF-001` or the
  pinned baseline parser. M-T2B must instead fail closed when the recurrence
  would produce non-finite state or exports that exceed accepted storage without
  explicit recharge/carry evidence.
- Legacy Fortran carries some reservoir variables as shared common-block scalars.
  M-T2B must prove the openWEPP port's per-hillslope storage carry explicitly;
  if baseline-compatible multi-hillslope behavior cannot be established, the
  implementation package must hold before production activation.

## Test-Vector Obligations

| Obligation ID | Scenario | Required checks |
|---|---|---|
| `TV-GWBASEFLOW-001` | disabled branch (`lr_bf=0`) | no coefficient defaults; generated `Qb_i`/`Qs_i` zero; `cbase` branch remains separate |
| `TV-GWBASEFLOW-002` | one-hillslope recurrence | for known area, storage, recharge, `bfcoeff`, and `dscoeff`, reconstruct `S_i`, `Qb_i`, `Qs_i` over at least two days |
| `TV-GWBASEFLOW-003` | domain failures | negative/non-finite storage, coefficients, threshold area, and outflow-over-storage cases fail closed |
| `TV-GWBASEFLOW-004` | pass/HBP export | generated `gwbfv`/`gwdsv` read by the downstream pass/watershed consumer; producer-only evidence rejected |
| `TV-GWBASEFLOW-005` | threshold branch | fixture separates below-`bftharea` and above-`bftharea` channel behavior |
| `TV-GWBASEFLOW-006` | namespace separation | fixture distinguishes generated `Qb_i`, lateral `latqcc`, and `cbase`; wrong aliases must fail |
| `TV-GWBASEFLOW-007` | Lane D active MOFE ledger | active-router source excludes groundwater baseflow; ledger exports groundwater/baseflow/deep seepage and closes water balance |
| `TV-GWBASEFLOW-008` | publication anti-alias and run closure | public outputs distinguish generated zero, disabled branch, missing authority, and legacy-carried baseflow; a real H2637 run reconstructs both run-level recurrence timing identities from published `S_0`, `S_N`, `Qb_N`, `Qs_N`, and cumulative totals |

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `GWBASEFLOW-MT2A-AUTHORITY` | `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/` | `active` | `maps-to-existing-INV` | `INV-GWBASEFLOW-001, INV-GWBASEFLOW-002, INV-GWBASEFLOW-003, INV-GWBASEFLOW-004, INV-GWBASEFLOW-005, INV-GWBASEFLOW-006, INV-GWBASEFLOW-007, INV-GWBASEFLOW-008` | `none` | M-T2A package-local authority is promoted into this contract; package artifacts remain evidence only. |

## Gap Register And Promotability Labels

| Gap ID | Statement | Promotion impact | Disposition |
|---|---|---|---|
| `GAP-GWBASEFLOW-001` | Runtime boundary-symbol registry entries for groundwater storage/recharge/baseflow/deep-seepage surfaces are not implemented. | Blocks implementation/publication closure, not this authority draft. | M-T2B must add or explicitly gap registry entries before runtime publication. |
| `GAP-GWBASEFLOW-002` | Multi-hillslope storage carry must be proven against pinned baseline behavior; legacy common-block scalar carry requires fixture evidence before production activation. | Blocks production implementation closure if unresolved. | M-T2B test-vector obligation `TV-GWBASEFLOW-002`. |
| `GAP-GWBASEFLOW-003` | Nonlinear groundwater/baseflow algorithms from Srivastava et al. (2017) are acknowledged lineage but not current `gwcoeff.txt` authority. | Non-promotable for nonlinear implementation. | Requires separate contract amendment if requested. |
| `GAP-GWBASEFLOW-004` | Existing public water-balance columns may not yet expose generated groundwater baseflow/deep seepage distinctly from legacy-carried zero behavior. | Blocks publication closure. | M-T2B/M-T3 must add metadata and anti-alias evidence. |

## Change Log

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-13` | `0.1.2` | Codex | INTVAL run-level recurrence observability: `INV-GWBASEFLOW-008` and publication obligations now require enabled-run execution provenance to expose authoritative `S_0`, `S_N`, `Qb_N`, and `Qs_N` with cumulative totals, while disabled runs retain absent rather than zero-filled operands. This closes independent H2637 recurrence reconstruction without changing recurrence timing or HBP/pass schemas. |
| `2026-07-09` | `0.1.1` | Codex | M-T2 closure amendment: generated `gwbfv`/`gwdsv` HBP payload fields are consumed as fixed-position scaled volumes under `SC-INFILE-HBP-001` v0.2.2; watershed/channel routing must use generated HBP baseflow under `lr_bf=1`, keep `cbase` exclusive to `lr_bf=0`, and evaluate `bftharea` against contributing area hectares before adding generated baseflow to the channel branch. |
| `2026-07-08` | `0.1.0` | Codex | Initial groundwater/baseflow process authority for M-T2A, binding Srivastava linear-reservoir equations, pinned baseline code maps, parser handoff, Lane D boundary obligations, and M-T2B test-vector handoff. |
