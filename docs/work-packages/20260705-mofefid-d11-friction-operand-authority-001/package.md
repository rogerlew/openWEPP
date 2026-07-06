# MOFEFID-D11 - Friction Operand Authority

Status: **EXECUTED-HOLD-SOURCE-AUTHORITY** (scaffolded 2026-07-05;
executed 2026-07-06). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: `SC-OFEROUTE-001#GAP-OFEROUTE-007` and active Lane D
friction operands.

## Objective

Close `GAP-OFEROUTE-007`: the Lane D runtime shadow currently drives every
OFE as a bare cell with hardcoded `k_o = 500`, rainfall intensity `I = 0`,
and no WEPP-input lineage for form, wave, or vegetation operands
(`C_d`, `D_r`, `lambda`, `LAI`, `h_c`). D11 must source,
default-authorize, or fail-close every active friction operand and wire the
active/shadow operand builder so the routed-water candidate no longer depends
on unlabeled hardcoded friction.

D11 is an input-authority and runtime-builder package. It is not a
shock-numerics package and must not treat Iwagaki Case 4 as accepted.

## Rationale

The Lane D shadow proved that live frame surfaces can drive the real cascade
without changing protected outputs, but it deliberately used a first-cut
friction policy. That policy is volume-safe but not timing-faithful:
friction shapes hydrograph celerity and storage. D14 runtime profiling and
D15 opt-in activation need a real operand surface, or a fail-closed policy
that prevents unsupported roughness/vegetation cases from being silently
modeled as bare soil.

D10 also confirmed a boundary: Iwagaki primary Case 4 names Manning
`n = 0.009`, while the D-val harness uses `k_o`. D11 may record source
findings that inform the D10 follow-on, but it must not close `GAP-OFEROUTE-005`
or declare Case 4 shock acceptance.

## Scope

### Included

- Amend `SC-OFEROUTE-001` as needed to record active friction operand
  authority, required inputs, default/fail-closed policy, symbol aliases, unit
  governance, and `GAP-OFEROUTE-007` disposition.
- Audit source candidates for each operand:
  - `I` from rain/hourly forcing lineage.
  - `k_o` from source-authorized surface type/default policy or fail-closed
    unsupported surface classes.
  - `C_d`, `D_r`, `lambda` from management/residue/roughness-element authority
    or fail-closed unsupported form/wave classes.
  - `LAI`, `h_c` from existing growth/canopy surfaces or fail-closed
    unsupported vegetation-resistance cases.
- Build or update the active/shadow Lane D friction operand builder and its
  typed guards, if contract authority supports it.
- Prove the current hardcoded `LANED_SHADOW_KO` / `I = 0` path is retired or
  explicitly limited to a named default/fail-closed policy.
- Add contract-derived tests and targeted H2637/fixture evidence for the
  active/shadow operand builder.
- Record the Case-4 boundary: D11 does not close the D10 source-authority hold
  or D-val shock acceptance.

### Excluded

- No production/default routed-water activation.
- No D10 shock-numerics correction, Case-4 acceptance, or
  `GAP-OFEROUTE-005` closure.
- No D12 melt-limb hourly-source implementation.
- No D13 ADR-0036 erosion hourly-shape switch.
- No D14 runtime profiling/optimization except any small timing evidence
  needed to prove the operand builder is not grossly pathological.
- No D15 opt-in production flip or D16 default-promotion policy.
- No tuned friction values chosen to match comparator traces.

## Dependencies

- `SC-OFEROUTE-001` rev 18.
- MOFEFID strategy §6.1 D11 row.
- D10 package and worker handoff:
  `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/`.
- Lane D runtime shadow package:
  `docs/work-packages/20260705-mofefid-laned-activation-increment-001/`.
- Current shadow implementation:
  `crates/openwepp-runner/src/hillslope/laned_shadow.rs`.
- Current friction kernels:
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
  or the current `ofe_routing` module where friction kernels live.
- Candidate source authorities, loaded on demand:
  management/plant/residue contracts and specs; current runtime growth,
  residue, canopy, rainfall, and management projection surfaces; and the
  Lane D friction references already listed in `SC-OFEROUTE-001`.

## Intended Write Set

Primary:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` only if status or
  sequencing text changes after execution.

Conditional, only if authority supports implementation:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`
- Focused tests under `crates/openwepp-runner`, `crates/openwepp-hillslope-orchestrator`,
  or `tests/integration/laned_shadow_h2637.rs`.

Protected:

- Production activation selector and default runtime policy.
- HBP/pass/watershed output schemas.
- D-val Case-4 numerical-method acceptance.
- Raw copyrighted source/workbook additions beyond existing governance.

## Phase Plan

1. **D11-S0 - Intake and baseline.** Read required authority, record the
   current `GAP-OFEROUTE-007` state, locate all current hardcoded friction
   paths, and confirm D10's Case-4 boundary.
2. **D11-S1 - Operand-source audit.** For `I`, `k_o`, `C_d`, `D_r`,
   `lambda`, `LAI`, and `h_c`, classify candidate sources as
   source-authorized, default-authorizable, unsupported/fail-closed, or
   out-of-scope. Record units, timing basis, lane/OFE basis, and failure mode.
3. **D11-S2 - Contract-first authority.** Amend or confirm
   `SC-OFEROUTE-001` with operand authority and typed guard/default policy
   before production/shadow code edits.
4. **D11-S3 - Contract-derived tests and pre-implementation gate.** Add tests
   that fail on the hardcoded first-cut path where replacement is authorized,
   and tests for fail-closed unsupported cases.
5. **D11-S4 - Builder implementation or authority HOLD.** Wire the active
   friction operand builder into the shadow/active candidate if the authority
   supports it. If authority is missing for one or more required operands,
   close in `HOLD` only with a named boundary and no hidden defaults.
6. **D11-S5 - Evidence and closure.** Run gates, update artifacts, complete
   dual review/disposition/verification, and set final status.

## Exit Criteria

- `GAP-OFEROUTE-007` is closed or held with explicit per-operand authority.
- Every active friction operand has one of:
  - source-authorized lineage and typed guard,
  - ratified bounded default with scope and tests,
  - fail-closed unsupported policy with typed error and tests.
- The active/shadow routed-water candidate no longer uses unlabeled
  hardcoded `k_o = 500` and `I = 0` for all lanes, unless that behavior is
  retained only under a ratified narrow default/fail-closed policy.
- The package proves the real shadow/active candidate consumer reads the new
  operand builder, or closes in `HOLD` before making a consumer-facing claim.
- D10 Case-4 shock acceptance remains open unless the separate D10 follow-on
  closes it; D11 must not accept/tune Case 4.
- No production/default activation or D12-D16 work occurs.
- Dual review findings are dispositioned as `accepted`, `rejected`,
  `deferred`, or `follow-up`; accepted findings are fixed before closure.
- Line-count governance is recorded for every touched `.rs` file.

## Required Gates

Selection follows `docs/standards/local-ci-gate-selection.md` where relevant,
but D11 cannot close without recording:

- `git diff --check`
- Markdown lint for touched docs.
- Contract/profile/BEI checks for changed `SC-OFEROUTE-001` surfaces.
- Unit-governance checks if units/symbols/defaults change.
- Focused tests for the operand-source audit and builder/fail-closed policy.
- H2637 or targeted Lane D shadow fixture evidence showing the candidate
  consumes the new builder, or explicit hold evidence if no builder is
  authorized.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, fixtures, or
  authority-suite posture are touched.

If heavy gates are delegated, record subagent output and log paths in
`artifacts/gate-results.md`.

## Conservation / Output Acceptance

D11 does not create new publication outputs, but friction changes timing and
storage of routed water. Before any builder/solver consumer edit, record an
operand-lineage table for each friction operand, its units, timing/OFE basis,
source authority, diagnostic vs authoritative status, and rejected aliases.
Acceptance must include consumer-path proof and at least one routed-path
sanity/closure check. Exact self-consistency alone is not sufficient.

## HOLD Legitimacy

D11 may close in `HOLD` only for a specific per-operand authority boundary:
missing/contradictory input authority, unsupported policy slice that must fail
closed, or a mechanism proven to belong to D10/D12/D13/D14/D15. A hold must
name the operand(s), cite evidence, list the in-envelope sourcing/default route
considered, and explain why that route cannot close in D11.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` subagents for read-only source/authority audit,
review, verification, fixture inspection, and heavy gate execution. Expected
outputs are compact findings, gate metrics, log paths, and package-local
review or verification artifact text. Write access is read-only unless a later
operator explicitly assigns a bounded write set.

Subagent requirement: `comparator_suite_runner` is REQUIRED for full workspace
nextest, H2637/fixture batches, and other heavy closure gates when available.
Do not run those heavy batches on the parent model unless the subagent is
unavailable; if unavailable, record command-level evidence and run locally
only when package governance permits substitution.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/operand-authority-map.md`
- `artifacts/source-audit-evidence.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/friction-builder-evidence.md`
- `artifacts/consumer-path-evidence.md`
- `artifacts/case4-boundary-evidence.md`
- `artifacts/hold-legitimacy-audit.md`
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

## Progress

- [x] 2026-07-05: Package scaffolded from MOFEFID §6.1 D11 row and
      `SC-OFEROUTE-001#GAP-OFEROUTE-007`.
- [x] 2026-07-06: D11-S0 intake and baseline completed; D10 Case-4
      boundary confirmed and current shadow hardcoded `k_o=500` / `I=0`
      path located.
- [x] 2026-07-06: D11-S1 operand-source audit completed with explorer
      subagent support. `I` and `LAI` have source candidates; `h_c` has
      incomplete candidate lineage; `k_o`, `C_d`, `D_r`, and `lambda` have
      no D11-ratified WEPP-runtime mapping/default.
- [x] 2026-07-06: D11-S2 contract-first authority completed in
      `SC-OFEROUTE-001` rev 19.
- [x] 2026-07-06: D11-S3 pre-implementation contract gate recorded as
      blocked by missing source/default authority; no contract-derived
      production tests were authored.
- [x] 2026-07-06: D11-S4 closed as authority HOLD rather than wiring a
      surrogate friction builder.
- [x] 2026-07-06: D11-S5 package artifacts, review, verification, and
      disposition completed.

## Surprises & Discoveries

- The direct runtime already carries a usable rainfall-intensity lineage for
  `I`: climate `intsty_m_s`, plus `wb14_hourly_rainfall_m[h] / 3600 s` as an
  hourly-bin candidate if carried through a builder. The shadow still forces
  the skin term to `I=0`.
- Plant state exposes `LAI`, and `canhgt`/`Hc` candidates exist, but the
  shadow observes only `DirectPublicationDayRow`, which currently does not
  carry a friction operand payload.
- Chapter-10 hydraulics roughness/cover terms are adjacent authority, not a
  direct alias to Papanicolaou `k_o`, `C_d`, `D_r`, or `lambda`.

## Decision Log

- Decision: D11 owns active friction operand authority and the active/shadow
  operand builder, but not Case-4 shock acceptance.
  Rationale: D10 held Case 4 on numerical/source-authority reconciliation and
  explicitly rejected `k_o` scans as tuning. D11 can remove hardcoded shadow
  friction without treating the Iwagaki comparand as accepted.
  Date/Author: 2026-07-05 / Codex.
- Decision: Close D11 as `EXECUTED-HOLD-SOURCE-AUTHORITY`, not by adding a
  bare-soil default or roughness surrogate.
  Rationale: D11 found source candidates for `I` and plant operands, but no
  ratified WEPP-runtime mapping/default for `k_o`, `C_d`, `D_r`, or
  `lambda`. Any builder using fabricated values would violate the package's
  no-surrogate-physics boundary.
  Date/Author: 2026-07-06 / Codex.

## Outcomes & Retrospective

D11 did not close `GAP-OFEROUTE-007`; it made the blocker exact. The current
shadow may remain a labeled diagnostic first cut, but it cannot carry
friction-fidelity, Case-4, activation, or default-promotion claims. The first
actionable follow-on is to ratify every missing friction operand source or
default, then wire and test a real consumer-read builder.
