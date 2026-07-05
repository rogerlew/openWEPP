# Increment 2c (ROADMAP §E.3) — Multi-OFE Wave-1 Chaining: Execution Entry Gate

Author: Claude Code, 2026-07-04. Evidence: **Static** at authoring (E.2 runtime
recon + the Increment-2 entry gate §2/§3/§4a). Execution record appends below.
Executor: Claude Code (operator: "merge and proceed with E.3"). Branch:
`erosion-e3-multi-ofe-chaining` (off main `c4ad7832`, E.2 merged).

Authority: [`increment-2-entry-gate.md`](increment-2-entry-gate.md) §2 (legacy
chain), §3 (Wave-1 owns the per-OFE physics; Wave-2 retires to a comparator
flag), **§4a (BINDING input authority: per-OFE particle-size sourcing — the
`prtcmp` per-element default lineage; a single hillslope-global override is
fail-closed rejected; legacy `partsize.dat` `usr_partsize` is a MOFE gap, do
not inherit)**; ADR-0036 (the hourly substrate this chains on); SC-SED-001
`INV-SED-012` + SC-RUNOFFPART-001 `INV-RUNOFFPART-030` (the sediment-coupled
`qin` hold this lifts).

## 1. What E.2 already provides (verified in-runtime)

- The solver + assembly carry the **complete decreasing-flow/inflow limbs**:
  `qin_m2_s`/`strldn` operands, the `qout <= 0` `qshear = qin·rspace` basis
  (`xinflo.for:206`), theta suppression at `qout <= qin`, quantum activation
  `w_h > 0 ∨ qin_h > 0` — all tested (the full-reinfiltration and
  falling-limb quanta deposit). Production supplies `qin_h = 0`, `strldn = 0`
  (erosion.rs daily-state build) — E.3 replaces those two zeros with the
  handoff.
- The hourly substrate: per-lane weights + plan; the downslope lane ordering
  already exists per day (`publish_dynamic_transfer_to_downstream`,
  03_executor.rs — water transfer lane N → N+1 within the day loop), which is
  the same ordering the erosion handoff rides.
- The erosion authority is built **per lane**
  (`DirectProductionTypedLaneSeedAuthority`, 00_builders:560-600 — has
  `execution_lane` context) but the seed builder indexes
  `parsed_soil.ofes.first()` / `slope.ofes.first()` / management
  `FIRST_OFE_INDEX = 1` unconditionally — the per-OFE generalization point.

## 2. Design decisions

### D1 — Per-lane, per-OFE seeds (§4a input authority)
`direct_production_wave1_operand_seed` gains the lane's OFE index: soil OFE
`ofes[i]`, slope OFE `ofes[i]` (per-OFE `fwidth`, segments, `slplen`,
`avgslp`), management PL projection OFE index `i+1`. **Particle classes derive
from each OFE's own surface soil** (`direct_production_erosion_particle_classes`
gains the OFE index) — per-OFE by construction, satisfying §4a with no
override surface at all (no `partsize.dat` analog exists in openWEPP; if one
is ever added it must be per-OFE keyed or fail-closed, per §4a).

### D2 — The hour-resolved handoff (the two zeros become real operands)
Within a day, after lane `i`'s erosion span solves, publish to lane `i+1`'s
erosion intake (a new lane-frame carry, alongside the water transfer):
- **`qin_h`**: lane `i`'s hourly unit outflow discharge
  `q_out_h = (q_runoff_i · w_h / 3600) · efflen_i` (m²/s) — the same
  discharge basis the solve used (`xinflo` `qin = qout` OFE idiom).
- **`ldtop_h` (→ `strldn_h`)**: lane `i`'s per-hour exported load per unit
  width `G_out_h` (kg/m). The receiving assembly normalizes to `strldn_h =
  G_out_h / (effdrn_h · tcend_{i+1} · width_{i+1} / rspace_{i+1})` — the
  RECEIVING lane's denormalization scale inverted (the `sloss.for:166`
  `dslod` scale), because `strldn` is nondimensional in the RECEIVER's
  normalization (`route.for:136` `load[0] = strldn` under the receiver's
  `param` scaling). **Recon item R2 verifies this against
  `xinflo`/`route`/`param` before wiring** — the normalization basis is the
  highest-risk correctness point of the increment.
- **Exit class fractions**: lane `i`'s exiting composition (D4 rule) for the
  receiver's inflow-blend and (E.4) enrichment lineage.

### D3 — Enable + Wave-2 comparator
The Wave-1 seed enables per lane on multi-OFE hillslopes
(`contributor_ofe_count > 1`) under the same no-tillage scope; Wave-2/EROD14
stops being the multi-OFE publication authority and is retained behind a
comparator flag (Investigation tier) for one window, then deleted (stage 2e).
Publication authority: the LAST lane's Wave-1 surfaces become the hillslope
HBP EVENT basis (exit of the chain); per-lane surfaces feed the per-OFE pass
rows as today.

### D4 — Exit-fraction blend (extends GAP-SED-007, pre-enrichment)
Non-cropland (`fidel = frac`, `param.for:452-458`) exit fractions on a
no-deposition OFE are **exactly**
`(G_in · frcflw_in + G_local · frac_own) / G_out` (the `enrich.for:205-213`
terminal blend with `fidel = frac`); with deposition the same blend is the
labeled un-enriched approximation (proportional depletion), superseded by
E.4. Per-OFE `frac_own` comes from D1's per-OFE classes — the §4a gate's
observable.

### D5 — Hold lift (INV-SED-012 / INV-RUNOFFPART-030)
The erosion `qin` now has lineage to the prior OFE's erosion `qout` + the
sediment/class handoff — the exact acceptance condition of both holds. The
INV-RUNOFFPART-031 interim clamp (erosion `qin` clamped to `qout`) retires:
the decreasing-flow hour is an ordinary deposition solve. Contract work:
SC-SED-001 amendment (multi-OFE chaining invariant + INV-SED-012
disposition + the D4 blend rule extension of GAP-SED-007);
SC-RUNOFFPART-001 amendment (030 hold disposition, 031 clamp retirement).
Manifest: `erod14_qin_sediment_coupled` truthfully publishable on the
Wave-1 chain (naming per the amendment).

## 2a. Recon R2 — RESOLVED (Ran static, 2026-07-04)

**The `strldn` basis is source-pinned and matches D2 exactly.** The inter-OFE
handoff quantity is a **sediment discharge per unit width**:
`sloss.for:333` `qsout = dslod2 / effdrn(iplane)` (the prior OFE's exit,
kg·m⁻¹·s⁻¹), and the RECEIVER normalizes by its own scale at
`param.for:243`: `strldn = qsout · rspace / tcend / width` (all receiver
operands; `width > 0` guard, else 0). The per-hour form
`strldn_h = (G_out_h / 3600 s) · rspace_{i+1} / (tcend_{i+1} · width_{i+1})`
is algebraically identical. Hold criterion 1 is cleared.

**Additional required machinery found (D2 expansion):** `param.for:249-390`
carries the **inter-OFE shear/transport continuity adjustment** (the
`INV-SED-008` Eq. [11.4.x] downslope-variability family): for `iplane > 1`
with `qout > 0 ∧ qin > 0`, the receiving OFE's normalized shear/transport
coefficient polynomials are re-derived so shear and transport capacity are
**continuous across the OFE boundary**, using the PRIOR OFE's end state —
`shrspv` (end shear), `anflst/bnflst/cnflst` (end shear coefficients),
`atclst/btclst/ctclst`, `tcprev`, `ktrprv` — with the documented singular
guards (`sratio`/`tcrati` floors, the zero-slope `qostar` substitution, the
2012 `shrati` overflow cap). **The handoff carry therefore includes the
prior lane's end-shear/transport state**, not only `(qin_h, qsout_h,
fractions)`. Without this block, transport capacity jumps at OFE boundaries
and deposition artifacts alias into every boundary — it is required scope
for 2c-2, not a refinement.

## 2b. Recon R1 + R4 — RESOLVED (Ran static, 2026-07-04)

**R1: lane = OFE 1:1, and per-OFE seeds already exist by construction.**
For `lane_count > 1`, `direct_production_typed_lane_seed_authorities`
(00_builders:432-499) slices the parsed inputs per OFE
(`build_static_per_ofe_lane_slices` → `build_lane_soil_profile` /
`build_lane_slope_profile` / `build_lane_management_output`) and builds each
lane's seed authority from its OWN single-OFE-shaped profiles — so the seed
builder's `.first()` indexing resolves to the lane's own OFE on multi-OFE
runs. **D1 (per-OFE particle classes, per-OFE `fwidth`/segments/erodibility)
is therefore already satisfied by the slicing** — §4a's requirement holds by
construction, and stage 2c-1 collapses into 2c-3's enable. Hold criterion 2
cleared. What actually blocks multi-OFE Wave-1: (a) the enable narrowing
(`contributor_ofe_count == 1`, 00_builders:1228), (b)
`wave2_enabled = contributor_ofe_count > 1` keeps EROD14 the multi-OFE
publication authority (the `compute_r7d6` branch order), (c) the unwired
handoff (`qin_h`/`strldn`/fractions zeros), (d) the un-ported
`param.for:249-390` continuity block.

**R4: the authority switch points** are `wave2_enabled` at 00_builders:1219
(+ the erod14 lane flag at :348) and the `compute_r7d6` publication branch
(Wave-2 first). The HBP event row selection (`hbp_sediment_row`) is
lane-agnostic today; under the chain the hillslope EVENT basis must be the
**exit-lane** surfaces (the chain's last OFE), which is what the
max-sediment-row heuristic no longer guarantees on multi-OFE — 2c-3 pins the
EVENT row to the exit lane's rows.

## 2c. Continuity-block port design (pinned)

- **Prior-state derivation is receiver-side** (`param.for:184-196`):
  `qtop = qin·rspace_i`; `shrtp1 = sheart(qtop, slpend_{i-1})`;
  `shrspv = sheart(qtop, cnslp_{i-1})`; `tcprev = trcoef(shrtp1)·shrspv^1.5`;
  `ktrprv = trcoef((shrtp1+shrspv)/2)/trcoef(shrtp1)` — only the prior
  lane's STATIC slopes plus `qin` are needed for these; the ported
  `erosion_shears(q, slope, width, rspace, grow=false, …)` is the `sheart`
  equivalent (no width growth, receiver friction/width context).
- **Carried prior state** (Fortran `save` in `param.for`): the prior OFE's
  final shear/transport coefficient values (`anflst/bnflst/cnflst`,
  `atclst/btclst/ctclst` — the last segment's xinflo/continuity-adjusted
  coefficients, set at `param.for:368-374` and the `qin <= 0` do-20 reset).
- **Wiring shape:** the ASSEMBLY computes `shrspv/shrtp1/tcprev/ktrprv`
  (it owns the hydraulics context) and passes them + the carried prior
  coefficient sets into `DirectWave1ContinuityInputs` as an
  `Option<Wave1InterOfeContinuity>`; the solver applies the
  `param.for:249-390` coefficient rewrite (with every documented singular
  guard: `sratio`/`tcrati` 1e-5 floors, zero-slope `qostar` substitution,
  the 2012 `shrati <= 1e12` cap, the ±0.001 denominator floors) between
  `xinflo` and `route` when the option is present. Per-hour quanta each
  carry their own hour's `qin_h`-derived continuity operands.

## 3. Stage plan

- **2c-0 recon completion (FIRST):** R1 lane↔OFE index mapping (how
  `execution_lane` indexes soil/slope/management surfaces; multi-OFE lane
  construction); R2 the legacy inter-OFE load normalization
  (`route.for:130-160` + `param.for` inflow scaling — pin `strldn`'s exact
  basis); R3 the W7DC01 substrate + an in-repo multi-OFE fixture (the WS3
  matrix is single-OFE; check dff_ws2/MOFE fixtures for a multi-OFE
  soil-contrasting candidate or craft one); R4 where EROD14 currently
  publishes so the authority switch is surgical.
- **2c-1:** per-lane per-OFE seeds (D1) behind the still-single-OFE enable —
  byte-stable (single-OFE lanes index OFE 0 as today).
- **2c-2:** the handoff carry (D2) + receiving-side assembly wiring
  (`qin_h`/`strldn_h`/fractions), still disabled for multi-OFE publication —
  shadow-solvable.
- **2c-3:** multi-OFE enable + publication authority switch + Wave-2
  comparator flag (D3) + D4 blend + hold-lift wiring (D5) + contracts.
- **2c-4:** gates — per-OFE mass closure each lane-day; the OFE-boundary
  handoff identity `G_out(i) = G_in(i+1)` (per hour); hillslope-exit
  closure; the §4a soil-contrasting-OFE fixture (per-OFE `sedcon`/exit
  fractions differ by OFE soil); the multi-OFE directional law; W7DC01
  substrate proof (multi-OFE HBP sediment nonzero, minor-1). Full AGENTS
  battery; push for Codex review.

## 5. Execution record (in progress, branch `erosion-e3-multi-ofe-chaining`)

- **2c-0 recon: COMPLETE** (§2a/§2b — R1/R2/R4 resolved, both hold
  criteria cleared; R3 fixture selection rides 2c-4).
- **2c-2: COMPLETE, three commits, 260/260 + clippy clean each:**
  - `5ecc1d90` — the INV-SED-008 continuity rewrite in the solver
    (`Wave1InterOfeContinuity`, applied between `xinflo` and `route`
    behind the legacy triple guard; solve-final coefficient sets exposed
    on the state).
  - `331ef169` — `Wave1InflowOperands` (the RAW handoff) + receiver-side
    derivations in the assembly (`strldn` per `param.for:243` after own
    hydraulics/transport; `shrspv/shrtp1` via the new `erosion_sheart`
    no-growth producer — a width-growing first cut was caught and
    corrected pre-commit; `tcprev`/`ktrprv` via `erosion_trcoef`).
  - `f34201ba` — `DirectErosionInflowIntake` carry (publisher in the
    executor beside the water transfer; clone-at-seed / clear-at-commit
    per-day lifecycle; plan-builder consumption incl. inflow-active
    hours on locally-dry days). Single-OFE byte-identical throughout.

## 5a. Execution record — 2c-3 + 2c-4 (COMPLETE, 2026-07-04)

- **2c-3a (`1feac488`):** enable de-narrowed (every no-tillage lane);
  `wave2_enabled = false` (EROD14 retired to a test-reachable comparator
  arm until stage 2e); D5 manifest lift
  (`wave1-hourly-sediment-coupled-handoff`,
  `erod14_qin_sediment_coupled = true`, both wave2 provenance fields
  false, the water-transfer-only warning retired). **Ran:** first real
  multi-OFE run (W7DC01 substrate p102, 2 OFEs) end-to-end — the FIRST
  multi-OFE sediment the runtime has ever produced. Three production
  findings fixed en route: (1) no-rainfall-excess flow hours
  theta-suppressed (`reid.for` basis — zero `effdrr` hit the
  non-suppressed validator); (2) inflow-only exit days: day
  toe-concentration denominator guarded to a defined 0; (3)
  stiff-quantum flux-consistency refusals skip that quantum's sediment
  with the surfaced `flux_refused_quanta` count (1 in 5,155 lane-days;
  the 1e-9 mass law stays hard).
- **2c-3b (`d3e727d0`):** the chain EVENT — EXIT-scoped with
  CHAIN-AGGREGATED `tdet`/`tdep` (per-day lane sums in the streaming
  summary), one intake rule for single- and multi-OFE shards; D4
  inflow-day exit-fraction blend (labeled GAP-SED-007 extension);
  contracts SC-SED-001 rev 44 (`INV-SED-016`, `INV-SED-012`
  DISPOSITION, blend extension, QSOUT/REID anchors), SC-RUNOFFPART-001
  rev 45 (030 SATISFIED on the chain; 031 erosion-side interim-clamp
  scope superseded), SC-INFILE-HBP-001 0.2.1 (§8.5 chain form).
  **Ran:** p102 event closes at rel 1.8e-13.
- **2c-4:** in-repo fixture `tests/fixtures/erosion_multi_ofe_p102/`
  (real W7DC01 2-OFE hillslope, climate truncated to 10 years) +
  `erosion_multi_ofe_p102_chain` integration test (multi-OFE sediment
  proof incl. material outlet deposition; chain-form `Σ S_h` and
  `Σ V_h` closures; manifest disposition surfaces; the §4a observable
  via an in-test OFE-2 texture coarsening that must move the exit
  composition). One further production finding: full-deposition days
  carry ±1e-13 chain-export accumulation dust — the EVENT capture
  qualifies on MATERIAL export (`> 1e-9 ×` the day's own mass scale,
  TOL-SED-005 basis), so a numerically-zero-export day is never
  serialized as the routed event; an env-gated capture debug probe
  (`OPENWEPP_DEBUG_HBP_CAPTURE`) remains for diagnosis.

## 6. 2c-3 continuation spec (superseded by §5a — retained for the design record)

1. **Enable:** 00_builders:1257 drops `contributor_ofe_count == 1` (the
   no-tillage narrowing stays). Wave-2 retires as authority:
   `wave2_enabled = false` at :1219 (code retained as the comparator arm
   reachable from tests — the INV-SED-015 pattern; deletion = stage 2e).
   Check the knock-ons: the erosion-activation day gate (00c:984), the
   `erod14_wave2_enabled` manifest flag (:348), and `compute_r7d6`'s
   `!wave2_enabled` condition on the hourly publication fields.
2. **Hillslope EVENT on multi-OFE:** the HBP event = the EXIT lane's
   `V_h`/`S_h`/per-class surfaces + CHAIN-AGGREGATED `tdet`/`tdep`
   (Σ lanes, same day). The intake closure generalizes to the chain form
   `Σ S_h(exit) = Σ_lanes(tdet − tdep)` (the per-lane inflows telescope
   out) — CLIWAT-E-047 + SC-INFILE-HBP-001 §8.5 amend accordingly.
3. **D4 blend** in the publication projection (inflow-weighted exit
   fractions; per-OFE `frac_own` from the sliced seeds).
4. **Contracts:** SC-SED-001 (multi-OFE chaining invariant, INV-SED-012
   disposition, GAP-SED-007 blend extension); SC-RUNOFFPART-001 (030
   hold disposition + 031 clamp retirement).
5. **2c-4 gates:** soil-contrasting-OFE fixture (§4a observable);
   per-OFE + handoff-identity + hillslope-exit conservation; W7DC01
   substrate proof; full battery; push for Codex review.

## 7. Hold criteria

1. R2 cannot pin the legacy `strldn` normalization basis unambiguously —
   stop and present the candidates (the receiver-scale inversion must be
   source-grounded, not inferred).
2. The lane↔OFE mapping is not 1:1 (any aggregation between OFEs and lanes)
   — the handoff design assumes lane = OFE (INV-RUNOFFPART-029 lane-state
   lineage); a mismatch is a design stop.
3. Per-OFE + handoff conservation gates fail materially on the real
   multi-OFE substrate.
