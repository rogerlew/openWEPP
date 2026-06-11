# D3 Fine-Sublayer Port — M1 Scoping Phase Prompt

Status: queued (scoping phase — produces scope, not code)
Author: Claude Code, 2026-06-11
Authority: `SC-SNOWFREEZE-001` v57 (`INV-SNOWFREEZE-006`/`-012`), FDHP01
package envelope, addenda 2e/3 + the D3 coarse-front failed-attempt evidence.

## Phase framing

This is an **M1 scoping artifact, not an implementation prompt.** The next
Codex pass executes this phase and fills the Deliverables sections below
(in this file). **No production code edits in this phase.** The completed
artifact becomes the **first required-reading item** of the subsequent
implementation pass.

Why this phase exists: two D3 shortcuts have now failed in a structured way —
the scalar-`frdp` model could not carry mass (addendum 2e), and the coarse
per-layer energy front ran clean but missed the envelope (mean max 643 mm vs
legacy 414, duration delta −428 days; recorded as failed evidence and backed
out, `efd2152b`). Both skipped the scoping step. The fine-sublayer port is
evidence-forced; this phase makes it land against a declared state shape
rather than an evolving one.

Subagent requirement: none for this phase (reading + authoring only; no
heavy runs).

## Required reading for this phase

- This package's `package.md` (envelope, protected boundaries) and
  `review_claude_fdhp01_closure_status_and_cohort_validation.md`
  (addenda 2e, 3 — why depth and mass decoupled).
- `SC-SNOWFREEZE-001.md` v57 (`INV-SNOWFREEZE-006`, `-012`, `-013`, the D3
  tightening) — the authority the map must trace to.
- Legacy pinned baseline `/workdir/wepp-forest_260430_baseline/src/`:
  `frostn.for` (entry handoff `:336`, hourly dispatch `:662–686`),
  `frwatc.for` (`wbtofs=0` frost→WB `:80–137`; `wbtofs=1` WB→frost `:139+`),
  `frzng.for`, `frznw.for`, `mlttp.for`, `mltbtm.for`, `watdst.for`
  (unfrozen-water redistribution under frost, Saxton–Rawls), plus the
  `watbalprint.for` publication (`soilw`/`soilf` summation, `:56–69`).
- CRM Ch. 3.8; Dun et al. 2010 (fine-layer discretization rationale).

## Deliverables (fill the sections below; all `Static:` legacy-source-traced)

### 1. Legacy state-machine map

Per-symbol table: name, meaning, units, dimensionality (per fine sublayer /
per soil layer / per plane), where written, where read, lifecycle (persists
across days vs rebuilt). **Must cover at minimum:**
`fgfrst`, `fgthwd`, `slfsd`, `slsic`, `slsw`, `nwfrzz`, `frozen`, `frzw`,
`soilw`, `st`, `yst`, `nfine` — extend with any symbol the routines
read/write that the implementation must carry (e.g. tillage-zone depth,
`qsrf`/`quf` accumulators). Note Dun-2008 fine-layer count per soil layer
and how sublayer thickness derives from `dg`.

### 2. Routine sequence and trigger map

The dispatch chain with **trigger conditions and arguments**, traced to
source lines — minimum:

```
winter → frostN
  frwatc(1)                          entry water-state handoff (frostn.for:336)
  hourly loop (frostn.for:662–686):
    freeze arm:        frzng(hour) → watdst(0., 3600., 2)
    freeze+bottom-melt: frzng(hour) → if qdry>0 mltbtm(hour) → watdst(0., 3600., 2)
    top-thaw arm:      mlttp(hour) → [mltbtm(hour)] → watdst(0., 3600., 0)
    bottom-thaw arm:   mltbtm(hour) → watdst(0., 3600., 0)
  frwatc(0)                          exit handoff at hour 24 OR fgthwd=1
```

Document: what selects each arm (sign/magnitude of `Qsrf`/`Quf`, snow/residue
state, existing front), what `frznw` does relative to `frzng`
(already-frozen-water refreeze), the `watdst` mode-flag semantics (`fgfzft`
0 vs 2), and the `fgthwd` thaw-complete early exit. This is the executable
form of `INV-SNOWFREEZE-012`; divergence from it must be contract-amended,
not improvised.

### 3. openWEPP state-shape proposal

The Rust-side state design **before code**: runtime symbol names
(`frost.runtime_*`), per-sublayer array shape and ownership, persistence
across days, and the explicit legacy↔openWEPP alias mapping table (naming
continuity per `docs/standards/kernel-work-package-preparation.md` §4 —
record aliases in `SC-SNOWFREEZE-001`). State must make frozen-water mass
**accumulated** (by the energy balance, layer by layer) and depth **derived
from the layered state** (deepest frozen sublayer), never the reverse —
the addendum-2e inversion is the named anti-pattern.

### 4. Seam mapping — `frwatc(1)` / `frwatc(0)` onto existing openWEPP seams

Explicit mapping of both handoff directions onto the current WB18/WB11
percolation-storage seam, the WB14/WB11 exchange diagnostics landed in
`91c12848` (`frost.runtime_frwatc_*`), and the WAT publication
(`frozwt` = Σ layer ice per `watbalprint.for` lineage; `frdp` from the
layered front). State which existing symbols/diagnostics survive, which are
re-bound, which retire. The v152 additive closure identity
(`Total-Soil + frozwt`) is the gate this seam must preserve.

### 5. Tests to author first (red, before implementation)

Contract-derived test list with named cases — minimum one each:

1. **State handoff** — `frwatc(1)`-equivalent splits WB liquid into sublayer
   state and `frwatc(0)`-equivalent reassembles it; round-trip conserves
   mass to noise.
2. **Freeze front stepping** — sustained surface heat loss advances the
   front sublayer-by-sublayer, consuming latent heat against sublayer water
   content; advance rate bounded by `Σ(dz/k)` resistance (deeper = slower).
3. **Already-frozen-water freezing** (`frznw` lineage) — refreeze of
   thaw-zone water above an existing front; no double-counting of ice.
4. **Bottom thaw** (`mltbtm`) — `Quf` retreats the front from below; mass
   returns to liquid; depth decreases.
5. **Top thaw** (`mlttp`) — surface energy melts from above creating a
   thawed layer over frost; `fgthwd` completion clears state.
6. **Closure identity preservation** — the v152 additive identity holds at
   noise through multi-day freeze/thaw cycling on a realistic profile
   (years-2–6-grade gate, per addendum 3).

### 6. Sizing and phase recommendation

Honest size estimate for the implementation pass; if it exceeds one pass,
the proposed phase line — noting the only legitimate line per the package
Branch-out is one where the landed phase still closes the depth+duration
gap (cap-free depth in the legacy envelope, duration delta collapsing from
−428/−518 toward zero).

## Hard boundaries (this phase and the implementation pass it scopes)

- **No coarse scalar or coarse per-layer substitute.** v57 + two recorded
  failures (`efd2152b`, addendum 2e) close that route. A third
  simplification attempt is grind, not scoping.
- **Conservation is non-negotiable:** years 2–6 additive-identity closure
  at ≤ ~3e-11 mm must survive every increment; the year-7 boundary watch
  item (addendum 3) must be explained or eliminated by the port.
- **No comparator-match tuning** (ADR-0017): legacy depth/duration is the
  envelope flag; the heat-flow contract is the authority.
- FQ-4 activation gate, kfactor magnitude, forest `ksatadj`, MOFE remain
  protected per `package.md`.

## Acceptance for this scoping phase

Sections 1–6 filled with source-traced content (`Static:` labels, file:line
citations against the pinned baseline); the legacy↔openWEPP alias table
present; the test list concrete enough to write red tests directly; sizing
recommendation stated. No production code edits. On completion, this
artifact becomes required-reading item 1 for the implementation pass.
