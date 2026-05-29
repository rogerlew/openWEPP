# WB16 Peak-Flow Implementation Audit — 2026-05-29

Status: Draft
Last updated: 2026-05-29
Evidence mode: Static
Scope: openWEPP's WB16 peak-runoff closure-diagnostics surface — kernel arithmetic, required-symbol provenance, contract authority, and cross-reference against the wepp-forest baseline `appmth` lineage. Out of scope: numerical-parity diff, runtime execution, watershed-side WS-route peak surfaces, channel-routing peaks.

## 1. Purpose

After HILLSTAB03 (2026-05-28, `SC-WATBAL-001` rev 39) and HILLSTAB06 (2026-05-29, rev 40) tightened the WB16 branch authority and near-zero domain, the WB16 kernel is the dominant residual blocker for cohort hold-lift (`HKERNEL-WB16-PEAK-E-003` = 1094 in the latest HILLSTAB05 rerun delta). This audit answers a single question: are the **inputs** to WB16 — specifically the Chezy depth-discharge exponent `m` and the depth-discharge coefficient `ealpha` — derived from physical state, or supplied as fixed seed defaults that bypass the upstream science? The branch arithmetic was reviewed during HILLSTAB03; the input-provenance question has not been audited.

## 2. Method

Did:
- Read `SC-WATBAL-001` §"WB16 Peak Runoff Closure-Diagnostics Addendum" at working-tree state (rev 40, [SC-WATBAL-001.md L739-812](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L739)).
- Read the openWEPP WB16 kernel guard + arithmetic block at [03_kernel_support_01_kernel_phases.rs L4080-4256](../../crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs#L4080).
- Read the runner seed path for `efflen`, `ealpha`, `m` at [hillslope/mod.rs L1820-1850](../../crates/openwepp-runner/src/hillslope/mod.rs#L1820).
- `grep -rn 'Wb16ExponentM\|BoundarySymbol::from("m")\|exponent_m\b'` across `crates/**/*.rs` to enumerate writers of the `m` boundary symbol.
- `grep -rn 'BoundarySymbol::from("ealpha")\|Wb16Ealpha'` to enumerate writers of `ealpha`.
- Read the legacy `appmth` consumer at [`appmth.for` L1-130](../../../wepp-forest_260430_baseline/src/appmth.for).
- Read the legacy `m` assignment at [`rdat.for` L88-110](../../../wepp-forest_260430_baseline/src/rdat.for).
- Read the legacy `alpha` derivation at [`rdat.for` L99-110](../../../wepp-forest_260430_baseline/src/rdat.for).
- Read the legacy friction-factor blend at [`frcfac.for` L295-330](../../../wepp-forest_260430_baseline/src/frcfac.for).
- Read the legacy `ealpha` snapshot/cascade at [`irs.for` L405-419, 488-489, 615-616](../../../wepp-forest_260430_baseline/src/irs.for), [`wshirs.for` L260-265](../../../wepp-forest_260430_baseline/src/wshirs.for), and [`eplane.for` L1-95](../../../wepp-forest_260430_baseline/src/eplane.for).
- Read the legacy common-block declarations at [`cprams2.inc` L7-20](../../../wepp-forest_260430_baseline/src/cprams2.inc), [`cconsta.inc` L7](../../../wepp-forest_260430_baseline/src/cconsta.inc), [`cffact.inc` L7](../../../wepp-forest_260430_baseline/src/cffact.inc).
- Cross-checked HILLSTAB03/06 disposition artifacts for the stated remediation surface.

Did not:
- Execute `cargo test`, `cargo check`, `openwepp-cli-hill`, or any kernel invocation.
- Numerically diff openWEPP WB16 output against `appmth.for` output for any fixture.
- Audit the watershed-side WS-route peak surfaces or channel `peakro` consumers.
- Audit `efflen` / `slplen` provenance beyond noting the runner-seed fallback (it is a separate audit surface).
- Survey the `frctrl` / `inrfto` / `fribas` / `frican` chain in legacy beyond confirming `frcteq` is the downstream blend.

## 3. WB16 inventory at HEAD

### 3.1 Contract-declared required inputs

[SC-WATBAL-001.md L745](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L745) declares:

| Surface | Symbols |
|---|---|
| Closure-diagnostics required inputs | `Q`, `timem_####`, `intsty_####`, `efflen`, `ealpha`, `m`, `I`, `irrigation.runtime_rate_m_per_s` |
| Closure-diagnostics peak outputs | `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` |

The contract names `ealpha` and `m` as required inputs but does **not** name an upstream producer, declare a unit (other than implicit nondimensional for `m`, time/length-derived for `ealpha`), or fix a legitimate value range. The HILLSTAB03 amendment ([rev 39](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L1120)) tightened the domain posture to reject `m <= 0`, `ealpha <= 0`, `efflen <= 0`, but did not author `m` or `ealpha` provenance.

### 3.2 openWEPP runtime kernel surface

[03_kernel_support_01_kernel_phases.rs L4102-4122](../../crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs#L4102) reads `ealpha` and `m` from the closure-diagnostics request state surface and applies the contract domain guards:

```
let ealpha = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EALPHA)?;
if ealpha <= WB11_ZERO_THRESHOLD { ... HKERNEL-WB16-PEAK-E-003 ... }

let exponent_m = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EXPONENT_M)?;
if exponent_m <= WB11_ZERO_THRESHOLD { ... HKERNEL-WB16-PEAK-E-003 ... }
```

These values then feed the kinematic-wave arithmetic at [L4216-4256](../../crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs#L4216):

```
let vstar = vave / remax;
let vave_power = vave.powf(exponent_m - 1.0);
let te_base = efflen / (ealpha * vave_power);
let te = te_base.powf(1.0 / exponent_m);
let tstar = te / effdrr;
```

This is a faithful Rust port of [`appmth.for` L78-86](../../../wepp-forest_260430_baseline/src/appmth.for#L78). Branch dispatch downstream (`tstar >= 1` partial-equilibrium, two `vstar < 1` quasi-equilibrium branches, `vstar >= 1` constant-excess) was corrected to baseline shape by HILLSTAB03.

### 3.3 Writers of `m` (`Wb16ExponentM`)

Grep results across `crates/**/*.rs`:

| Site | Action |
|---|---|
| [openwepp-kernel-contract/src/lib.rs L307, L468-469](../../crates/openwepp-kernel-contract/src/lib.rs#L307) | Symbol enum + name binding (`Wb16ExponentM -> "m"`) |
| [constants.rs L288-289](../../crates/openwepp-hillslope-orchestrator/src/constants.rs#L288) | Constant alias `WB16_SYMBOL_EXPONENT_M` |
| [03_kernel_support_01_kernel_phases.rs L4113](../../crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs#L4113) | Reader (`require_state_scalar`) |
| [runner/hillslope/mod.rs L1842-1846](../../crates/openwepp-runner/src/hillslope/mod.rs#L1842) | **Sole writer** — seed fallback inserts `1.5` if symbol is `None` |

Reader is one; writer is one. There is no parser, no kernel, no input-file path that emits `m`.

### 3.4 Writers of `ealpha` (`Wb16Ealpha`)

Grep results across `crates/**/*.rs`:

| Site | Action |
|---|---|
| [openwepp-kernel-contract/src/lib.rs L468](../../crates/openwepp-kernel-contract/src/lib.rs#L468) | Symbol enum + name binding (`Wb16Ealpha -> "ealpha"`) |
| [03_kernel_support_01_kernel_phases.rs L4102](../../crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs#L4102) | Reader (`require_state_scalar`) |
| [runner/hillslope/mod.rs L1837-1841](../../crates/openwepp-runner/src/hillslope/mod.rs#L1837) | **Sole writer** — seed fallback inserts `1.0` if symbol is `None` |

Reader is one; writer is one. No parser, no kernel, no input-file path emits `ealpha`. The same `.is_none()`-guarded seed pattern applies as for `m`.

## 4. wepp-forest baseline lineage

### 4.1 `m` — global Chezy depth-discharge exponent

[`rdat.for` L91-93](../../../wepp-forest_260430_baseline/src/rdat.for#L91):

```
c     depth-discharge exponent
c
      m = 1.5
```

Hard-coded as a single positive constant, set in the input-read routine. Documentation:
- [`appmth.for` L33](../../../wepp-forest_260430_baseline/src/appmth.for#L33): "m — kinematic depth-discharge exponent (nd)"
- [`hdepth.for` L14](../../../wepp-forest_260430_baseline/src/hdepth.for#L14): "m — Chezy depth-discharge exponent"
- [`eplane.for` L52](../../../wepp-forest_260430_baseline/src/eplane.for#L52): "m — 3/2"

`m = 1.5` reflects the wide-shallow Chezy regime where `q = α · h^(3/2)`. Legacy does not vary `m` by OFE, surface cover, or event. **It is structurally constant.** No legacy code path produces `m <= 0`.

### 4.2 `ealpha` — three-stage derivation

**Stage 1 — `alpha(iplane)`, the per-OFE Chezy α at peak.** [`rdat.for` L99-110](../../../wepp-forest_260430_baseline/src/rdat.for#L99):

```
if (conseq(nowcrp,iplane).eq.0) then
  alpha(iplane) = sqrt(avgslp(iplane)*8.0*accgav/frcteq(iplane))
  ...
else
  alpha(iplane) = sqrt(cntslp(conseq(nowcrp,iplane)) * 8.0 *
 1      accgav / frcteq(iplane))
  ...
end if
```

This is `α = √(8 g S / f)`: the wide-channel Chezy α with slope `S` (m/m), gravity `g = 9.807 m/s²` ([`inidat.for` L1054](../../../wepp-forest_260430_baseline/src/inidat.for#L1054), `cconsta.inc` common `consta`), and equilibrium friction factor `f = frcteq(iplane)`. The contour branch substitutes `cntslp` for `avgslp`.

**Stage 2 — `frcteq(iplane)`, the equilibrium Darcy-Weisbach friction blend.** [`frcfac.for` L320-327](../../../wepp-forest_260430_baseline/src/frcfac.for#L320):

```
if(rillar .lt. 1.0) then
  frcteq(iplane) = inrfto + rillar * (frctrl(iplane)-inrfto)
else
  frctrl(iplane) = inrfto
  frcsol(iplane) = inrfso
  frcteq(iplane) = inrfto
endif
```

where:
- `inrfto` is the total interrill friction, an empirical roll-up of residue, basal cover, canopy cover, and rock from [`frcfac.for` L295-306](../../../wepp-forest_260430_baseline/src/frcfac.for#L295) using the `125.91 · cov^0.8` / `38.95 · cancov^0.8` forms.
- `frctrl(iplane)` is the rill friction, computed elsewhere from rill geometry and roughness.
- `rillar = width(iplane) / rspace(iplane)` ([`frcfac.for` L312](../../../wepp-forest_260430_baseline/src/frcfac.for#L312)) is the rill-area fraction. When rills span their full spacing (broad sheet flow), the rill weighting collapses and only interrill friction applies.

So `frcteq` varies per OFE and per day with surface cover, residue, rill geometry, and canopy. It is **not** a constant.

**Stage 3 — projection to `ealpha`.** Two paths:

Single-OFE invocation ([`irs.for` L405-406](../../../wepp-forest_260430_baseline/src/irs.for#L405), L488-489, L615-616; [`wshirs.for` L262-263](../../../wepp-forest_260430_baseline/src/wshirs.for#L262)):

```
alphay(iplane) = alpha(iplane)
ealpha = alphay(iplane)
```

`alphay` is documented as "previous days alpha value" ([`cprams2.inc` L19](../../../wepp-forest_260430_baseline/src/cprams2.inc#L19)) and is the day-snapshot of `alpha`. Initialized to zero in [`inidat.for` L365](../../../wepp-forest_260430_baseline/src/inidat.for#L365); refreshed at each IRS invocation.

Multi-OFE cascade invocation ([`irs.for` L419](../../../wepp-forest_260430_baseline/src/irs.for#L419)):

```
eplane(ibpln, iepln, slplen, alphay, m, ealpha)
```

calls into [`eplane.for` L80-92](../../../wepp-forest_260430_baseline/src/eplane.for#L80), which composes per-OFE `alpha`s into a single equivalent coefficient over the cascade:

```
do 20 i = ibpln, iepln
  cml = cml + slplen(i)
  tmpvr1 = cml ** power3                       ! power3 = (m+1)/m
  sdst = sdst + (tmpvr1-tmpvr2) / (alpha(i)**power2)   ! power2 = 1/m
  tmpvr2 = tmpvr1
20 continue
ealpha = (suml/sdst) ** m * suml
```

This is the **equivalent-plane projection** for serial OFEs: the inverse-α-weighted cumulative-length integral, normalized by total length, raised to `m` and length-scaled.

## 5. Findings

### 5.1 `m` provenance is faithful to legacy but the contract is silent

The runner seed `m = 1.5` ([hillslope/mod.rs L1842-1846](../../crates/openwepp-runner/src/hillslope/mod.rs#L1842)) matches the legacy assignment `m = 1.5` at [`rdat.for` L93](../../../wepp-forest_260430_baseline/src/rdat.for#L93) exactly. Both treat `m` as a global Chezy exponent, not a state-derived value.

However, `SC-WATBAL-001` declares `m` as a required closure-diagnostics input symbol ([L745](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L745)) and applies a `m > 0` domain guard ([L781-785](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L781)), but does **not** declare:
- the canonical value (`1.5`),
- the physical interpretation (Chezy regime, `q = α h^(3/2)`),
- the producer of record,
- the range of regimes (Manning `5/3`, laminar `3`) the symbol may legitimately take.

The reader cannot determine from the contract whether `m = 1.5` is the right value, whether it should vary by cover, or whether a future kernel could legitimately emit `m ≠ 1.5`.

### 5.2 The `m <= 0` domain guard cannot fire in the current cohort

Because the runner seed at [hillslope/mod.rs L1842-1846](../../crates/openwepp-runner/src/hillslope/mod.rs#L1842) is the only writer of the `m` symbol and unconditionally seeds `1.5` whenever the symbol is absent, runtime `m == 1.5` for every WB16 invocation. The HILLSTAB03 amendment that added `m <= 0` as a domain-invalid intermediate ([SC-WATBAL-001.md L783](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L783)) is mathematically correct (the kinematic-wave equations require `m > 0` to avoid division-by-zero in `1/m` and divergence in `vave^(m-1)`) but is **unreachable through the current input pipeline**. The HILLSTAB03 residual reduction (563 → 437 `HKERNEL-WB16-PEAK-E-003`) came from the `vstar >= 1` constant-excess branch and the `tc(vstar)` correction, not from the `m <= 0` clause.

### 5.3 `ealpha` provenance is missing — `1.0` seed is a science gap, not a legacy match

The runner seed `ealpha = 1.0` ([hillslope/mod.rs L1837-1841](../../crates/openwepp-runner/src/hillslope/mod.rs#L1837)) has **no legacy analog.** Legacy `ealpha` is a three-stage derived quantity (per-OFE Chezy α from slope/gravity/friction → daily `alphay` snapshot → optional `eplane` cascade projection), with the friction term itself rolled up from cover, residue, canopy, and rill geometry. Substituting a unitless constant `1.0` for this chain:

| Aspect | Legacy | openWEPP today |
|---|---|---|
| Per-OFE variation | Yes — `α = √(8gS/f)` varies with OFE slope `S` and friction `f` | No — constant `1.0` for every OFE |
| Day-to-day variation | Yes — `frcteq` updates with cover/residue/rill state | No |
| Cascade composition | Yes — `eplane.for` integrates per-OFE α over slope length | No — same `1.0` regardless of OFE count |
| Cover sensitivity | Yes — `inrfto` is roll-up of residue/basal/canopy/rock fractions | None |
| Slope sensitivity | Yes — `α ∝ √S` | None |

**Static back-of-envelope magnitude check** of legacy `ealpha = √(8 g S / f)` with `g = 9.807`:

| Surface | `S` | `f` | Legacy `ealpha` ≈ |
|---|---|---|---|
| Cropland, gentle | 0.05 | 0.5 | 2.8 |
| Cropland, moderate | 0.10 | 0.5 | 4.0 |
| Forested, moderate | 0.15 | 10 | 1.1 |
| Bare soil, steep | 0.25 | 0.3 | 8.1 |
| Heavy residue, gentle | 0.05 | 50 | 0.28 |

The openWEPP default `1.0` sits below typical cropland values and above heavy-residue values. The substitution is silent — the contract domain guard accepts `1.0` because it is positive and finite, and no closure invariant detects the magnitude error.

### 5.4 Downstream effect on WB16 outputs is structural, not random

`te = (efflen / (ealpha · vave^(m-1)))^(1/m)`. With `m = 1.5`, this reduces to `te ∝ (1/ealpha)^(2/3)`. Substituting a constant `ealpha = 1.0` for a derived value with typical magnitude `2.8` (cropland, gentle) gives a `te` that is `2.8^(2/3) ≈ 1.97` times too large, which:

- inflates `tstar = te / effdrr`,
- pushes more events into the `tstar >= 1` partial-equilibrium branch where `qpstar = 1/tstar^m` is small,
- **systematically underestimates `peakro`** for cropland and bare-soil cases,
- pushes `watdur = Q / peakro` upward, biasing toward the 86400 s daily cap declared at [SC-WATBAL-001.md L779-780](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L779).

Direction of the bias depends on whether legacy `ealpha` exceeds or falls below `1.0`. Magnitude is a one-to-one function of the substitution error. The bias is **not** detected by:

- The WB16 domain guard at [03_kernel_support_01_kernel_phases.rs L4103](../../crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs#L4103) — `1.0` is positive and finite.
- The continuity check `watdur = Q / peakro` ([SC-WATBAL-001.md L778](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L778)) — it is preserved tautologically by `watdur = Q / peakro`.
- The branch-selector test vectors at [SC-WATBAL-001.md L796-812](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L796) — they specify `ealpha` directly per vector and verify branch arithmetic, not upstream provenance.
- Legacy-comparator deltas under the ADR-0011 confidence tier — daily/event `peakro` differences would be attributed to "kernel arithmetic" rather than to the missing input pipeline.

### 5.5 `SC-WATBAL-001` contract gap

`SC-WATBAL-001` §WB16 lists `ealpha` as a required closure-diagnostic input but specifies neither its derivation nor its upstream contract authority. The wepp-forest chain `slope (`avgslp` / `cntslp`) → friction (`frctrl`, `inrfto`, `frcteq`) → α (`alpha(iplane)`) → daily snapshot (`alphay`) → cascade projection (`eplane`) → `ealpha`` has no `SC-*` contract anchoring it in the openWEPP repo. The HILLSTAB03 (rev 39) and HILLSTAB06 (rev 40) amendments closed the **branch-selection** and **near-zero-runoff** authority gaps but did not author the **input-derivation** authority.

A reader of `SC-WATBAL-001` rev 40 cannot determine that `ealpha = 1.0` is wrong, because the contract does not say what `ealpha` should be.

## 6. Caveats

- **Static only.** No `cargo test`, no kernel invocation, no fixture run. The magnitude claims in §5.4 are derived from the algebra of `te = (efflen / (ealpha · vave^(m-1)))^(1/m)` and back-of-envelope values for `S` and `f`. They are not numerical-parity diffs against `appmth.for`.
- **Sampling limit on writers.** `grep` for symbol names targeted `Wb16ExponentM`, `Wb16Ealpha`, and the literal strings `"m"` and `"ealpha"` in `BoundarySymbol::from(...)` form. If a different boundary-symbol constructor or a code-generation macro emits these symbols, this audit would miss it. Spot checks via the constants module and the kernel contract enum did not surface additional producers.
- **Recency window.** The audit reflects working-tree state at 2026-05-29, after HILLSTAB06 landed `SC-WATBAL-001` rev 40 ([L1119](../specifications/science-contracts/contracts/SC-WATBAL-001.md#L1119)). It does not reflect any HILLSTAB07 or later work.
- **Legacy provenance only via `wepp-forest_260430_baseline/src/`.** Other legacy branches (`wepp_260131`, USDA mainline) were not consulted. The baseline-anchor decision is recorded at [ADR-0012](../decisions/0012-legacy-wepp-260430-baseline-anchor.md); this audit follows it.
- **Out of scope:** watershed peakro coupling (WSH-route `ipeakr`, channel `peakro` consumers), `efflen` provenance, `intsty_####` hyetograph provenance, irrigation `runtime_rate_m_per_s` provenance. Each is a separate input-provenance question.
- **`m` constancy is a legacy assumption, not a science statement.** This audit confirms openWEPP matches legacy. Whether `m = 1.5` is the correct regime for forested vs cropland vs row-crop surfaces is a science question outside the audit's scope; the WEPP literature distinguishes Chezy, Manning, and laminar regimes that map to different `m` values.

## 7. Recommended follow-ups (not performed in this audit)

- Author an `SC-HYDRAULICS-*` (or `SC-WATBAL-001` addendum) that declares the `ealpha` derivation chain: slope → friction-factor blend → per-OFE Chezy α → `alphay` snapshot → optional cascade projection. Cite the legacy lineage at the anchored baseline hash.
- Implement a producer kernel (or a parser-to-runtime projection) that emits `ealpha` per OFE per event from physical state. Replace the runner-seed `1.0` fallback with that producer. Hold the fallback only for explicit-override test vectors.
- Add a contract-derived test that exercises the producer chain end-to-end (slope/friction inputs → `ealpha` output) and a magnitude-bounds sanity assertion (e.g., `0.05 < ealpha < 50` for non-degenerate inputs).
- Decide whether `m` should remain a global constant `1.5` or be a regime selector keyed on land cover, and amend `SC-WATBAL-001` to declare the chosen authority.
- Re-run the HILLSTAB cohort harness after the `ealpha` producer lands to attribute residual WB16 failures to genuine branch-arithmetic issues rather than to silent input bias.

These follow-ups are work-package candidates, not audit conclusions. The audit only records the current-state finding that the input pipeline is absent.
