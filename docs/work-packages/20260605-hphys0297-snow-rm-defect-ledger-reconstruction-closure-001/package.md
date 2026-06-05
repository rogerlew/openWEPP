# HPHYS0297 Snow/RM Defect Ledger Reconstruction Closure

Status: executed-hold

This work package is an autonomous execution plan and must remain a living
document during execution. It follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Objective

Convert HPHYS0296 H1/H7/H39 snow/`RM` residual candidates into an auditable
defect ledger. For each target window, identify the openWEPP and
`/workdir/wepp-forest_260430_baseline` source-line mechanisms, run a controlled
negative-melt reconstruction against openWEPP traces, and assign exactly one
verdict: `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, or `UNRESOLVED`. Do not
re-tier any residual or patch WB17/WB18/WB19/WB13 unless the defect ledger proves
the required verdict.

## Rationale

HPHYS0296 proved that six first-2013/spring-2014 windows contain material
negative raw hourly melt but did not prove causality. Review disposition
correctly rejected correlation plus internal closure as acceptance authority.
HPHYS0297 must replace the loose candidate bucket with reconstruction evidence:
the pinned baseline branch uses `1 - ngtvML/pstvML`, while corrected openWEPP
uses the physically conservative net-melt branch. If the pinned-baseline branch
does not reconstruct the comparator residual to a named tolerance, the window
remains `UNRESOLVED` even when negative melt is present.

## Progress

- [x] Scaffold package and autonomous prompt.
- [x] Amend canonical `SC-*` defect-ledger authority.
- [x] Add contract-derived test.
- [x] Run pre-implementation contract gate.
- [x] Run full H1..H39 metrics and H1/H7/H39 target traces.
- [x] Run per-window reconstruction ledger.
- [x] Run validation gates.
- [x] Update review, verification, disposition, and handoff artifacts.

## Surprises & Discoveries

- Observation: The pinned-baseline negative-melt sign/scale branch alone does
  not reconstruct any of the six HPHYS0296 corrected-negative-melt candidate
  windows to the named `2.000 mm` tolerance.
  Evidence: HPHYS0297 ledger reconstruction residuals range from `-4.997207`
  to `-45.281781 mm`.
- Observation: All three spring-2016 target windows still have immaterial
  negative raw melt.
  Evidence: negative raw melt is only `-0.224814` to `-0.255930 mm`, while
  observed baseline-candidate `RM` residual is `15.276407` to `16.885426 mm`.
- Observation: Full H1..H39 metrics did not move from HPHYS0296.
  Evidence: semantic pass remains `0/39` and `Q` remains `39/39`.

## Decision Log

- Decision: Scope HPHYS0297 to defect-ledger evidence, not production patching.
  Rationale: HPHYS0296 review made reconstruction and independent correctness
  mandatory before any residual leaves the failing set.
  Date/Author: `2026-06-05` / `Codex`.
- Decision: Keep every H1/H7/H39 target window `UNRESOLVED`.
  Rationale: no window has both reconstruction closure and independent
  defective-model verdict evidence; accepting any candidate would recreate the
  HPHYS0296 review defect.
  Date/Author: `2026-06-05` / `Codex`.
- Decision: Do not patch WB17/WB18/WB19/WB13.
  Rationale: full-suite `Q` remains closed and HPHYS0297 evidence still points
  to unresolved snow/winter producer authority rather than downstream hydrology
  compensation.
  Date/Author: `2026-06-05` / `Codex`.

## Outcomes & Retrospective

HPHYS0297 implemented the defect-ledger gate and ran the full H1..H39 suite plus
targeted H1/H7/H39 reconstruction diagnostics. The package did not re-tier any
residual: all nine target windows remain `UNRESOLVED`. The important result is
negative: the rejected pinned-baseline `1 - ngtvML/pstvML` branch does not by
itself reconstruct the baseline `RM` windows to tolerance, so the remaining
snow/`RM` residuals require deeper winter producer lineage work rather than
acceptance or downstream compensation.

## Included Scope

- Amend canonical `SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`, and
  `SC-WATBAL-001` with HPHYS0297 defect-ledger and reconstruction authority.
- Add a static contract-derived HPHYS0297 test.
- Reuse HPHYS0296 trace surfaces for H1/H7/H39.
- Run full H1..H39 semantic metrics.
- Build a target-window ledger that records:
  - openWEPP corrected source line,
  - `/workdir/wepp-forest_260430_baseline` source line,
  - observed candidate/baseline `RM` residual,
  - reconstructed pinned-baseline branch delta,
  - reconstruction residual to named tolerance,
  - independent correctness rationale,
  - final verdict.

## Excluded Scope

- Do not patch WB17 `Ep`/`Es`.
- Do not patch WB18 percolation or aggregate `watcon`.
- Do not patch WB19 lateral flow.
- Do not patch WB13 aggregate storage or `RM` publication as compensation.
- Do not reproduce the `/workdir/wepp-forest_260430_baseline` negative-melt
  sign/scale branch as target runtime physics.
- Do not silently re-tier or remove comparator rows.

## Deliverables

- Canonical contract amendments and revision-history entries.
- Contract-derived test
  `tests/integration/hphys0297_snow_rm_defect_ledger_contract.rs`.
- Per-window defect ledger artifact.
- Reconstruction evidence artifact.
- Full H1..H39 metrics artifact.
- Gate evidence, owned-file manifest, review/verification artifacts,
  disposition, and worker handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/review-disposition.md`
- `docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest/src/winter.for`

## Intended Write Set

- `Cargo.toml`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/README.md`
- `tests/integration/hphys0297_snow_rm_defect_ledger_contract.rs`
- `docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/**`

## Phase Plan

1. Contract authority: add HPHYS0297 defect-ledger requirements to canonical
   `SC-*` files.
2. Contract-derived test: add a static guard proving the defect-ledger criteria,
   source-line provenance, and baseline path requirements remain present.
3. Pre-implementation gate: run the HPHYS0297 contract test before diagnostics.
4. Diagnostics: run full H1..H39 metrics and H1/H7/H39 target traces; compute
   pinned-baseline branch reconstruction against openWEPP trace fields.
5. Disposition: assign per-window verdicts, record holds, and prohibit
   downstream compensation.
6. Review/verification: update placeholders and disposition artifacts; dual
   review remains required before final closure.

## Contract-First Sequence

1. Implement required contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Modify production code only after diagnostic ownership is proven.

## Exit Criteria

- Contracts and contract-derived tests exist and pass.
- Full H1..H39 metrics are recorded.
- Every target H1/H7/H39 window has a ledger row with reconstruction residual
  and `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, or `UNRESOLVED` verdict.
- No residual is re-tiered without reconstruction and independent correctness
  evidence.
- No downstream compensation patch is applied.
- Package remains `executed-hold` unless dual review and verification close.

## Security-Impact Gate

No external systems, credentials, network calls, or shell interpolation are
required. Work is local flat-file reads/edits plus local test and diagnostic
commands.
