# Workspace gate hold-lift

Status: `ACTIVE — INTAKE`

Execution mode: `package-end-to-end`

This is a living ExecPlan maintained under `docs/codex_exec_plans.md`.

## Objective

Lift the mandatory workspace-gate HOLD recorded by
`20260830-snow-stage3-cold-content-fixed-point-optimization-001` by obtaining a
passing warnings-denied workspace Clippy command and a passing complete
full-workspace correctness profile on one exact clean terminal source identity.

## Progress

- [x] 2026-08-30: authorize and scaffold the hold-lift package.
- [ ] 2026-08-30: freeze exact source identities and classify retained failures.
- [ ] 2026-08-30: correct warnings-denied and attributable correctness defects.
- [ ] 2026-08-30: execute exact-clean terminal mandatory gates.
- [ ] 2026-08-30: complete dual review, dual verification, disposition, and delivery.

## Observed defects

- `WGHL-CLIPPY-001`: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  exits 101 after `filter_map_bool_then` in `openwepp-coupled-time` and
  `similar_names` in `openwepp-biogeochemistry`; Cargo may expose further
  diagnostics after those are corrected.
- `WGHL-FULL-001`: `cargo nextest run --workspace --profile full` completes
  3,628 attempted tests with 3,503 pass, 96 fail, and 29 timeout. Three stale
  accepted-endpoint source scans are already corrected; 122 adverse outcomes
  remain unclassified against the fixed-point package baseline.

## Rationale

Focused solver, contract, authority, conservation, and canonical performance
evidence passes, but package governance prohibits completion while mandatory
campaign-strength workspace gates fail. This package owns the cross-workspace
classification and repair needed to replace that HOLD with direct passing
evidence. It does not treat pre-existing failures, retry, or location outside a
prior write set as an implicit waiver.

## Included scope

- exact-source failure inventory and baseline/current comparison;
- narrow warnings-denied source-quality corrections;
- fixture, orchestration, runner, CLI, assurance, and test-support corrections
  proven necessary by the complete profile;
- timeout diagnosis and deterministic resource/scheduling correction when the
  test's semantic obligation is preserved;
- direct focused tests followed by exact-clean full Clippy and full Nextest;
- review, verification, line-count, terminal-diff, and truthful disposition.

## Excluded scope

- numerical tolerance, conservation threshold, science-contract, constitutive
  physics, event chronology, custody, topology, receipt, rollback, or adaptive
  temporal-policy changes;
- deleting, ignoring, weakening, filtering, reclassifying, or relaxing a test
  merely to make the workspace profile pass;
- TESTGATE/planner repair, coverage/CRAP campaigns, dependency changes, release,
  publication, or unrelated performance optimization;
- treating a baseline failure as waived without an explicit authoritative
  correction or accepted lifecycle disposition.

## Correction authority envelope

The package may make behavior-preserving source-quality edits and correct
production/test infrastructure when a retained failure plus focused evidence
demonstrates the defect. Every Rust implementation path must be entered in
`artifacts/failure-inventory.md` with failure ID, owner, exact source/test path,
classification, intended correction, and focused command before editing. Any
finding that requires science, numerical-policy, public-output, serialization,
or contract-authority change is outside this envelope and requires an amended
package plus applicable canonical authority before implementation.

## Intended write set

- `docs/work-packages/README.md`;
- this package tree;
- `crates/openwepp-coupled-time/src/event.rs`;
- `crates/openwepp-biogeochemistry/src/lib.rs`;
- exact production/test paths under `crates/openwepp-assurance/`,
  `crates/openwepp-runner/`, `crates/openwepp-hillslope-orchestrator/`, and
  `tests/integration/` entered prospectively in the failure inventory after
  baseline/current classification;
- `.config/nextest.toml` only if measured timeout evidence proves a scheduler
  defect and semantic test duration/coverage remains unchanged.

## Required evidence

- source identities and retained-log hashes;
- complete deduplicated baseline/current failure inventories;
- causality classification for every current adverse outcome;
- focused before/after evidence for each correction family;
- exact-clean warnings-denied Clippy and full-workspace Nextest results;
- no-test-weakening audit, line-count reconciliation, terminal diff, dual
  independent review, and dual independent verification.

## Gate selection

Risk is `CRITICAL`: this package owns workspace-wide correctness execution and
may touch cross-domain orchestration/test infrastructure. Mandatory terminal
commands are:

- `nix develop -c cargo fmt --all -- --check`;
- `nix develop -c cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --workspace --profile full`;
- every focused command named by the prospective failure inventory;
- `git diff --check` and exact terminal write-set reconciliation.

Additional A0/A1/A3, anti-evasion, contract, conservation, consumer, restart,
or publication commands become mandatory if the exact diff touches their
owning surfaces. No faster profile substitutes for the selected full profile.

## Exit criteria

- warnings-denied workspace Clippy exits zero on the exact clean terminal source;
- every test started by the full profile passes on that same source, with
  skipped inventory separately reconciled against profile configuration;
- no test assertion, timeout, fixture, authority binding, or failure policy is
  weakened to obtain the pass;
- all attributable failures have focused regression evidence;
- the terminal diff stays inside the amended prospective write set and every
  applicable instruction/line-count requirement is dispositioned;
- dual review and dual verification accept the direct gate evidence and final
  package status.

## Review, verification, and security

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner` subagents for baseline/current
full-workspace and Clippy execution, triage/worker subagents for bounded failure
families, two independent review subagents, and two independent verification
subagents. Comparator outputs are compact metrics, exact identities, failure
inventories, and retained log paths with read-only source access. Triage outputs
are bounded diagnoses or prospective inventory entries; worker write access is
limited to explicitly assigned files. Review and verification write access is
limited to the assigned package artifact. Standing user/session authorization
was supplied on 2026-08-30.

Security impact is `NONE` unless exact triage discovers a security/protected-data
surface; such a discovery requires explicit package amendment before edits.

## Gate non-deferral

The two mandatory failing commands are the package objective and cannot be
deferred, waived, or converted to a narrative handoff. Continue through safe
in-envelope repair while any classified correction remains available.
