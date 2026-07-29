# CANOPY-CAL-06 Canopy-Gradient Congruence

Status: `complete`

Evidence mode: `Ran + Static`

## Objective

Evaluate the frozen CAL-04B 37-member GSI timing ensemble across the
source-supplied Marcell, Harvard, and Hubbard Brook within-site canopy
gradients. Preserve CAL-05 source incompleteness and classify canopy,
interception, snow, litter/residue, frost, runoff, and erosion cells without
downstream compensation or unsupported attribution.

## Intent

This package is a characterization and independent-gradient evaluation. It
does not perform empirical calibration, parameter refitting, independent
validation of CAL-04B transferability, or production-physics implementation.
Under ADR-0042, calibrated GSI timing remains
`IMPLEMENTED / EMPIRICALLY_CALIBRATED / PARTIALLY_IDENTIFIABLE`; later canopy
amplitude operands remain calibration-ready and data-limited. Predictive
needle and fine-woody sources remain
`AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`.

## Included scope

- Freeze the seven native forest and two open-control lanes already installed
  by CAL-03.
- Preserve all 37 CAL-04B accepted GSI members; do not select a best member.
- Run each forest/member pair through the real direct-production executor.
- Run the two open controls once through the same executable and protected
  fixture bindings.
- Digest-bind ephemeral daily canopy research traces and WAT outputs; retain
  compact tidy daily climatology, per-run/per-year reconstruction operands,
  run, observation-score, and manifest objects in the package.
- Evaluate prespecified seasonal canopy ordering and amplitude, snow
  accumulation/peak/density/melt-out, litter/source completeness, residue,
  frost availability, interception, runoff, and erosion advancement.
- Produce the six plot-only human-interpretation views required by the CAL-06
  figure contract, each with a Markdown caption/ancillary-information sidecar
  that preserves null and `NOT ADVANCED` semantics.

## Excluded scope

- No GSI or canopy refit after Harvard holdout opening.
- No invented needle, fine-woody, decomposition, canopy, snow, frost,
  hydrology, or erosion physics.
- No production Rust, canonical science-contract, fixture, or observation
  edits unless execution exposes a contract-backed defect that cannot be
  characterized without correction.
- No Southern Hemisphere claim; CAL-07 owns that boundary.
- No assurance publication claim; CANOPY-ASSURE-01 owns publication.

## Authority and dependencies

- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/planning/canopy-cal-06-figure-contract.md`
- `docs/decisions/0042-science-implementation-and-calibration-readiness.md`
- `SC-PLANT-001` invariants 033--039
- `SC-RESIDUE-001` native litter/residue invariants
- `SC-SNOWFREEZE-001` daily-canopy and dynamic-residue consumer authority
- CAL-03 fixture/output identities
- CAL-04B frozen accepted ensemble
- CAL-05 and CANOPY-LITTER-SOURCE-AUTHORITY-01 source-completeness verdicts

## Intended write set

- This package directory.
- `docs/work-packages/README.md`.
- `docs/planning/canopy-phenology-assurance-roadmap.md`.
- `docs/planning/canopy-cal-06-figure-contract.md` only to mark fulfilled
  result bindings.

Production code, canonical contracts, fixtures, and observations are protected.

## Phase plan

1. Freeze intent, cell definitions, protected inputs, result schemas, and
   source/missing semantics.
2. Implement deterministic package-local execution, analysis, validation, and
   accessible SVG rendering tools.
3. Build the exact runner and execute all 261 prespecified lanes
   (259 forest/member plus two open controls).
4. Independently reconstruct summaries and classify all cells.
5. Render and reproducibly validate all required human-interpretation figures.
6. Run source-quality, focused behavior, contract, package, and exact-diff
   checks.
7. Complete two independent reviews, disposition every finding, then complete
   two independent verifications.
8. Reconcile exact diff, line counts, roadmap/catalog, and final disposition.

## Contract-first and no-surrogate posture

No new production physics is intended. If execution requires a production
science change, stop before that edit and amend the applicable canonical
contract, add contract-derived tests, record the pre-implementation gate, and
only then edit production code. No surrogate, provisional, proxy, or heuristic
physics may enter production.

## Prespecified advancement logic

- Canopy cells may be `SUPPORTED`, `BOUNDED`, `CONTRADICTED`, or
  `NOT_EVALUATED`.
- Snow cells separate model response from observed agreement and retain
  canopy-independent residuals.
- Litter cells preserve leaf, needle, fine-woody, and total sources separately;
  predictive missing sources are null, never zero.
- Frost, ET, runoff, and erosion consequences advance only when their named
  upstream cells pass. Otherwise their result is retained but labeled
  `NOT_ADVANCED`, never treated as zero or hidden.
- Ordering alone cannot support amplitude or observational agreement.

## Validation requirements

- Deterministic input/result manifests and digest verification.
- Complete 37-member inventory for each of seven forest lanes and exactly one
  run for each open control.
- Independent summary reconstruction from retained daily/output objects.
- Contract invariant/source-semantics checks and real-consumer lineage proof.
- Renderer determinism, SVG parse/accessibility checks, and figure-to-table
  bindings.
- Focused package tool tests, relevant Rust contract tests, Rustfmt/Clippy when
  Rust changes exist, and Markdown lint.
- Exact terminal diff reconciliation and `.rs` line-count governance.

No current-scope requirement may be deferred while the package is complete.
Any failed, blocked, or unjustified not-run requirement forces `HOLD`.

## Review and verification

Two independent scientific/code reviews must check authority, ensemble
retention, attribution boundaries, quantitative cell legitimacy, negative
evidence, figure integrity, and gate legitimacy. Every finding receives
`accepted`, `rejected`, `deferred`, or `follow-up` disposition; closure permits
no undispositioned finding.

Two independent verifiers must reproduce the terminal validator and at least
one summary/figure digest independently, then check the exact package
disposition.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only review agents and two read-only
verification agents for the scopes above; expected outputs are package review
and verification artifacts, and write access is limited to those four named
artifact files.

## Security-impact gate

No secrets, network credentials, user-controlled command interpolation, or
public interface changes are in scope. Execution uses fixed local fixtures,
explicit subprocess argument arrays, fresh bounded temporary roots, and
digest-bound outputs.

## Exit criteria

- All 261 planned runs are accounted for without hidden replacement or member
  selection.
- Every prespecified cell has a machine-readable verdict and rationale.
- All six required plot/sidecar pairs bind exact source tables and reproduce
  deterministically.
- CAL-04B uncertainty and CAL-05 missing-source semantics remain visible.
- Required validation, dual review, finding disposition, dual verification,
  exact-diff, and line-count gates pass.
- Roadmap and catalog truthfully record the CAL-06 result and limitations.
