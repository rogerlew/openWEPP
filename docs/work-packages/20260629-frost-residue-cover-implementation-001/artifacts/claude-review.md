# Claude Code Review — Frost Residue-Cover Implementation

**Reviewer:** Claude Code · **Date:** 2026-06-29
**Evidence mode:** Static (read the production code, the `SC-RESIDUE-001` rev 11
amendment, and the package artifacts) + Codex's Ran (the verification gates and the
`18 → 14` diagnostic result; not independently re-run by this reviewer).

## Overall: ACCEPT as a real contract-first improvement — three findings to resolve

The implementation is sound and is genuine forward progress (the first dynamic
frost-residue coupling openWEPP has had). `INV-RESIDUE-019` is a well-formed
contract: the plant→residue transfer is contracted **mass-conserving**
("decomposition/removal/grazing are the only authorized losses"), static-seed reuse
after a mass change is a **hard-fail**, the **inert/no-senescence path is explicitly
preserved** (zero `oratea` stays a valid no-decay constant), and the snow-test
changes are a mechanical `contract_version 112 → 113` cascade, **not** a physics
ripple. Conservation and inert-identity are contracted and the workspace gates are
green (Codex's run).

The three findings below are calibration/consistency issues, not correctness
blockers in the coupling itself. **#1 and #3 should be resolved before this lands;
#2 should be recorded.** Per the openWEPP review model these are surfaced as
issues + evidence; the disposition and the exact fixes are Codex's.

---

## Finding 1 — The disposition overclaims: `18 → 14` is a *partial* contribution

**Evidence:**
- `residue_parameterization_diagnostic.md:7` — "the Sleepers timing candidate
  defects **are attributed to** fixture residue parameterization."
- Same artifact: candidate-defect cells reduced **`18 → 14`** (South `4 → 2`,
  W9 `14 → 12`) — i.e. **4 of 18 cells cleared; 14 remain.**

The signature is *not* attributed to residue parameterization; **a minority of it is.**
Seasonal residue is a **confirmed contributor** (branch-A direction validated — onset
moved in the observed direction), but it accounts for ~22% of the candidate-defect
cells. The disposition wording attributes the whole signature to a cause that
explains a quarter of it.

**Supporting signal (weak seasonal separation):** autumn mean `0.0786372 m` vs spring
mean `0.0752999 m` — the autumn/spring *mean* separation is only ~4%, even though the
within-year depth ranges `0.0185 – 0.0906 m`. The litter stays high into spring, so
the seasonal residue relieves **onset** more than **thaw** — consistent with only a
few cells clearing, since the residual is thaw-late-dominated (see Finding-adjacent
signal below).

**Surfaced for Codex:** calibrate the disposition to a confirmed *partial* contributor
(`4/18` cleared, `14` remain) rather than a full attribution; GAP-SNOWFREEZE-002
should reflect that 14 cells are still unattributed.

## Finding 2 — The autumn litter drop is anchored on a FIXED Julian date, not physical forcing

**Evidence:**
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:3005-3013`
  — `fall_litter_drop_window_contains`: `end = jdharv`,
  `start = jdharv − FOREST_LITTER_DROP_WINDOW_DAYS (45)`; window `[jdharv−45, jdharv]`.
- `uses_fall_litter_drop_schedule` (`:3001`) gates on `imngmt == 2 && jdharv > 0 &&
  has_seasonal_litter_signal`.
- For these managements `jdharv ≈ 286` (≈ Oct 13) — the WEPPcloud fixed-date fallback.

The leaf-drop timing is the **fixed Julian `jdharv`**, the exact pattern the
leaf-on/leaf-off backlog's core design rule forbids ("no fixed Julian dates
anywhere"). **The contract is honest about this** — `INV-RESIDUE-019` says the window
ends "on the **management fall date**," making no false physical-forcing claim — which
is the right call.

This is an **acceptable first pass**: the litter drop shares the `jdharv` anchor with
the canopy leaf-off, which is itself still backlogged as fixed-date, so the litter
input cannot be more physically-driven than the senescence event it follows. It is
**not** "physically-driven seasonal litter."

**Surfaced for Codex:** record the fixed-`jdharv` anchor as the known limitation in the
package + strategy §11, to be **re-anchored to the physical frost/daylength trigger
when the leaf-on/leaf-off phenology backlog lands** (the same anchor fixes both limbs).

## Finding 3 — The forest-litter decay constant contradicts its own cited authority

**Evidence:**
- `00_builders_and_authority.rs:9` —
  `FOREST_LITTER_FALLBACK_DECAY_RATE_PER_DAY: f64 = 1.25 / 365.25` (**k = 1.25 yr⁻¹**).
- `INV-RESIDUE-019` (SC-RESIDUE-001 rev 11) — "the authority-backed forest-litter
  turnover fallback (**k = 1.25 yr⁻¹** …)".
- `REF-RESIDUE-FOREST-LITTER-DECAY` (same contract) — cites "roughly 43% first-year
  turnover, **corresponding to k≈0.56 yr⁻¹**" and "authorizes a moderate
  **k = 0.5 yr⁻¹** first-order surface-litter fallback."

The implemented constant is **~2.5× the literature value the contract itself cites**
(k = 1.25 → 71% first-year turnover, vs the cited 43%). The constant, the invariant,
and the changelog say `1.25`; the reference row says `0.5` and cites evidence for
`≈0.56`. A k that fast is also in tension with the backlog's "slowly-decaying
multi-year forest floor."

**Surfaced for Codex:** reconcile the constant against `REF-RESIDUE-FOREST-LITTER-DECAY`
— either the constant should be ~`0.56 yr⁻¹` (matching the cited 43% turnover) or the
reference text is stale and must be corrected; right now the contract is internally
inconsistent, and the "not fixture tuning" claim depends on the constant matching its
authority.

---

## Adjacent signal for the follow-up (hypothesis, not a finding)

The 14 remaining cells are thaw-late-dominated (Step-2: South all-thaw-late; W9
early-onset + thaw-late). Because the litter stays high into spring (autumn ≈ spring
mean, Finding 1), seasonal residue relieves **onset** but leaves **spring
over-insulation** — so it would not be expected to clear thaw-late cells. The lever
for the thaw residual is therefore likely **spring litter decay/compaction** (does the
litter thin enough under the spring snowpack?) **or** a genuine frost-solver item
(`Qwet` / legacy-envelope) — to be separated in the follow-up, not this package. This
also bears on Finding 3: the decay rate is exactly what governs how much litter
survives to spring.

## Recommended routing

- **Before landing:** resolve **#1** (disposition wording) and **#3** (decay constant
  vs cited authority).
- **At landing:** record **#2** (fixed-`jdharv` anchor) as the known limitation in the
  package + strategy §11.
- Disposition and exact fixes are Codex's (its code + contract).
