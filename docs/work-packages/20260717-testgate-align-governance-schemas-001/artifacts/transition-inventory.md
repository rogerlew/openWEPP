# Transition Inventory

Evidence class: `Static`

## Aligned In TESTGATE-ALIGN-01

- Root, crate, test, and work-package `AGENTS.md` lifecycle wording.
- ExecPlan, prompt wording, kernel-package preparation, mechanical-refactor,
  module-test, CQR, local-CI, and Rust-scientific standards.
- CQR nightly package/prompt templates and recurring ExecPlan.
- Current contributor orientation/contributing guidance and the prospective
  watershed runtime architecture closure section, added through the recorded
  finding-driven write-set amendment.
- The only queued package found with duplicated legacy full-loop wording:
  `20260613-refactor022-mofe-scheduler-runner-watershed-line-count-split-001`.
  The other queued packages did not duplicate terminal full-gate wording.
- ADR-0021 cadence and correctness-authority lane wording.
- Versioned schema authority, seed impact map, fixtures, Cargo registration,
  and the source-level consistency guard.

Historical completed/held package prompts and evidence remain immutable
records of the rules and commands used at their execution time. They are not
prospective authority and were not bulk rewritten.

## Intentionally Retained Transition Behavior

The following remain conservative implementations until their ordered packages
land. Their current behavior is not independent policy:

- `.config/nextest.toml` profiles;
- `tools/local_ci/` timing and future receipt capture;
- `tools/release/run_adjudicated_crap_gate.sh` and its checker;
- `tools/release/run_release_candidate_gates.sh`;
- `.github/workflows/release-gates.yml`; and
- existing stable status contexts.

`TESTGATE-PLAN-01` owns mechanical change/Cargo/impact planning and receipt
verification in shadow mode. `TESTGATE-CI-01` owns affected execution,
coverage/CRAP, lane splitting, status migration, scorecard evidence, and
cutover/rollback.

## Assurance Follow-On

The assurance architecture, catalog, schemas, reports, locks, transactions,
planner/identity/assembly/publication code, release transition scripts, and
public/generated inventories remain unchanged. `TESTGATE-ASSURE-01` owns
registry-wide watch discovery, exact-head campaign impact folding, and release
transfer integration after planner identities stabilize.

## External Certification Follow-On

Provider-side rulesets, protected evidence branches/tags, the dedicated
publisher GitHub App, revocation state, atomic-push evidence, artifact
retention, crash recovery, and fresh-clone verification remain implementation
work. No local schema claims that those external controls currently exist.
