# Pre-Execution Intent

Status: `frozen`.

Evidence class: Ran + Static.

## Exact Intake

- Execution date: `2026-07-30`.
- Working directory: `/home/workdir/openWEPP`.
- Scaffold/base commit:
  `31e14bdf23ab10dd06ce38a28d4897521f2490c1`.
- Base state: clean immediately after the scaffold commit.
- Branch: `main`; no branch creation or switch.
- Science intent: authority reconciliation, calibration-readiness assessment,
  independent-validation design, and four-cell factorial pre-registration.
- Implementation intent: documentation and package-local deterministic
  analysis only. No production, fixture, contract, schema, selector, or default
  edit.

## Intended Write Set

- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260729-snow-surface-eb-01-reconciliation-factorial-design-001/**`

All Rust, tests, fixtures, canonical contracts, reference objects, assurance
sources, and external repositories are protected read-only inputs.

## Validation Selection

Preselected documentation/analysis increment checks:

- run package-local generator/checker determinism;
- parse every CSV with `.venv/bin/python`;
- parse every SVG as XML and require one title, one description, and
  `role="img"`;
- require a same-stem Markdown sidecar for every figure;
- validate local Markdown links;
- run `markdown-doc lint` and `markdown-doc validate` on the package, campaign
  roadmap, canonical roadmap, and catalog;
- preview `uk2us` normalization on new prose;
- run `git diff --check`;
- reconcile the exact base-to-terminal and worktree diff with this write set.

Rust formatting, Clippy, Nextest, cargo-deny, comparator, empirical rerun, and
campaign-closure gates are `NOT APPLICABLE` to this documentation-only
increment unless the terminal diff discovers executable, contract, fixture, or
generated-runtime impact. Such discovery requires prospective package
amendment before the edit.

## Observation Role Procedure

No empirical calibration is authorized. Installed measured observations are
assigned before any future result-bearing execution as either:

- `INDEPENDENT_VALIDATION` when source custody, model stratum, observation
  stratum, units, and operator are bound; or
- `DIAGNOSTIC_ONLY` when a model stratum is unbound, a required observation is
  absent, or the evidence provides only a protected no-regression context.

No dataset will be assigned to both roles.

## Risk And Security

Risk classification: editorial/characterization. The package changes
prospective planning and evidence only and cannot change executable behavior.

Security-impact gate: `NOT APPLICABLE`. No parser, external-input ingestion,
trust root, protected-data custody, secret, publication, or release change is
authorized.
