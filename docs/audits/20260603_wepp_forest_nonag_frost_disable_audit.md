# wepp-forest Non-Ag Frost Disablement — Static Audit — 2026-06-03

Status: Final
Last updated: 2026-06-03
Evidence mode: Static
Scope: In scope — whether the pinned `wepp-forest_260430_baseline` simulates soil frost for the non-agricultural (forest/range) hillslopes in the HPHYS parity cohort, the mechanism, its recorded provenance/rationale, and a physical-sensibility assessment. Out of scope — openWEPP's runtime frost handling (flagged as follow-up), snowpack/SWE behavior, and any execution.

## 1. Purpose

Does the pinned baseline simulate soil frost for the non-ag (forest/range) hillslopes used in the HPHYS0249–0266 parity work; by what mechanism; with what recorded rationale; and is disabling frost for all non-ag physically defensible? This audit consolidates the static frost analysis conducted while reviewing the HPHYS storage/`Ep` residual stream.

## 2. Method

Static reads only (no execution beyond `grep`/`git log`/`git show`):

- Baseline source at pinned hash `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` (`/workdir/wepp-forest_260430_baseline`): `src/winter.for`, `src/infile.for`, `src/scon.for`, `src/cke.inc`, `src/contin.for`.
- `git show 189e32e8` and `git log -L 335,358:src/winter.for` in the baseline repo for provenance of the frost gate.
- wepppy (`/home/workdir/wepppy`): `wepp/soils/utils/wepp_soil_util.py`, `nodb/mods/disturbed/README.md`, `nodb/mods/disturbed/data/*lookup*.csv` for the scope of the `ksatadj` flag.
- Not executed: no WEPP/openWEPP runs, no openWEPP source consumption of `ksflag` was confirmed (see Caveats / Follow-ups). The Dun et al. 2010 reference was used at the level of its title/citation, not a read of the paper's internals.

## 3. Findings

### 3.1 Mechanism: frost is gated on `ksflag`, which is forced to 0 for all non-cropland

| Element | Location | Behavior |
|---|---|---|
| Winter entry | `/workdir/wepp-forest_260430_baseline/src/contin.for:849` | `call winter(rain(iplane), snoflg)` — the run-option `snoflg` is the master switch for winter (snow+frost). |
| Frost gate | `/workdir/wepp-forest_260430_baseline/src/winter.for:339` | `CAS ksat-adj: Allow frost calc for agricultural crops only. A. Srivastava` → `if (ksflag.eq.1) call frostN(hour)`. When `ksflag=0`, `frostN` is skipped (`wfr_skip` incremented). |
| Snowpack | `/workdir/wepp-forest_260430_baseline/src/winter.for` (after the frost block) | `call snowd(...)` is **unconditional** — snowpack runs whenever winter runs, independent of `ksflag`. |
| `ksflag` semantics | `/workdir/wepp-forest_260430_baseline/src/cke.inc:18` | "user input flag (in SOIL file)… `ksflag=0` means use no adjustments; `ksflag=1` means use all adjustments" (internal hydraulic-conductivity adjustment). |
| Non-ag force-off | `/workdir/wepp-forest_260430_baseline/src/infile.for:2205` | `if (lanuse(1).ne.1) ksflag = 0` — any non-cropland land use forces `ksflag=0`. |

**Net:** for every non-ag (forest/range) hillslope, `ksflag=0` → `frostN` never runs → frozen-soil simulation is OFF. Snowpack (`snowd`) still runs when `snoflg=1`. **`ksflag` disables frost, not snow.**

### 3.2 Provenance: a provisional change with no physical justification on record

- Origin commit `189e32e8` ("Ksat adjustment and switching off frost simulation", author `SrivAnu` / A. Srivastava). Message: *"…also made a change to switch off frost simulation in the winter SR for forest hillslopes. Still allows frosting for agricultural soils. **This will all need to be looks at when forest plant regeneration routines are incorporated in future.**"* The change is **explicitly provisional** and was **bundled** with the forest hydrophobicity (`ksatadj`) work, not justified on frost physics.
- The underlying "non-ag → `ksflag=0`" convention is older and was **self-flagged as unjustified** by the original WEPP authors: `/workdir/wepp-forest_260430_baseline/src/scon.for:499` (*"For Range ksflag = 0 in all cases. why? dcf 11/29/95"*) and `/workdir/wepp-forest_260430_baseline/src/infile.for:2201` (*"…After repeated attempts to obtain information from Tucson ARS group (Weltz, Kidwell)… changed back 3/5/97 - dcf"*).

No documented physical rationale exists for disabling frost on non-ag in either the original convention or Anurag's commit.

### 3.3 Scope mismatch: the guard is far broader than its apparent motivation

- `ksatadj` is the **fire-hydrophobicity** flag, not a general forest flag. wepppy disturbed `README.md:212`: *"When `ksatadj=1`, WEPP dynamically adjusts conductivity based on soil saturation… [simulating] hydrophobic layers — waxy residues from burned organic matter."* The disturbed lookup shows **forest low-sev fire → `ksatadj=0`** (`README.md:192`), i.e. `ksatadj=1` is essentially the high-severity-burn case.
- The frost-off rides on `ksflag`, which is 0 for **all** non-ag (burned or not), whereas `ksatadj=1` exists only on high-severity burns.

The frost-off is therefore scoped far more broadly than the `ksatadj` concern it shipped alongside. Mechanically it reuses a flag that was *already* zero for all non-ag rather than expressing a targeted condition — consistent with a coarse guard rather than a frost-physics decision.

### 3.4 Physical assessment: buffering is defensible; absence is not

- **Frost buffering is defensible for forest.** Forest litter, duff, canopy, and an insulating snowpack reduce frost penetration depth and duration relative to bare/tilled cropland. A model that *damps* forest frost is physically reasonable.
- **Frost absence is not defensible.** High-elevation, snow-covered forest soils — exactly the HPHYS cohort, whose first `Ep` divergences land in mid-January — do freeze. Setting frost identically to zero discards a real winter process.
- The WEPP frost subroutines were a recognized, actively-improved component, not a disposable one: **Dun, S., Wu, J.Q., McCool, D.K., Frankenberger, J.R., Flanagan, D.C. (2010). "Improving frost-simulation subroutines of the Water Erosion Prediction Project (WEPP) model." Transactions of the ASABE 53(5):1399–1411.** `wepp-forest` is a Dun-lineage codebase (pervasive `S. Dun` modifications in `evap.for`, `swu.for`, etc.; a "winter Dun-dissertation review" appears in the baseline doc history), which makes wholesale frost disablement in this fork especially hard to justify. *(This bullet's physical reasoning is the auditor's Static inference; the Dun 2010 reference is cited at the level of its title/citation, not a read of its internals.)*

### 3.5 Parity implication

- Frost-off is a property of the **baseline** for these hillslopes. It is therefore **not** a candidate openWEPP-vs-baseline divergence source **provided openWEPP also honors `ksflag=0`** (no active frozen-soil path; `frzw≡0`). This is the static explanation for HPHYS0252's "inert by construction" result: the `fzdrfc = drfc − frzw` frozen-adjusted lateral cannot move anything when `frzw≡0`.
- It **caps the meaningfulness of winter-season parity.** The worst residuals (mid-January `Ep` onset, `Snow-Water`, `RM`) sit in the season where frost-absence matters most. Matching a baseline that omits winter soil frost can close the *parity gap* without producing *correct* winter forest hydrology.

## 4. Caveats

- **Static only.** No WEPP or openWEPP execution. Counts/behaviors are read from source and commit metadata.
- **openWEPP side not verified here.** Whether openWEPP's runtime actually treats frost as off (i.e. honors `ksflag=0`, keeps `frzw≡0`, and renders the `fzdrfc` machinery a no-op) is asserted as the *expected* consistent behavior but was not confirmed in this audit.
- **Dun 2010 not read.** Cited from the user-supplied reference and its title; no specific findings or numbers are attributed to it.
- **`ksatadj` severity threshold** is characterized from the wepppy disturbed README and lookup examples (low-sev → 0), not an exhaustive enumeration of every severity row.
- **Recency.** Baseline line numbers are valid at hash `dac3c950`; the `winter.for:339` frost gate predates the visible history for that block (only diagnostic counters were added around it in commit `c0b07c89`).

## 5. Recommended follow-ups (not performed in this audit)

1. **Confirm openWEPP frost-off consistency** — verify `frzw≡0` for the non-ag cohort and that the `fzdrfc`/frozen-lateral path is a no-op, so frost-off is a shared assumption and not a hidden divergence.
2. **Decide the correctness posture** (work-package / ADR territory, not an audit): pure legacy parity (accept baseline frost-off as the target) vs. physical correctness (implement Dun-2010-style buffered forest frost). These diverge precisely in the winter season where current residuals are worst.
3. **External winter sanity check** — compare against observed SWE / frost-depth for representative sites, given that the baseline's winter forest hydrology is physically incomplete and is therefore a weak parity authority for `Snow-Water`/`RM`/winter `Ep`.
