# Kickoff: Stage 3 Wind Source Custody And Exposure Authority

Scope: local openWEPP repository science-authority work; flat-file reads and
edits in `/home/workdir/openWEPP` only. External network reads are allowed only
to acquire primary public dataset metadata needed by the package; no external
system mutation is required or authorized.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through disposition.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260807-snow-stage3-wind-source-custody-and-exposure-authority-001/package.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md`, the contract-authoring
  procedure, kernel profile, science-contract index, `SC-SNOWENERGY-001`, and
  `SC-SNOWFREEZE-001` before any contract or kernel-authority edit;
- `crates/AGENTS.md` and nearest nested instructions before Rust reads that
  become edits;
- `tests/AGENTS.md` before test edits;
- `docs/standards/testing-and-gate-strategy.md` before final gate selection.

On-demand:

- predecessor disposition, wind custody, and worker handoff;
- actual GRIDMET/CLI generation paths and retained fixture manifests;
- primary GRIDMET, NLDAS-2, and Abatzoglou (2013) metadata;
- DRAFT assurance source records if canonical identified sources change.

Required-reading budget: `454777` local bytes for Core, `WARN`; the catalog is
required Core governance and accounts for most of the excess. Map:
`artifacts/required-reading-map.md`.

## Task

Execute the package objective end-to-end. Freeze result-blind authority and
decision rules; trace exact GRIDMET product/cell/cadence/aggregation into CLI
and the real Stage 3 consumer; separate nominal 10 m forcing, PMET-adjusted 2 m
wind, and virtual 5 m transfer geometry; independently reproduce the neutral
~10% friction-velocity and ~21% turbulent-product bound; and adjudicate each
site's forest-snow exposure applicability from authority.

Contract-first sequencing: amend canonical contracts, add contract-derived
tests, pass the clean pre-implementation gate, then make any separately
authorized implementation edit. Production Rust is not initially selected.

No surrogate physics: production code must implement only actual
contract-backed or baseline-authoritative physics. No provisional, proxy,
heuristic, fitted, or residual-minimizing wind/canopy formula is allowed.

No fitting: do not infer exposure from wind values, carrier residuals, or a
desired energy balance. Do not choose attenuation to make Stage 3 plausible.
Use `AUTHORITY_MISSING` when authoritative exposure evidence is absent.

Claim limit: a valid height conversion does not prove that 4 km gridded wind is
applicable to a sub-canopy snow surface. Do not license a canopy operator in
this package. Paradise WY2015 is a separately queued, non-blocking diagnostic.

Validation: declare pre-implementation intent, execute every directly selected
gate, reconcile the exact terminal diff, retain Static/Ran labels, and do not
defer an unmet current-scope gate while claiming completion.

Subagent requirement: REQUIRED for any selected heavy batch/closure run; spawn
the `comparator_suite_runner` and do not run heavy gates on the parent model
unless unavailable, with command-level evidence recorded. This prompt
explicitly authorizes subagent spawning/delegation to two independent read-only
science/Rust reviewers, two independent read-only terminal verifiers, and the
`comparator_suite_runner`; outputs are compact findings/metrics and log paths;
write access is read-only.

Autonomy: execute all package phases and update artifacts without requesting
additional user direction unless a genuine external-authority or scope blocker
is reached.

Outputs: complete package evidence, contract/test changes when required,
roadmap/catalog updates, prompt archival, assurance impact, reviews,
verification, disposition, and worker handoff.
