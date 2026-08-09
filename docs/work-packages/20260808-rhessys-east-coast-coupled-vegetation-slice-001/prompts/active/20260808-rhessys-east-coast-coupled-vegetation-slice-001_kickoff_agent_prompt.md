# Kickoff: RHESSys East Coast Coupled Vegetation Slice

Scope: local repository work in `/home/workdir/openWEPP` plus read-only
inspection of `/workdir/RHESSysEastCoast` and `/workdir/GIS2RHESSys` at the
commits pinned in `package.md`. Do not modify the external checkouts, deploy,
publish, send external messages, or touch files outside the declared write set.

Execution mode: package-end-to-end.

Precondition: do not execute this prompt until
`20260808-rhessys-east-coast-code-literature-authority-audit-001` has a passing
terminal disposition, has amended this package, and leaves no required
`BLOCK_SUCCESSOR` row. If the precondition is unmet, stop without edits and
report the predecessor dependency.

The precursor's 2026-08-08 audit amendment records required
`AUTH-RHEC-001..011` and `AUTH-RHEC-014..016` blockers, so this prompt is
intentionally non-executable for production work until a reviewed
authority-admission amendment closes them. Source licensing and audit
completion do not satisfy that condition.

After release, execute every phase in `package.md` sequentially through terminal
disposition or a proven declared hold. Begin by verifying the predecessor's
accepted inventory, concordance, deviations, parameter matrix, authority gaps,
contract amendments, reviews, and successor-amendment report. Do not begin
production Rust until the package's independent contract-first gate passes.

User constraints are binding: support deciduous and mixed forests; accept
existing valid RHESSys vegetation definition files; do not narrow scope merely
to avoid coupled photosynthesis/conductance work. Mixed forest is explicit
multistratum composition, never an undocumented averaged parameter profile.

Source-aware posture: direct adaptation from the two pinned MIT repositories is
authorized only within the predecessor's accepted boundary. Preserve exact
source coordinates and the MIT notice. Do not reopen a concordance or migration
disposition without new evidence and prospective package reconciliation.

Coupling language must match the accepted precursor record for Jarvis-style
conductance, Penman-Monteith water flux, Farquhar photosynthesis, phenology,
canopy state, and root demand. Do not imply bidirectional assimilation-solved
stomata unless the precursor evidence and admitted contract prove that behavior.

## Required Reading

### Core

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/successor-amendment-report.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/source-function-state-inventory.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/parameter-authority-matrix.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/code-literature-concordance-matrix.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/authority-gap-register.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-vegetation-radiation-interception-conductance-slice-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`
- `/home/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md`
- `/home/workdir/openWEPP/docs/backlog/20260806-rhessys-derived-vegetation-crate.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`
- `/home/workdir/openWEPP/tests/AGENTS.md`

### Conditional

- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-spec.md`
- `/home/workdir/openWEPP/docs/specifications/unit-governance.md`
- `/home/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/home/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/home/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/home/workdir/openWEPP/docs/specifications/external-authority/promotion-protocol.md`
- `/home/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/home/workdir/openWEPP/docs/decisions/0042-science-implementation-and-calibration-readiness.md`
- `/workdir/RHESSysEastCoast/LICENSE`
- `/workdir/GIS2RHESSys/LICENSE`

### On Demand

- RHESSysEastCoast and GIS2RHESSys source files named by accepted precursor rows
  only when implementation or differential-test work requires their exact
  expression.
- `SC-LANDSURFACEENERGY-001`, `SC-EVAP-001`, `SC-WATBAL-001`, `SC-PLANT-001`,
  management, and residue contracts only when the selected boundary touches
  their ownership or shared lineage.
- Primary literature named by an accepted precursor row when implementing or
  reviewing that exact behavior.
- External-authority templates, registries, obligations, and adjacent Rust/test
  sources when their authoring phase begins.

Required-reading budget: `677241` local Core bytes, `WARN` (`>400000` and
`<=800000`). External source files and on-demand literature are excluded until
their trigger applies. Record the recomputed terminal budget in package evidence
if Core files change.

## Execution

1. Verify the precursor's terminal identity, accepted reviews, complete audit
   matrices, successor amendments, source identities, and absence of required
   `BLOCK_SUCCESSOR` rows. Record the openWEPP base/diff, instruction chain,
   write set, and required-reading budget.
2. Close the precursor's required `AUTH-RHEC-*` rows through reviewed canonical
   authority; then reconcile the admitted boundary with the concrete Rust API and fixtures;
   prospectively amend the package before implementation if a new dependency
   appears.
3. Amend canonical contracts as still needed, add contract-derived tests and
   locked fixtures, define the required A3 suite, and complete independent
   review and the pre-implementation gate.
4. Implement the strict vegetation-file loader and coupled default-off Rust
   slice with typed errors and source/provenance identities.
5. Prove conservation, operand lineage, source-differential behavior, invalid
   input handling, default-off boundaries, and calibration readiness.
6. Reconcile the exact diff and run every applicable directly selected focused,
   contract, A0/A1/A3, anti-evasion, dependency, security/license, quick, and
   Critical full-workspace gate.
7. Complete required independent science/source and Rust reviews, disposition
   all findings, rerun invalidated gates, and complete terminal verification.
8. Archive this prompt byte-for-byte, update lifecycle records, disposition the
   package truthfully, and commit/push the stable increment when authorized.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/source reviewers, one
`rust_code_reviewer`, one `rust_qa_reviewer`, one
`comparator_suite_runner` for Critical heavy gates, and two independent
read-only terminal verifiers. Expected outputs are the named package review,
review-disposition, gate, and verification artifacts; write access is read-only
for reviewers/verifiers and bounded to ignored target/log/scratch paths for the
comparator runner. Preserve independent initial reviews. If session policy
prevents delegation, record the policy block and do not claim the required gate
or close the package.

No surrogate physics: source code may provide implementation provenance, but
an unresolved equation, domain, parameter, unit, guard, tolerance, ownership,
or compatibility transformation cannot be replaced with an invented heuristic
or silent default. Stop `executed-hold` at the declared boundary instead.
