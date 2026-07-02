# A01 Sweep Notes

Working notes per surface (S1..S6). Findings graduate to `findings.md`
with dispositions. Evidence classes marked per claim.

## S3 — publication geometry (seeded from B01 evidence)

- `Q` and `QOFE` publication (`01_publication.rs:361-379`): `q = q_runoff ×
  efflen/cumulative_length`, `qofe = q_ofe × efflen/ofe_length` (pre-260516
  legacy convention — dispositioned as `MOFEFID-B02` contract decision, not
  an A01 defect). `runvol = QOFE × per-OFE area` — the legacy cancellation;
  self-consistent with the QOFE convention. Runner geometry gate blocks
  `efflen > cumulative_length + 1e-9` (`05_runner_execution_and_outputs.rs:204`).
- **Seed finding F-A3:** dormant `QcapSoftLimit` clamp-status taxonomy value
  (`sim-contract status.rs:113`) — defined, tested in the taxonomy test,
  never emitted by any producer. Dead taxonomy surface. [Static]
- Peak operands (`compute_r7d6_peak_runoff`, `runoff.rs:690-789`): floors +
  duration clamp; `efflen` used as kinematic timing length. No geometry
  duality smell found in the peak path itself (QOFE-basis pairing was
  verified in the T-B2 arc). [Static]

## S4 — per-OFE closure reconstructability (seeded from B01 evidence)

- **Seed finding F-A4 (gap, not defect):** no external tool can reconstruct
  per-OFE conservation from openWEPP's published WAT/PASS the way the
  wepp-forest external audit reads `H.wat`/`H.pass`. `owcmp` is per-column
  diff; snow tools read internal traces. `INV-WATBAL-096` itself warns that
  WB13 row aliases are structural checks, not conservation identities.
  The exported column set (P, RM, Q, Ep/Es/Er, Dp, UpStrmQ, SubRIn, latqcc,
  Total-Soil, frozwt, Snow-Water, …) appears *nearly* sufficient; whether
  it fully closes (interception flux column nullable; snow/frost storage
  deltas derivable from state columns) needs a worked example. The
  **independence property** — an audit computing a residual the model never
  sees — is the wepp-forest program's one unambiguous methodological win
  and openWEPP currently lacks it. Disposition direction: follow-up
  (external per-OFE closure audit tool, shaped by the B11 latqcc-day
  design constraint), candidate Lane C2 sibling. [Static]
- **Seed finding F-A5 (hardening):** R4B `closure_residual_m`
  (`storage.rs:823-843`) re-evaluates the assignment RHS minus itself —
  algebraically zero; catches only non-finite arithmetic. The substantive
  guards are nonnegativity + the projection ledger-vs-state pair
  (`projection.rs:179-204`). The naming invites false confidence in
  contract citations. [Ran — verified this package]

## Cross-references

- B10's `INV-SNOWFREEZE-015` net-algebra vs SNOWSCI-S1 positive-parts
  tension is dispositioned in B01 (contract-decision follow-up); not
  re-opened here.
- **Seed question F-A1 (S1):** `INV-RUNOFFPART-029` mandates the lane
  transfer state carry a "runoff-continuation/case-classifier outcome";
  the four-case classifier appears only in `erosion.rs:749-796` as a
  *validator* of an externally supplied `case_value`. Producer trace in
  progress — if no runoff-path computation exists, this is a spec-vs-code
  consistency finding. [Static, open]

## S1 — transfer lineage (swept)

Explorer sweep + my verification of the F-A1 chain. Mechanics confirmed
sound: per-day dynamic transfer publishes the upstream lane's outputs into
the downstream lane's inbox between commit and the downstream seed
(day-major, lane-ascending loop; `03_executor.rs:128-153`); inbox
overwritten daily (no accumulation defect surface); surface (slot-0 lump,
`q_runoff_m`) and lateral (24-slot hourly) carried separately; area ratio
`A_upstream/A_current` computed in the runner
(`05_runner_execution_and_outputs.rs:156`) and applied at R4J consumption
with a raw>0⇒ratio>0 guard; immediate-neighbor ratio is correct because
each lane's `q_runoff` already folds its own runon (cascade accumulates).
Topology adjacency validated at both the static ledger and the dynamic
publish. **No defect found** (consistent with FARPOINT01's three closed
identities at 19 OFEs).

**F-A1 (CONFIRMED — spec-vs-code divergence, within declared hold):**
`INV-RUNOFFPART-029` mandates the lane state carry a
"runoff-continuation/case-classifier outcome." What exists: `case_value`
is computed **once at seed time** from seed `qout = 0.0`
(`build_mofe03_wave2_case_scalars`,
`direct_seed_projections/02_mofe03_wave2_projection.rs:250` — case 2.0 for
positive seed qout, else 4.0, with **synthetic** companion scalars
`vj = 0.25·qout, qj = 0.5·qout, fh = qout, fp = 0.5·qout`), and the
runtime **never recomputes it** — the erosion span updates
qout/qin/peak/qostar but leaves `case_value` and its companions seeded
(`erosion.rs:386-436`), so `validate_erod14_case` (`erosion.rs:749-796`)
checks seeded values against seeded values every day. Mitigations
verified: (a) consumption is **validator-only** — no kernel branch selects
equations by `case_value`; (b) the water path never reads it; (c) this is
inside the declared `INV-RUNOFFPART-030` governance-hold (sediment
coupling not accepted), and the hold's manifest-labeling requirement is
met via `erod14_qin_source_policy` derived from the wave2 flag
(`05_runner_execution_and_outputs.rs:332,343`); (d) wave2 seed guards
fail closed (`02_output_and_climate_helpers.rs:887-930`). Disposition:
**recorded; routed to the INV-RUNOFFPART-030 hold-closure package** — the
real per-day case classifier is part of accepting MOFE sediment coupling
(and Lane D's hydrograph work supplies the missing hydraulic operands).
Evidence: Ran (all sites read this package).

## S2 — hourly carry arrays (swept)

Same-day working arrays, freshly zeroed per day inside `LateralRun`
(`subsurface.rs:1666`), populated per substep under
`mofe_hourly_carry_arrays_enabled` (= `lane_substeps == 24`, forced by
`contributor_ofe_count > 1`; `00_wb11_projection.rs:23`), consumed
same-day (saturation → R4L addback) or published downstream (lateral →
transfer inbox). No cross-day carry surface exists to leak. Single-OFE
daily runs keep them inactive with a manifest guard
(`openwepp-cli-watershed.rs:1823-1843`). **No defect found.** One note:
MOFE forces 24 subsurface substeps even on otherwise-daily lanes — an
intentional activation documented in the projection; behavior difference
single-vs-multi is by construction, not leakage.

## S6 — single-OFE specialization (swept)

No master `lane_count == 1` branch; single-OFE is a natural zero-upstream/
zero-downstream specialization: static ledger yields (0,0) received for
`upstream_lane_id == 0`; dynamic publish early-returns for
`downstream_lane_id == 0`; erosion `qin` zeroed for lane 0; wave2 and
carry arrays gated off; runner short-circuits per-OFE slicing. The
zero-feed design means MOFE machinery cannot perturb single-OFE results
except through the substep gate, which is itself single-OFE-inactive.
**No defect found.**

## S5 — winter column × MOFE (swept)

Explorer sweep + my verification of the two load-bearing sites (legacy
`watbal_hourly.for:377-420`; `runoff.rs:168,540-582`).

- **Per-lane independence confirmed:** no lane reads another lane's winter
  state anywhere; the only inter-lane channel is the four liquid scalars in
  `DirectTransferBuffers`. Snow/frost state never crosses lanes; no active
  drifting (consistent with `GAP-SNOWFREEZE-003`). Melt/rain reaches the
  downstream lane only as already-liquefied runoff.
- **F-A6 (positive + watch-item):** the FARPOINT01 `watbtm`-into-`Dp`
  double-count class is **structurally absent** — `watbtm`/`watpdg` are
  internal frost-partition closure terms with no arithmetic consumer
  besides the frost residual (`frost.rs:520-524`); storage couples frost
  through `frwatc_net_liquid_delta_m` only. Watch: the FDHP01 design
  artifact documents a `Dp += watbtm` identity that is *not wired* today;
  if that coupling is ever activated, re-audit this seam first. Note also:
  HBP frost provenance publishes from the outlet lane only
  (`05_runner_execution_and_outputs.rs:257-269`) — a convention, recorded.
- **F-A2 (CONFIRMED — source-intent divergence, fidelity-shaped):**
  **runon is excluded from the infiltration supply.** openWEPP WB14
  infiltration consumes the hyetograph only (rain + routed melt, frost-
  gated via effective conductivity; `runoff.rs:1315-1371`); runon enters
  afterward as a pure addition in the runoff partition
  (`runoff.rs:652-688`), so upslope water can only become runoff or be
  absorbed by depression/frost-retention — it can never re-infiltrate into
  downslope soil storage. Legacy hourly source does the opposite,
  **at both granularities** (**Ran**, verified in the pinned ADR-0024
  baseline `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
  and unchanged in current source at `:411-413`):
  daily supply `fin = fin + (ui_HUrunf·efflen(i−1) −
  ui_Hcrunf·efflen(i))/slplen(i)` (baseline `:361-363`), and hourly supply
  `xfin = fin/ui_LFtstpF + (ui_LfUrf(ii) + ui_SUrunf(ii)) ×
  (fwidth(i−1)·slplen(i−1))/(fwidth(i)·slplen(i))` (baseline `:471-473`) —
  the hourly upstream carry arrays, **surface and lateral both**, join the
  water available to infiltrate each hour. Consequences: on runon-bearing days
  openWEPP systematically over-routes surface runoff and under-wets
  downslope soils relative to legacy intent; downslope re-infiltration is
  the first-order process behind vegetated filter strips (Neibling &
  Alberts; Dermisis 2010), so this is central to MOFE fidelity even though
  the INV-RUNOFFPART-028 event-closure identity holds either way (it is a
  partition question, not a conservation question). The combined
  surface+lateral runon pool (`runon_input = UpStrmQ + SubRIn`,
  `runoff.rs:168`) is itself contract-ratified by INV-028. Scope note: on
  the wet-forest H2637 fixture surface runon is small (lateral-dominated),
  which is consistent with MOFE01/FARPOINT01/MAGPARITY01 closing without
  meeting this seam.
