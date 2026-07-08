# WA Sediment Reference Adequacy Attribution

Status: `EXECUTED-HOLD-SEDIMENT-METRIC-AUTHORITY`
Evidence mode: Ran.
Date: 2026-07-08

## Objective

Attribute the `wa_cascades_forest_h1` refined-75 fine-reference annual
pass-sediment adequacy miss before any renewed `dx5` production mesh-policy
promotion.

The triggering miss is from
`20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001`:
`dx2p5_dt75` versus `dx1p25_dt75` has annual pass-sediment max relative
delta `0.022131684` on `tdep:4`, exceeding the one-third adequacy threshold
`0.0066666667`. The same comparison passed routed outlet, routed hourly shape,
end-window storage, tail-fold, uniform-shape, degenerate-shape, and closure
surfaces.

## Scope

In scope:

- Scaffold package-local evidence and prompt artifacts.
- Replay the existing WA refined-75 ladder outputs from the prior coupled
  space-time package.
- Confirm the failing `tdep:4` annual pass-sediment surface.
- Compare the implicated daily pass-sediment deltas against routed-water trace
  surfaces:
  - terminal routed outlet mass,
  - routed hourly shape,
  - end-window storage,
  - tail fold,
  - clamp/closure evidence,
  - uniform-shape and source-shape-degenerate counters.
- Classify the miss as one of:
  - sediment response to a sub-threshold routed-hydrograph shape perturbation,
  - routed-water timing/magnitude sensitivity,
  - active-router numerics.
- Record whether the evidence supports a contract-first follow-on or a
  mechanism hold.

Out of scope:

- No production `dx5` mesh-policy promotion.
- No active mesh default flip.
- No sediment or routed-shape tolerance widening.
- No SC-OFEROUTE-001 amendment unless the attribution proves an
  authority-backed mechanism-specific metric correction.
- No rerun of the full selected-cohort ladder unless existing artifacts are
  missing or inconsistent.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/mesh-policy-ratification.md`

Conditional:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  rev 43 sections for active mesh-policy judged surfaces, routed-hydrograph
  erosion shape, and rev-43 coupled space-time evidence.
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/mesh-policy-ratification.json`
  for exact comparator values.
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/coupled-spacetime-summary.json`
  for run provenance, hashes, and existing trace-summary surfaces.

On demand:

- D13 routed-hydrograph erosion-shape package artifacts sufficient to verify
  the active erosion consumer obligations:
  `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/`.
- D15A active-owner artifacts sufficient to verify routed-hydrograph consumer
  proof:
  `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/`.

## Phase Plan

### Phase A - Scaffold and Authority Map

- Create package-local `package.md`, `artifacts/`, `prompts/active/`,
  `prompts/archived/`, and package-local catalog entries.
- Record required-reading disposition and exact evidence sources.
- Record subagent authorization in the kickoff prompt.

### Phase B - Surface Confirmation

- Confirm `wa_cascades_forest_h1` `fine_reference_adequacy_dt75` fails on
  `tdep:4`.
- Record candidate/reference rung identities:
  - candidate: `dx2p5_dt75`
  - reference: `dx1p25_dt75`
- Record run provenance and pass parquet/trace file hashes.

### Phase C - Daily Attribution

- Decompose year-4 daily `tdep` deltas.
- Compare top sediment-delta days against routed-water deltas from the active
  trace and pass-parquet water surfaces.
- Check whether the annual miss is explained by:
  - material routed-water outlet magnitude/shape divergence,
  - a small set of erosion-response days with water deltas below mesh-policy
    tolerances,
  - active-router guard/counter/closure failures.

### Phase D - Disposition

- If active-router numerics are implicated, stop at hold with the exact trace
  evidence and first follow-on package/action.
- If routed-water timing/magnitude is implicated outside current routed-water
  thresholds, stop at hold unless the needed correction is contract-authorized
  and in envelope.
- If the miss is a sediment response to a sub-threshold routed-hydrograph shape
  perturbation, record whether the first follow-on is a contract-first sediment
  metric/tolerance package or an erosion-water-magnitude coupling package.
- Do not promote `dx5` in this package.

## Conservation and Output Acceptance

Annual pass-sediment is a conservation-sensitive published output surface.
This package must:

- identify the exact pass-parquet operand (`tdep`, year 4),
- separate annual aggregation from daily contributors,
- compare against independent routed-water trace surfaces rather than relying
  on self-consistency,
- reject tolerance changes that merely fit the failing value,
- preserve exact provenance for the release binary and run artifacts used.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation for read-only
review and verification. Authorized roles:

- review: inspect attribution logic, classification, and contract posture.
- verification: independently check replay commands, hashes, and gate claims.

Expected outputs are package-local `artifacts/review-*.md` and
`artifacts/verification-*.md`. Write access is bounded to this package's
artifact directory unless the operator explicitly expands scope.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/surface-confirmation.md`
- `artifacts/wa-sediment-attribution.json`
- `artifacts/wa-sediment-attribution.md`
- `artifacts/classification.md`
- `artifacts/implementation.md`
- `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required:

- `python -m py_compile artifacts/analyze_wa_sediment_reference.py`
- replay analyzer command recorded in `artifacts/gate-results.md`
- `git diff --check`
- Markdown/doc lint for touched docs

Conditionally required:

- Contract/profile/BEI checks if `SC-OFEROUTE-001` or another `SC-*` contract
  is amended.
- Focused Lane D / `ofe_routing` tests if Rust code changes.
- Full Rust gate suite only if production Rust code or contract bindings are
  changed:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`

## Exit Criteria

`EXECUTED-COMPLETE-ATTRIBUTED`:

- Failing annual pass-sediment surface is confirmed.
- Daily attribution and routed-water comparison are replayable.
- Mechanism classification is recorded with evidence.
- Follow-on package/action is precise.
- No production flip or tolerance widening lands.

`EXECUTED-HOLD-*`:

- Attribution cannot distinguish the mechanism with existing evidence, or the
  implicated correction is outside this package envelope.
- Hold legitimacy audit names exact blocker, evidence, in-envelope routes
  considered, and first actionable follow-on.
