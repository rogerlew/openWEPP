# Increment 1c-fidelity (ROADMAP §E.1) — Single-OFE Surface Completeness: Entry Gate

Author: Claude Code, 2026-07-04. Evidence: **Static** (legacy source-intent recon
in `/workdir/wepp-forest_260430_baseline/src/` via a read-only survey agent, the
current direct runtime, SC-SED-001) — no execution at authoring time; the
execution record lands in the implementation notes below this artifact's design
sections as the increment runs.

Status: executed in the same pass (operator direction 2026-07-04: "scaffold and
execute E.1"). Executor: Claude Code (ADR-0035 authorization). Scope authority:
[`increment-2-entry-gate.md`](increment-2-entry-gate.md) §4/§7 (stage 2a) and
[`docs/ROADMAP.md`](../../../ROADMAP.md) §E.1.

## 1. Scope (three items, all structural)

1. **`field_width_m` from hillslope geometry** — replace the unit-width `1.0`
   default in `direct_production_wave1_operand_seed`
   (`00_builders_and_authority.rs:1111`) with the parsed slope-file profile
   width (`fwidth`).
2. **5-class `sedcon` publication** — the genuinely-zeroed surface: the Wave-1
   publication projection publishes `[0.0; 5]`
   (`direct_runtime/erosion.rs:1363`) while the pass parquet already carries
   `sedcon_1..5` columns and the watershed kernel already consumes per-class
   concentration (`watershed-orchestrator kernel/direct.rs:764`).
3. **Nonzero-deposition validation coverage** — `tdep`
   (`total_deposition_kg`) is already published and solver-computed; it reads 0
   on p61/DFF-WS3 only because those profiles are detachment-dominated. The gap
   is a *depositing* fixture proving the deposition limb conserves.

Out of scope (deliberately): the HBP EVENT schema (stays the single-class
schema1 fixture payload — the npart-resolved HBP surface is designed **once**
with the hourly-flow surface in E.2/2b, per `increment-2-entry-gate.md` §8
"HBP schema is designed once"); enrichment (`enrich.for` lineage = E.4/2d);
any magnitude judgment (E.5).

## 2. Legacy source-intent resolved (recon, baseline `dac3c950`)

The four load-bearing facts, from the read-only survey of
`wshpas.for`/`sloss.for`/`sedseg.for`/`route.for`/`param.for`/`enrich.for`:

1. **`sedcon` formula** (`sloss.for:305-317`, last OFE only):
   `sedcon(i) = avsole / (runoff·efflen) · frcflw(i)`, units kg/m³,
   zero when `peakro <= 0`. `avsole` is the per-unit-width exported load
   (kg/m). **This is exactly the scalar concentration the Wave-1 solver
   already publishes** (`erosion_continuity.rs` `wave1_totals`,
   `exported_kg_m / (runoff_depth_m · efflen_m)`, the `sloss.for:314` form) —
   so the per-class array is `scalar_concentration × frcflw(i)`.
   **`sedcon` is width-independent** (numerator and denominator are both
   per-unit-width; `fwidth` cancels). The watershed reconstructs per-class
   mass as `sedcon(i) × runvol` (`wshred.for:180-186`), where `runvol`
   carries `fwidth` (`contin.for:1240-1245`).
2. **`frcflw` without enrichment**: `route.for:142-160` initializes
   `frcflw(i) = frac(i)` (the `prtcmp` detached-soil composition) at the top
   of the OFE when there is no upslope inflow. `enrich.for` re-proportions
   only in **deposition** regions, plus a terminal end-of-OFE blend
   (`enrich.for:205-213`) of inflow/rill/interrill compositions. On the
   **non-cropland path** (`param.for:452-458`: `fidel(i) = frac(i)`,
   `intdr = 1`), with zero deposition and zero inflow, that blend collapses
   **exactly** to `frcflw(i) = frac(i)`. The cropland path does *not*
   collapse (`fidel = frac·drinti/intdr`, `param.for:446`).
3. **`tdet`/`tdep` are total kg in the legacy pass file** — scaled by
   `fwidth` at construction (`sedseg.for:389-391, 512-514`:
   `tdet = sum2·fwidth·filoss`). The openWEPP HBP payload already declares
   `total_detachment_kg` / `total_deposition_kg` in kg
   (`parsers/hbp/types.rs:110-111`), so the unit-width default has been
   publishing per-width numbers into a field whose contract says absolute
   kg. Wiring `fwidth` is a **payload-semantics correction** (INV-SED-010),
   not only a fidelity item.
4. **The continuity solve is single-effective-class in legacy too**
   (`param.for:570-609`: `veleff` from the three finest classes;
   `erod/depos/runge` operate on scalar load). Per-class machinery lives
   entirely in `frcflw`/`enrich`. Our class-blind solver is therefore
   **faithful to the legacy decomposition**, not a simplification of it.

## 3. Adjudications

### 3.1 Per-class `sedcon` basis = detached composition (`frac`), pre-enrichment

Publish `sedcon_i = frac_i × scalar_concentration` on all routed days.

- **On the current enable scope this is legacy-exact whenever the profile
  does not deposit**: the enable is single-OFE (`qin = 0`, `strldn = 0`) and
  non-cropland (`is_cropland = false`), which is precisely the
  `frcflw = frac` collapse case (§2.2).
- **On depositing days the split is the un-enriched composition** — legacy
  would re-proportion toward fines (`enrich.for`). This is a **labeled
  scope-limit** under `INV-SED-011` (enrichment procedure caveats must stay
  explicit), superseded by E.4/2d (`enrich.for`-lineage port). Total exported
  mass is exact regardless of the split: the watershed consumer forms
  `Σ_i sedcon_i × runvol`, and `Σ_i frac_i = 1` makes that identical to the
  scalar-concentration mass. The class *distribution* is first-cut only
  where deposition occurred.
- Rejected alternatives: per-class-only-when-no-deposition (inconsistent
  surface for consumers); porting `enrich.for` now (that is 2d's coupled
  particle work per the Increment-2 entry gate — deposition re-proportioning
  without per-class deposition state would be a half-port).
- Guard: fail-closed validation that the seeded composition closes at the
  SC-SED-001 `TOL-SED-005` tolerance (`|Σ frac − 1| ≤ 1e-9`) before the
  split publishes (a zeroed class table must error, not publish zeros);
  the split is then normalized by the validated sum so the published
  class sum equals the scalar toe concentration to f64 rounding
  *(round-1 hardening — the initial cut gated at 1e-6 without
  normalization, which admitted up to 1e-6 relative drift between the
  class sum and the scalar; see §6)*.

### 3.2 `field_width_m` source = parsed slope `fwidth`, threaded typed

The parser already validates `fwidth` finite/positive on both the 2023.3
per-OFE and shared-metadata paths (`parsers/slope.rs:383-386, 451-455`) but
the typed projection drops it (`TypedSlopeOfeRuntimeProjection` carries
`azimuth_deg` and not `fwidth`). Add `fwidth_m` to the projection (guarded
finite/positive at projection like `slplen_m`, fail-closed) and source the
seed from it. `sedcon` is unaffected (width-independent, §2.1); `tdet`/`tdep`
and the HBP payload rescale to true kg.

### 3.3 Deposition coverage = concave-profile fixture-forcing solve

Reuse the `erod16` harness shape (real McKenzie fixture forcing read back
from the pass parquet + production operand producers + direct solver call),
with a **concave** slope profile (steep upper, near-flat toe) in place of the
fixture's own: transport capacity collapses on the toe and the deposition
limb (`depc`/`depeqs`/`depend`/`depos` lineage) must engage. Gate: at least
one storm day with `total_deposition > 0`; the telescoping conservation
identity holds on every active day (in-solve hard gate + external assert);
per-class sum equals the scalar concentration. A full-run depositing fixture
(new .sol/.man/.cli set) is deliberately deferred — the deposition *forcing*
representation changes in E.2 (hourly falling limb), so crafting one now
would be re-crafted immediately after.

### 3.4 Record correction: the p61 "~5×" is not `field_width`

The 1b-C close-out hypothesized p61's ~5× magnitude gap was "likely
`field_width = 1.0`". The recon refutes that attribution twice over:
(a) the legacy comparand (`H61.ebe.dat` `Sed.Del 4.2 kg/m`) is
**per-unit-width**, and so was our width-1.0 `tdet = 20.9` — the ratio is
width-independent; (b) it compares total **detachment** against delivered
**export** (`Sed.Del`), a different cut-point when any deposition occurs
(per [`project-comparator-surface-artifacts`] discipline: prove cut-points
are like-for-like before attributing). Magnitude decomposition remains E.5,
judged after the water-magnitude authority closes, comparing
`exported_kg_m` (not `tdet`) against delivery-class legacy surfaces — as a
flag, never a target (ADR-0017).

### 3.5 Adjacent gap observed, explicitly NOT touched (E.2 intake)

The legacy pass EVENT writes `peakro(nplane)·harea(ihill)` — a true m³/s.
The openWEPP HBP `peak_runoff_m3_s` is fed from the WB16 peak surface whose
frame fields behave as a depth-rate (m/s) despite the suffix (a known
misnomer, 1b-C record). That unit question belongs to the E.2 HBP redesign
(the hourly-flow EVENT surface work), where the peak/volume summary fields
are already being re-specified. Out of E.1's write set; recorded here so 2b
inherits it as an explicit intake item.

## 4. Gate (stage-2a acceptance, from the Increment-2 entry gate)

1. Single-OFE closure unchanged: full suite green, in-solve conservation
   gates untouched.
2. p61 / DFF-WS3 directional law intact (both are width-invariant by
   construction: p61 asserts nonzero/finite, WS-3 compares two cells sharing
   one canonical slope file, `fwidth = 102.4`).
3. A depositing fixture produces conserving nonzero `tdep` (§3.3).
4. The now-nonzero `sedcon` conserves per class: `Σ_i sedcon_i` equals the
   scalar toe concentration (unit + integration coverage), and the DFF-WS3
   finite-only placeholder assertions are strengthened to nonzero.
5. Full AGENTS gates; branch pushed for Codex review; no self-merge.

## 5. Execution record (Ran, 2026-07-04, branch `erosion-e1-inc1c-fidelity`)

- **`field_width_m`**: `TypedSlopeOfeRuntimeProjection.fwidth_m` added
  (fail-closed finite/positive at projection, `HS-RUNTIME-E-065/066`,
  variant-coverage test extended); seed sources `slope_ofe.fwidth_m`.
  **Ran** (p61 end-to-end): `tdet = 15,148.098 kg` = `20.913 kg/m ×
  724.3 m` — the per-width magnitude is bit-for-bit the 1b-C value scaled
  by the parsed width; `sedcon` unchanged (width-independent, as
  `sloss.for` requires).
- **5-class `sedcon`**: `direct_wave1_publication_projection` publishes
  `frac_i × toe-concentration` with a fail-closed `Σ frac ≈ 1` guard
  (`WAVE1_CLASS_FRACTION_SUM_TOL = 1e-6`; a zeroed/unseeded class table is
  a typed error, never a silent zero split). Basis labeled `GAP-SED-007`
  (SC-SED-001 v42) with three new provenance anchors
  (`REF-SED-LEGACY-SLOSS-SEDCON`, `-SEDSEG-WIDTH`, `-FRCFLW-INIT`).
  **Ran** (p61): class sum `19.077 kg/m³` = the prior scalar exactly; all
  five classes nonzero. Unit tests: split identity, class-sum
  conservation, unseeded-composition fail-closed; frame-level span test
  extended.
- **Depositing coverage**: concave validation-instrument profile
  (`[(0,0.85),(0.30,0.70),(0.55,0.18),(0.75,0.03),(0.88,0.008),(1.0,0.003)]`)
  under the full 227-storm real McKenzie population with production
  producers. **Ran**: 190 clean solves, **all 190 deposit**
  (`tdep = 52,444 kg`, `tdet = 831,958 kg`), per-day telescoping
  conservation + export-bound asserted on every clean day; 37
  slowest-peak storms refuse via the named `flux_closure` discretization
  gate (16.3%, bounded ≤ 20% by assertion; any other error class fails
  the test). Design note: toe flatness and stiffness pull opposite ways —
  a profile-family sweep against the real storm envelope showed every
  depositing profile refuses its stiffest slow-peak days
  (`eata ∝ shrsol/tcend` exceeds the 101-point grid's resolution), so the
  refusals are the fail-closed guard working, where legacy's identical
  100-point grid (no such gate) silently integrates.
- **Validator caveat**: "full suite" evidence at push time is the AGENTS
  gate battery recorded in the WP status update; the depositing instrument
  is solver+producer-level (real forcing, real operands), not a full-run
  production fixture — the production-path depositing fixture is
  deliberately deferred to E.2, whose hourly substrate changes the
  deposition forcing representation.

## 6. Codex review round 1 (2026-07-04) — all three findings fixed

1. **Medium — Σ frac tolerance vs the mass-exact claim (CONFIRMED).** The
   guard admitted `|Σ frac − 1| ≤ 1e-6` while the contract's class-fraction
   closure is `TOL-SED-005` (`≤ 1e-9`) and the GAP-SED-007 text claimed the
   class sum "exact" — a seeded table summing to `1.0000005` would have
   published a class sum ≠ the scalar concentration. Fix: guard tightened
   to `TOL-SED-005` (1e-9) **and** the split normalizes by the validated
   sum, making `Σ sedcon_i =` scalar to f64 rounding by construction (the
   division is a ≤ 1e-9 adjustment inside the closure tolerance, not a
   correction of an invalid composition — those fail the gate). Contract +
   code docs re-worded from "exact" to the constructive form.
2. **Medium — `field_width_m` had no executable regression (CONFIRMED).**
   The p61 test asserted only `tdet > 0`/finite; a revert to unit width
   would have passed. Fix: output-level reconstruction in the p61 test —
   on zero-deposition event days, `tdet = Σ_i sedcon_i × runvol` ties the
   erosion-seed width to the water-path area through two independently
   produced surfaces (`sedcon` width-independent, `runvol` area-scaled).
   **Ran** both directions: green on the real path (residual ~2.4e-16
   rel, gated at 1e-9), and a deliberate temporary unit-width revert
   fails it by the width factor (`20.91` vs `15,148.1 kg`) — restored
   after the negative check.
3. **Low — stale pre-1b-C comment in the erosion authority builder
   (CONFIRMED).** The "production seed cannot yet construct its operand
   payload" block predated 1b-C. Rewritten: the continuity solve is
   production-active via the operand-seed path; `wave1_enabled` there is
   the separate (still-disabled) Increment-1 pointwise EROD13 check.
