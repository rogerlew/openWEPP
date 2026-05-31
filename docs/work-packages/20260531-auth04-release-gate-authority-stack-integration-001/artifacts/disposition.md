# AUTH04 Disposition

Status: completed  
Evidence mode: Static + Ran

## Scope
- Integrate correctness authority-stack suites into release/CI gate wiring with
  lane and failure-class policy.

## Decision
- **GO**

## Exit-criteria adjudication

1. Release workflow enforces required authority stack gates:
   - pass
   - required lane executes and blocks on hard-fail outcomes.
2. Periodic/manual suites are routable and documented:
   - pass
   - workflow + script support explicit periodic/manual triggers.
3. Failure classes are machine-enforced and operator-visible:
   - pass
   - hard-fail returns non-zero; investigation is recorded non-blocking in
     `authority_suite_results.md`.

## Rationale

- AUTH04 implemented contract-defined lane semantics in:
  - `.github/workflows/release-gates.yml`
  - `tools/release/run_release_candidate_gates.sh`
- Contract-derived coverage was added in:
  - `tests/integration/auth04_release_gate_authority_stack_contract.rs`
- Full release-gate automation run (skip stability) passed with authority lane
  reporting artifacts.

## Follow-on

1. Register Level-5/Level-6 suites with `gate_lane=periodic|manual` and
   `failure_class=investigation|hard-fail` as they are authored.
2. Add at least one investigation-class suite to exercise non-blocking failure
   handling in CI under a controlled negative fixture.
