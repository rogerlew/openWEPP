# MOFEFID-D9 - D-val Disposition

Status: **EXECUTED-COMPLETE** (scaffolded 2026-07-05;
executed 2026-07-05). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: `SC-OFEROUTE-001#INV-OFEROUTE-011`.

## Objective

Close the non-numerics D-val acceptance surface for Lane D. This package
must re-run and adjudicate Papanicolaou Cases 1-3 after the D8 corrections,
execute the deferred Zone 1/Zone 2 stream-power taxonomy, and write an exact
Case-4 acceptance handoff for `GAP-OFEROUTE-005` / D10.

The package may update contracts, D-val harness code, diagnostics, and tests
needed to make those verdicts executable. It must not implement production
activation, default activation, D10 shock-numerics corrections, D11 friction
operand sourcing, D12 melt-limb source coverage, D13 erosion hourly-shape
switching, or the D14 routing flip.

## Rationale

`INV-OFEROUTE-011` currently blocks Lane D activation because its D-val
status is partial. D8 corrected the skin-intensity convention and sampled
hydrograph metric, then left Cases 1-3 as operand/comparator-surface
dispositions and Case 4 as the `GAP-OFEROUTE-005` shock-numerics boundary.
Before any production routing activation package can proceed, the
non-numerics parts of `INV-OFEROUTE-011` need a clean, current, reviewed
ledger:

- Cases 1-3 either satisfy the contract or carry named,
  contract-backed dispositions.
- Zone 1/Zone 2 taxonomy is actually executed or formally held with
  evidence.
- Case 4 has a precise handoff to D10, so D10 owns only the shock/numerics
  closure and not a vague D-val residue.

## Scope

### Included

- Amend `SC-OFEROUTE-001` only as needed to record D9 verdicts,
  evidence classes, acceptance surfaces, and handoff boundaries.
- Re-run or extend the D-val harness for Cases 1-3 using the D8-corrected
  code path and the existing supplemental-derived fixtures.
- Execute the Zone 1/Zone 2 stream-power taxonomy using the in-repo
  supplemental-derived experiment surfaces, or record a legitimate hold if
  the needed observed/derived data is unavailable or ambiguous.
- Add contract-derived tests or harness checks that make the D9 verdicts
  regression-visible.
- Record exact Case-4 acceptance text for D10: what remains blocked by
  `GAP-OFEROUTE-005`, what evidence D10 must produce, and what D9 already
  closed.
- Preserve copyright governance: do not vendor raw workbook rows or full
  hydrograph series beyond existing approved fixtures.

### Excluded

- No production runtime activation or default activation.
- No changes to direct-runtime publication, `OPENWEPP_LANED_SHADOW`, or the
  active routed-water flip.
- No D10 shock-numerics implementation or numerical-method tuning.
- No D11 friction operand sourcing/default decisions beyond recording
  Case-2/Case-3 operand implications.
- No D12 melt-limb hourly-shape implementation.
- No D13 ADR-0036 erosion hourly-shape switch.
- No watershed/channel routing changes.

## Dependencies

- `SC-OFEROUTE-001` current rev 16.
- `docs/planning/mofe-fidelity-campaign-strategy.md` section 6.1 D9-D15.
- D8 package:
  `docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/`.
- Lane D seam packages:
  `20260705-mofefid-laned-seam-implementation-001/` and
  `20260705-mofefid-laned-activation-increment-001/`.
- Papanicolaou 2018 references and supplemental-derived fixtures already in
  the repo under the existing copyright governance posture.

## Intended Write Set

Primary:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260705-mofefid-d9-dval-disposition-001/`
- `docs/planning/mofe-fidelity-campaign-strategy.md` only if package status
  or sequencing text changes.
- `docs/work-packages/README.md`

Conditional, only if required by execution:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/examples/dval_case.rs`
- `tools/dval/compare_dval.py`
- Focused D-val tests in the same crate or integration-test area.

Protected:

- No production direct-runtime activation files.
- No HBP/pass/watershed output schema changes.
- No raw copyrighted workbook/source document additions.

## Phase Plan

1. **D9-S0 - Intake and baseline.** Read required sources, record the
   current `INV-OFEROUTE-011` status, capture baseline D8 metrics for
   Cases 1-4, and verify no production activation path is touched.
2. **D9-S1 - Case 1-3 disposition.** Re-run or reconstruct the D-val
   metrics after D8 and classify each case as `satisfies`, `operand-limited`,
   `comparator-surface boundary`, or `contract gap`, with evidence.
3. **D9-S2 - Zone taxonomy.** Execute the Zone 1/Zone 2 stream-power
   taxonomy or record a hold with the exact missing data/authority and a
   defect-shaped follow-on.
4. **D9-S3 - Case 4 handoff.** Reduce Case 4 to a precise D10 obligation:
   shock-numerics authority, convergence criteria, Iwagaki evidence, and
   H2637 resolution-sensitivity reproduction.
5. **D9-S4 - Contract/tests/artifacts.** Amend `SC-OFEROUTE-001`, add or
   update tests/harness checks, record gate evidence, and update package
   artifacts.
6. **D9-S5 - Reviews and disposition.** Complete dual reviews, disposition
   findings, dual verification, line-count governance, and final status.

## Exit Criteria

- `INV-OFEROUTE-011` non-numerics status is current and non-contradictory in
  `SC-OFEROUTE-001`.
- Cases 1-3 have explicit verdicts with `Static:` and/or `Ran:` evidence.
- Zone 1/Zone 2 taxonomy has either `Ran:` evidence or a legitimate
  evidence/authority hold with the first actionable follow-on.
- Case 4 is handed to D10 with exact acceptance obligations; D9 must not
  leave a vague "investigate shock" note.
- Any contract edits have changelog rows, guard-map/BEI implications updated
  where needed, and contract-derived tests or harness checks.
- No production activation, default activation, shadow-to-active switch, or
  routed-water consumer switch occurs.
- Dual review findings are dispositioned as `accepted`, `rejected`,
  `deferred`, or `follow-up`; accepted findings are fixed before closure.
- Line-count governance is recorded for every touched `.rs` file.

## Required Gates

Selection follows `docs/standards/local-ci-gate-selection.md` if available,
but the package cannot close without recording:

- `git diff --check`
- Markdown lint for touched docs.
- Contract/profile/BEI checks for changed `SC-OFEROUTE-001` surfaces.
- Focused D-val tests/harness commands that exercise Cases 1-3 and Zone
  taxonomy.
- Focused Rust tests for touched `ofe_routing` / D-val surfaces.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings or authority
  suite posture are touched.

If a heavy gate is delegated to a subagent, record the subagent output and
log path in `artifacts/gate-results.md`.

## Conservation / Output Acceptance

D9 is validation/adjudication, not a publication or conservation-output
producer. If execution adds or changes any conservation-sensitive output,
diagnostic, or aggregate surface, the executor must add an operand-lineage
table before edits and satisfy the conservation/publication acceptance rule in
`docs/work-packages/AGENTS.md`.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` subagents for read-only review, verification,
source/harness inspection, and heavy D-val/full-gate execution. Expected
outputs are compact findings, gate metrics, and package-local review or
verification artifact text. Write access is read-only unless a later operator
explicitly assigns a bounded write set.

Subagent requirement: comparator-style D-val batch runs, full workspace
nextest, and other heavy closure gates should be delegated to
`comparator_suite_runner` when available. If unavailable, record the tool
block and run locally.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/dval-case-disposition.md`
- `artifacts/zone-taxonomy-evidence.md`
- `artifacts/case4-d10-handoff.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`

## HOLD Legitimacy

D9 may close in `HOLD` only for a specific evidence or authority boundary:
missing/ambiguous supplemental-derived data, contradictory contract authority,
or a mechanism proven to belong to D10-D15. A hold must name the boundary,
cite evidence, list the in-scope correction route considered, and state why
that route cannot close in D9.

## Progress

- [x] 2026-07-05: Package scaffolded from the MOFEFID §6.1 D9 row.
- [x] 2026-07-05T22:25Z: Executed D9-S0 intake and baseline from D8 artifacts,
      `SC-OFEROUTE-001` rev 16, and package-local required reading.
- [x] 2026-07-05T22:33Z: Executed D9-S1 Case 1-3 disposition via
      `compare_dval.py` reruns and Case 2 `Ks=10 mm/h` sensitivity.
- [x] 2026-07-05T22:36Z: Executed D9-S2 Zone taxonomy from supplemental
      Figure 9 using the copyright-safe `tools/dval/zone_taxonomy.py` harness.
- [x] 2026-07-05T22:40Z: Executed D9-S3 Case 4 handoff to D10 /
      `GAP-OFEROUTE-005`.
- [x] 2026-07-05T22:42Z: Executed D9-S4 contract/tests/artifacts:
      `SC-OFEROUTE-001` rev 17, registry date, taxonomy harness, and required
      package evidence artifacts.
- [x] 2026-07-05T23:24Z: Completed D9-S5 reviews, verification, and
      disposition. All accepted findings are closed; final disposition is
      `EXECUTED-COMPLETE`.

## Surprises & Discoveries

- Figure 9 uses workbook label `Clods` for the isolated-roughness taxonomy
  block. The D9 harness maps this to the contract's isolated-roughness class.
- Bare roughness has only one sub-threshold workbook intensity grid point under
  the published `I*=0.16` threshold, so D9 checks bare taxonomy through
  threshold support and Zone 2 near-linearity rather than a Zone 1 fit.

## Decision Log

- Decision: D9 is scoped as validation/adjudication, not production activation.
  Rationale: `SC-OFEROUTE-001` activation remains blocked by D9-D13; bundling
  D-val disposition with D14 would hide unresolved acceptance surfaces inside
  a runtime flip.
  Date/Author: 2026-07-05 / Codex.
- Decision: Add a small Figure 9 taxonomy harness instead of recording manual
  spreadsheet calculations only.
  Rationale: The package requires focused harness commands for Zone taxonomy,
  and the script can verify sha256 and emit scalar summaries without vendoring
  copyrighted workbook rows.
  Date/Author: 2026-07-05 / Codex.
- Decision: Amend `SC-OFEROUTE-001` rev 17.
  Rationale: D9 changes the canonical status of `INV-OFEROUTE-011` by moving
  Zone taxonomy from deferred to executed and narrowing the open blocker to
  Case 4 / `GAP-OFEROUTE-005`.
  Date/Author: 2026-07-05 / Codex.

## Outcomes & Retrospective

- D9 closes the non-numerics `INV-OFEROUTE-011` surface. Cases 1-3 retain
  named dispositions, Zone taxonomy is executed and passing, and Case 4 is
  isolated to D10 / `GAP-OFEROUTE-005`.
- No production/default activation, `OPENWEPP_LANED_SHADOW` activation, D10
  shock-numerics implementation, D11 friction sourcing/default promotion, D12
  melt-limb work, D13 erosion-shape implementation, or surrogate process
  physics was added.
