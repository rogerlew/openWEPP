# Claude Code Review — WBVAL01 Closure Identity, Residual Attribution, and the Fail-Closed Signal

Reviewer: Claude Code
Date (UTC): 2026-06-06
Evidence mode: **Static** — read the package artifacts (`single-ofe-closure-ledger.md`,
`run-manifest.md`, `rung2-frost-target-handoff.md`, `disposition.md`,
`gate-results.md`), the openWEPP WAT schema
(`crates/openwepp-hillslope-output/src/hillslope_wat.rs`), the summary accumulator
(`crates/openwepp-summary-accumulator/src/lib.rs`), and `SC-WATBAL-001`. I did
**not** re-run `openwepp-cli-hill`, read the per-day `.wat.parquet` cell values
(binary), or independently recompute the residuals. I assessed the ledger's
**method** against the schema/contract and spot-checked the published table
arithmetic only (e.g. p1 yr2: `934.600 - 821.683 - 86.692 = 26.225` ✓). The
`Ran` results are Codex's, attributed.

Verdict: **`executed-hold` is correct and the run hygiene is genuinely strong**
(see F5). But the headline chain — "12/12 conservation-break -> prioritized frost
targets" — is **not yet supported by the evidence**. Two findings gate it (B1, F2),
plus three quality notes (F3, F4, F5). Per the openWEPP review model these are
findings + evidence; disposition is Codex's, with my recommendation at the end.

---

## B1 (gating) — "conservation-break" is asserted before the closure identity is complete and before the leak is attributed

The ledger computes `R = (P + Irr) - (Q + Ep + Es + Er + Dp + latqcc) - Δ(SoilWaterTotal + Snow-Water)`
and labels every emitted hillslope `conservation-break` (R ≈ **+24 to +79 mm/yr**,
**same sign every hillslope, every year 2..6**). Two problems before that label can
mean "kernel does not conserve":

**(i) Resolved check — frozen water is NOT the gap.** I first suspected the ledger
omitted `frozwt` (frozen soil water), which would be a first-order term at a cold
1859 m site. It does not: the schema defines `Total-Soil` = *unfrozen* water,
`frozwt` = *frozen* water, and `SoilWaterTotal = watcon + frozwt`
(`hillslope_wat.rs:244-294`, `summary-accumulator/src/lib.rs:286`). The ledger's
storage term uses `SoilWaterTotal`, so frozen water is already captured. Recording
this so the next package does **not** chase a frozwt ghost.

**(ii) Remaining completeness gaps + undetermined attribution.** The published WAT
identity has terms the ledger does not include or verify:
- `InterceptionStorage` (Δ of `pintlv + resint`) — an "optional producer-authoritative"
  WAT storage term, omitted from ΔStorage. Confirm whether it is published/non-zero
  in these runs; if so its Δ belongs in the balance (likely small, but it is a
  storage compartment).
- `Tile` (tile drainage) — an output column, omitted. Likely ~0 for forest, but
  unverified.
- `UpStrmQ`, `SubRIn` (upstream inflow / subsurface run-in) — for a single-OFE
  hillslope these should be ~0, but the ledger assumes rather than demonstrates it.

If those are genuinely ~0/absent (plausible), then the identity is *substantially*
complete and the **+24-79 mm/yr leak is real internal non-closure** — i.e.
openWEPP publishes more water in (`P+Irr`) than it publishes out + stores. That is
worth knowing, but the package stops at the label without **attributing** it. The
sign matters:

- `R > 0` means water enters the published accounting and neither leaves as an
  output nor shows up as a storage gain — a **publication/mass leak**, water
  vanishing.
- This is the **opposite sign** of the known over-drainage residual
  (`project-waterbalance-overdrainage-residual`), where excess `Dp` would make
  outputs too large and push `R < 0`. So this is not that defect.
- A uniform, same-sign vanishing-water residual that scales at a snowy
  high-elevation site is more consistent with a **snow/phase mass-loss in the
  pack** (precip -> snowpack -> lost -> never delivered as `RM` -> never reaches
  soil -> never output) than with anything frost (frozen *storage* is already in
  `SoilWaterTotal`) or infiltration would produce.

**Why this gates frost:** the handoff hands all 12 to **frost** as closure-repair
targets. But frozen storage is already conserved in the ledger, and the leak's
sign points *away* from frost/over-drainage and *toward* the snow route that ADR-0017
+ the roadmap deliberately suspended. Driving frost off an unattributed,
possibly-snow-sourced residual risks re-running the HPHYS pattern: grinding a
mechanism that is not where the water is lost. This is the
`project-comparator-surface-artifacts` lesson applied to closure — prove the
identity is complete and attribute the residual **before** assigning it to a
mechanism.

**Recommendation:** before any frost targeting, (a) complete the identity
(add Δ`InterceptionStorage` + `Tile`; demonstrate `UpStrmQ`/`SubRIn` ≈ 0) and
re-derive R; (b) if the leak survives, attribute it (snow-phase mass balance vs a
publication-stage drop) — do not label it a frost target until a mechanism owns it.

## F2 (gating) — the 45% fail-closed rate is the harder, higher-confidence rung-1 signal, and it is upstream of frost

`10/22` single-OFE hillslopes **fail closed before any water balance is produced**.
These are concrete, reproducible openWEPP defects on real data — arguably
higher-confidence rung-1 work than the (unattributed) conservation breaks:

- **6 × `CLIM-RUNTIME-E-017`** (hourly radiation exceeds the physical
  extraterrestrial bound): `p2,p4,p6,p9,p14,p17`. This is the SIMIMPL28 radiation
  guard at `06_simimpl28_hourly_forcing.rs:704-713` — the **same module HPHYS0320
  just edited**. On real DRIGGS data the synthesized hourly radiation exceeds the
  sunmap-derived upper bound for 6 hillslopes. Either the bound is mis-calibrated
  (guard too tight) or the radiation synthesis is wrong on those aspects/slopes
  (forcing bug). Either way it is climate-forcing, upstream of frost, and worth a
  direct look given the module's recent churn.
- **4 × `HKERNEL-WB11-PERC-E-003`** (WB11 percolation domain violation): `p7,p11,p18,p20`,
  **all at the identical key — 1990 Julian day 95** (early April, snowmelt onset).
  Four hillslopes failing at one specific snowmelt-season day is a **systematic
  forcing/percolation-domain event**, not four independent bugs. Likely a
  snowmelt-driven percolation input violating the WB11 domain — again upstream of,
  or entangled with, snow rather than frost.

**Roadmap implication:** on real data the first walls are **climate-radiation
robustness** and a **snowmelt percolation-domain failure** — not frost. The
handoff does flag a "prerequisite/domain-unblock package," which is right, but it
still frames frost as the next mechanism rung and prioritizes the unattributed
breaks. I'd weight it the other way: the fail-closed defects are the concrete
rung-1 work, and "frost next" should be reconsidered until the run completes on a
real population.

## F3 (quality) — the "dominant magnitude output term" column is diagnostically misleading

The ledger and the frost handoff characterize targets by a "dominant magnitude
output term" (`Ep`-dominant vs `latqcc`) and even prioritize frost work by that
pattern. That column reports the **largest flux**, not the **source of the
residual**. R is unattributed (B1), so the largest flux says nothing about why the
balance fails to close. Using it to characterize/prioritize frost targets steers
the next rung by a signal unrelated to the defect. Drop it from target selection,
or replace it with an actual residual attribution.

## F4 (quality) — TOML-wrapper fidelity and a front-door ergonomics finding

The production `openwepp-cli-hill` front door **cannot consume a real wepppy `.run`
directory** — direct `.run` execution fails closed with `CLIHILL-E-010` (requires
TOML runfiles). Codex generated TOML wrappers referencing the `/wc1` inputs and
**inlined `snow.txt` values** (`rst=0.0`, `newsnw=100.0`, `ssd=250.0`). Two notes:

- The entire result is conditional on the wrappers faithfully reproducing the
  legacy `.run`/sidecar selection (management, soil, climate, and especially the
  inlined snow parameters). That fidelity should be on record; the inlined snow
  params are a harness-injected assumption, not read from the run.
- That real-run validation requires a hand-rolled wrapper-generation step is itself
  a finding: every future real-run validation pays this tax and inherits its
  fidelity risk. A small `.run`-directory adapter (or a documented, tested wrapper
  generator) would de-risk this and unblock the broader validation campaign the
  roadmap implies.

## F5 (positive) — the execution discipline is the ADR-0017 posture done right

Worth stating plainly because it is the part not to regress: real release binary
built and **SHA-256-recorded** with source commit; **no WAT terms imputed** for
blocked hillslopes; year 1 **honestly excluded** with an explicit
`initial-storage-missing` label rather than a fabricated t=0 row; multi-OFE `pw0`
correctly deferred to MOFE/observe-only; and the domain violations were **preserved
as fail-closed**, not canonicalized away. The `executed-hold` disposition is the
honest call given the incomplete population.

---

## Disposition recommendation (Codex owns)

Keep `executed-hold`. Before this evidence drives a frost package:

1. **Complete the closure identity** (add Δ`InterceptionStorage` + `Tile`; show
   `UpStrmQ`/`SubRIn` ≈ 0; `SoilWaterTotal` already includes `frozwt` — confirmed)
   and re-derive R. Determine whether the +24-79 mm/yr leak survives.
2. **Attribute the surviving leak** before labeling it a frost target. Its sign
   (water vanishing) points away from frost/over-drainage and possibly back at the
   suspended snow route — do not silently reopen snow, but do not mis-assign it to
   frost either.
3. **Treat the 10 fail-closed defects as the concrete rung-1 work** (SIMIMPL28
   radiation bound; WB11 percolation day-95) and re-decide sequencing: a
   domain-unblock + closure-identity-completion pass likely precedes frost.
4. **Stop using "dominant magnitude output term"** to characterize/prioritize
   frost targets.
5. Consider a tested `.run`-directory adapter so the validation campaign is not
   wrapper-fidelity-bound (F4).

I did not run the suite; acceptance of the above against the contract is Codex's.
