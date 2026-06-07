# FROSTVAL01 Kickoff

Execution mode: package-end-to-end

Autonomy: execute end-to-end for the declared scope — enumerate the single-OFE
hillslopes, run `openwepp-cli-hill`, run the frost-activation audit then the
closure-under-frost audit, classify, produce all deliverables, dual
review/verification, disposition, and a defect-shaped handoff — without asking for
direction on intermediate steps. Ask only if hard-blocked.

## Item 1 — frost-activation gate first (do not skip)

Before reporting any closure-under-frost result, **prove frost is genuinely
active** on `/wc1/runs/al/algebraic-radium`'s single-OFE hillslopes: `ksflag = 1`
is honored, and frost depth / frozen soil water / `ksflag`-gated conductivity
differ from a no-frost baseline on cold days. A water balance that "closes"
because frost is silently inactive is a **false pass** — that is the specific
failure this gate exists to prevent. If frost does not activate, that is the
primary finding; emit it as the defect and defer the closure audit.

## Item 2 — closure under frost

Recompute the rung-1 complete-identity residual per single-OFE hillslope/year
**with frost engaged** (including `Interception`, `frozwt`/frozen storage, snow
terms) and run the totalwatsed3 audit. Confirm the closed rung-1 balance stays
closed within tolerance. Classify `frost-clean` vs `frost-break`.

## Acceptance authority

- Conservation closure with frost engaged (rung-1 identity + totalwatsed3 audit),
  NOT comparator match. `wepp_260606` is a flag (ADR-0017, ADR-0011).
- A `frost-break` population is the expected rung-2 DC-ExecPlan input, not a
  failure. Failure = an incomplete/untruthful ledger, or closure-under-frost
  reported without first proving frost is active (Item 1).
- Fail closed on missing terms; never impute zero to force closure.
- Truthful evidence mode: **Ran** for actual `openwepp-cli-hill` invocations,
  **Static** for reasoned classification.

## Hard constraints (protected boundaries)

- No production edits (validation/characterization); corrections are follow-on
  frost DC-ExecPlans.
- Snow magnitude → Stage-2 backlog, not in-package.
- The 17-OFE hillslope is MOFE → observe-only (rung-3); no routing closure here.
- Forest `ksatadj` path is out of scope; this validates the standard `ksflag`
  frost gate on Cropland.

## Required reading

- `docs/work-packages/20260608-frostval01-ksflag-frost-single-ofe-closure-validation-001/package.md`
- `docs/work-packages/README.md` (Current roadmap, rung-2)
- `docs/decisions/0011-...md`, `0017-...md`, `docs/defect_closure_execplans.md`
- `docs/backlog/20260605-snow-code-deferred-science-review.md` (snow protected boundary)
- Frost reference (legacy): `/workdir/wepp-forest_260430_baseline/src/infile.for`
  (`ksflag` read + `lanuse≠1` override), `infpar.for` (`ksflag`-gated
  conductivity), `winter.for` (frost depth).
- Precedent: WBVAL01/WBVAL04 (single-OFE closure validation shape),
  `tests/integration/cli01_runner_hillslope_integration.rs`,
  `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`.
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606`,
  `wepp_260606_hill`.
