# LANED Router D16 Hybrid Route-Coefficient Authoring Bridge

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-BRIDGE-AUTHORITY

## Objective

Execute the named follow-on from
`20260707-laned-router-d16-hybrid-cohort-authority-hold-lift-001`: produce
source-authorized native Lane-D route-coefficient inputs for the selected D16
hybrid promotion cohort, either by importing explicit native `ow-lanuse-1`
inputs or by landing a contract-first legacy-to-native bridge.

Execution result: held before implementation. The selected external cohort
roots still have no native `ow-lanuse-1` managements, no
`routing_coefficients`, and no openWEPP `*.run.toml` active inputs. Existing
LANUSE/SC authority and the D11 source audit do not contain a safe
legacy-field mapping for all five static route coefficients.

## Rationale

The previous D16 hold-lift proved the active plain-vs-hybrid cohort cannot be
constructed from current owcmp inventory roots because active Lane-D routing
fails closed unless every scheduled lane has source-authorized static route
coefficients. It named `D16-HYB-ROUTE-COEFF-AUTHORING-BRIDGE` as the first
actionable follow-on.

This package executes that follow-on until one authority path closes or the
bridge is proven illegitimate inside the current envelope. It is not allowed
to invent coefficient values, reuse H2637 timing scaffolding as cohort
authority, or infer Papanicolaou roughness-element operands from adjacent
legacy fields without a ratified bridge.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/package.md`
- `docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/artifacts/source-audit-evidence.md`
- `docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/artifacts/hold-legitimacy-audit.md`
- `docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/artifacts/friction-builder-evidence.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-cohort-authority-hold-lift-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-cohort-authority-hold-lift-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-cohort-authority-hold-lift-001/artifacts/worker-handoff.md`

## Scope

Included:

- Re-audit the selected external D16 cohort roots for source-authored native
  `ow-lanuse-1` inputs, `routing_coefficients`, and active `*.run.toml`
  inputs.
- Audit current contract/provenance authority for a legacy-to-native bridge
  that could map named legacy input fields to `k_o`, form `C_d`, `D_r`,
  `lambda`, and vegetation `C_d`.
- Implement only if one authority path is present and complete.
- Record hold evidence and a first actionable source-acquisition follow-on if
  neither path is present.

Excluded:

- Default selector promotion.
- Fidelity tolerance ratification.
- H2637-only timing or fidelity claims beyond negative evidence.
- Surrogate coefficient generation from row width, ridge spacing, random
  roughness, residue, canopy cover, Chapter-10 hydraulics, or D-val fixtures.
- Changes to owcmp suite posture or required-case bindings unless
  source-authorized active inputs exist.

## Acceptance Criteria

This package may lift the route-coefficient hold only if one of these paths is
fully satisfied:

- Source-authored input path: selected cohort members carry native
  `ow-lanuse-1` managements with complete, schedule-consistent
  `routing_coefficients` for every active Lane-D lane, and active preflight
  succeeds.
- Bridge path: a canonical contract amendment names the legacy source fields,
  units, validation domain, and mapping for all five static route coefficients,
  with tests, BEI/profile evidence, and anti-evasion coverage.

If neither path exists in the current repo/session, close as hold and do not
partially flip production/default promotion or comparator suite posture.

## Intended Write Set

- This package directory.
- `docs/work-packages/README.md`.
- `docs/specifications/science-contracts/contracts/SC-*.md`, tests, and bridge
  implementation files only if authority exists and a contract-first bridge is
  safe to land.
- No Rust kernel/runtime code for package-local defaults or compatibility
  wrappers.

## Phase Plan

1. **S0 Scaffold and authority ledger.** Create package-local plan,
   artifacts, prompts, and catalog entry.
2. **S1 Source-authored input audit.** Search selected roots for native
   management datvers, route-coefficient markers, and active run inputs.
3. **S2 Bridge authority audit.** Evaluate existing LANUSE, SC-INFILE,
   SC-OFEROUTE, and D11 evidence for a safe legacy-to-native mapping.
4. **S3 Implementation decision.** Land bridge/input implementation only if an
   authority path is complete; otherwise record the hold.
5. **S4 Review, verification, and gates.** Complete dual review, finding
   disposition, dual verification, line-count governance, final disposition,
   and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `rust_code_reviewer`, `rust_qa_reviewer`,
`verification_runner`, and `explorer` subagents for source-authority review,
bridge-legitimacy review, package gate verification, and bounded codebase
questions. Expected outputs are package-local `artifacts/review-*.md` and
`artifacts/verification-*.md`. Write access is read-only unless a worker is
explicitly assigned a bounded implementation fix inside the intended write set.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/source-authored-input-audit.md`
- `artifacts/bridge-authority-audit.md`
- `artifacts/command-evidence.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Current source-authored input scan over selected D16 cohort roots.
- Bridge-authority audit against LANUSE, SC-INFILE, SC-OFEROUTE, and D11
  evidence.
- Active missing-coefficients fail-closed guard.
- Contract/profile/BEI checks if any `SC-*` authority is touched.
- Native parse/projection tests if input-authoring or bridge code is touched.
- Active plain/hybrid preflight if source-authorized active inputs are
  produced.
- Anti-evasion guards if any required-case binding, cohort fixture, external
  authority suite posture, or suite manifest is touched.
- `cargo fmt --check`
- `.rs` line-count governance.
- Full Rust closure loop only if Rust code, contracts, fixtures, suite posture,
  or active input fixtures are changed.

## Closure Outcomes

- `EXECUTED-COMPLETE-ROUTE-COEFFICIENT-AUTHORITY`: native inputs or a ratified
  bridge exist, active preflight passes, and the D16 executable cohort can be
  constructed.
- `EXECUTED-HOLD-ROUTE-COEFFICIENT-BRIDGE-AUTHORITY`: no source-authored
  native inputs exist and current contracts/provenance do not authorize a
  bridge.
- `EXECUTED-HOLD-INPUT-ACQUISITION`: bridge authority is rejected but a clear
  operator/source-authored input acquisition path remains.
