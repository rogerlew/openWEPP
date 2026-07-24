# TESTGATE Net-Benefit Recovery Plan

Date: 2026-07-24

Evidence class: `Static:` current implementation, canonical testing strategy,
repository history, retained qualification receipts, and the trajectory
assessment.

## Decision

TESTGATE can become useful only by replacing the current architecture with a
minimal exact-head forest1 workflow. Making the existing planner, recovery,
receipt, reconstruction, and attestation stack finally pass would demonstrate
internal consistency, not net benefit.

The recovery objective is therefore:

> Execute already-authoritative repository checks in cost order on forest1,
> retain ordinary evidence, and add no second policy or verification system.

This is a simplification and deletion program, not another hardening phase.

## Definition of success

The replacement is successful only if all of the following are directly
observed:

1. Two consecutive exact-head forest1 runs pass without agent or operator
   intervention.
2. No failure in those runs is caused by workflow, runner integration,
   evidence formatting, orchestration, or TESTGATE-specific tests.
3. Every deterministic static, configuration, affected-coverage, CRAP, mount,
   and capacity check finishes before full-workspace Nextest starts.
4. A deliberately seeded cheap failure is rejected within five minutes and
   starts no heavy command.
5. Workflow overhead outside the repository commands and artifact upload is
   under five minutes and under 10% of total successful runtime.
6. Failure diagnosis requires only the failed command, its log/JUnit, exact
   head SHA, and runner identity; median operator diagnosis time in the two
   qualification runs is under ten minutes.
7. The mandatory implementation stays within:
   - one workflow;
   - one shell script;
   - 250 workflow lines;
   - 400 script lines;
   - no new Rust crate;
   - no new schema;
   - no ledger, recovery protocol, checkpoint importer, planner, or verifier.
8. Total recovery work is capped at one engineering day, two heavy
   qualification runs, and three forest1 runner-hours.

If any limit is exceeded, TESTGATE is abandoned and the canonical commands are
run directly for package/campaign closure.

## What to retain

These pieces provide value independently of the failed architecture:

- forest1 as the heavy execution host;
- the bounded executable `/t` and `/tmp` surfaces and runner resource checks;
- `.config/nextest.toml` profiles and test-group scheduling;
- `tools/release/run_adjudicated_crap_gate.sh`;
- repository authority and anti-evasion scripts selected by the package;
- explicit dispatch and the stable concurrency group;
- exact current-main SHA rejection;
- GitHub Actions logs, JUnit, coverage/CRAP JSON, and ordinary artifact upload;
- the rule that `LOCAL_UNTRUSTED` is expected local evidence and never alone a
  failure; and
- the rule that retired Omarchy queue records are ignored.

These are the actual test and runner capabilities. They do not require the
gate planner to remain in the execution path.

## What to remove from the mandatory path

The following layers have not demonstrated incremental defect-detection value
and must not mediate ordinary increments:

- intent and terminal plan reconstruction;
- package-authority chains as runtime workflow inputs;
- generated execution DAGs and node identities;
- staged LIGHT/HEAVY receipts;
- pre-heavy audit documents and audit identities;
- checkpoint mirroring and checkpoint import;
- recovery archive selection, invalidation, and resume decisions;
- durable tooling-defect ledgers;
- hosted repository reconstruction;
- hosted receipt-envelope verification;
- recovery and receipt attestations;
- trust promotion from local execution to hosted authority;
- combined-quality proof machinery; and
- TESTGATE-specific qualification matrices.

The existing implementations may remain temporarily as nonexecuted historical
code while deletion is reviewed. They must not remain authoritative or block
science work.

## Minimal workflow

The replacement has one forest1 job and no hosted verification job.

### Inputs

- `base_ref`: exact package scaffold/base commit.
- `head_ref`: implicit `github.sha`, required to equal current `origin/main`.
- `affected_packages`: explicit comma-separated Cargo package names from the
  work package. Empty means global quality; it never infers a narrower scope.

No package path, authority-chain ID, receipt ID, recovery root, audit ID, or
combined-proof ID is accepted.

### Phase 1: cheap preflight

Run sequentially and stop at the first failure:

1. Checkout exact head and confirm it equals current `origin/main`.
2. Confirm forest1 identity, tool versions, available space, memory limit,
   executable `/t`, and executable `/tmp`.
3. Confirm a clean checkout and exact base/head diff.
4. Run `cargo fmt --all --check`.
5. Run Markdown and schema lint selected by changed paths.
6. Run package/write-set and authority anti-evasion scripts only when their
   governed paths changed.
7. Run fresh affected coverage/CRAP for every explicitly supplied affected
   production package:

   ```bash
   bash tools/release/run_adjudicated_crap_gate.sh \
     --scope affected \
     --package <package> \
     --nextest-profile affected \
     --base-ref <base> \
     --output-dir <evidence>/affected-crap
   ```

8. Reject zero executed tests, missing JUnit, dirty source, actionable CRAP,
   or any required output absence.

Nothing after this point may rediscover a condition that these checks can
determine.

### Phase 2: heavy validation

Only after Phase 1 passes:

1. Run `cargo nextest run --workspace --profile full` once.
2. Run fresh global adjudicated CRAP only when the package boundary requires
   campaign/global closure. Ordinary bounded increments stop after affected
   CRAP plus full Nextest unless the canonical testing strategy independently
   requires global quality.
3. Do not reconstruct another checkout or compile another inventory.
4. Do not resume a failed run. A new attempt begins from a clean runner
   workspace.

### Phase 3: evidence

Upload one artifact directory containing:

- `summary.json` with repository, base SHA, head SHA, runner name, image
  digest, start/end timestamps, commands, exit codes, and final result;
- full Nextest JUnit;
- affected and/or global CRAP JSON and checksum manifests;
- outputs from selected authority/anti-evasion checks; and
- plain command logs.

`summary.json` is generated directly with `jq`. It has no custom schema,
derived identity, trust promotion, or independent verifier. GitHub already
binds the artifact to the workflow run and commit.

## Required ordering correction

The latest failure proves the current order is wrong:

```text
current:
  full workspace (heavy) -> global CRAP -> discover new function at 0% coverage

replacement:
  affected coverage/CRAP -> full workspace -> optional required global CRAP
```

The public `validate_relocated_audit` coverage failure is not a reason to
dispatch again. If the legacy architecture is retained long enough to run one
final comparison, add the direct public-path test and pass affected CRAP
locally first. That work does not qualify the architecture or justify keeping
it.

## Authority and documentation changes

Before implementation, a reset package must prospectively amend:

- `AGENTS.md`;
- `docs/ROADMAP.md`;
- `docs/standards/testing-and-gate-strategy.md`;
- ADR-0040 or a superseding ADR;
- `tools/local_ci/README.md`; and
- work-package templates that currently require TESTGATE plans, receipts,
  audits, recovery, or hosted verification.

The amendments must:

- mark the current TESTGATE architecture non-authoritative;
- restore canonical repository commands as authority;
- declare the minimal workflow an observational dispatcher, not a policy
  engine;
- prohibit current TESTGATE from blocking science work during replacement;
- bind forest1 and `LOCAL_UNTRUSTED` exactly as user-supplied invariants; and
- declare the deletion boundary and deadline.

Without this authority reset, a small workflow would remain trapped behind the
old architecture's requirements and could not deliver simplification.

## Execution packages

### Package A: authority reset

Documentation-only. Freeze current TESTGATE, remove its blocking status, define
the minimal command order, and authorize deletion. No heavy run.

Exit criterion: repository guidance contains no obligation to use the old
planner/recovery/verifier path for ordinary increments.

### Package B: minimal workflow

Create the one workflow and one shell script using only retained components.
Do not modify the gate-planner crate except to disconnect it from workflow
execution.

Exit criteria:

- workflow/script size budgets pass;
- seeded preflight failure exits under five minutes without heavy execution;
- shell/static checks pass; and
- exact command/evidence outputs are locally reproducible.

### Package C: bounded qualification

Run at most two successful exact-head forest1 qualifications. The first may
expose one implementation defect. A second control-plane failure terminates
the experiment and selects direct manual commands permanently.

Record command durations separately from workflow overhead.

### Package D: deletion

After two successful runs, delete or archive the unused planner, executor,
verifier, recovery, checkpoint, receipt, and schema surfaces. Remove their
tests from mandatory full-workspace execution rather than paying permanent
maintenance cost for retired architecture.

Exit criterion: ordinary full-workspace validation does not compile or test a
retired control plane.

## Benefit measurement

Use the retained trajectory as the failure baseline:

- 22 observed runs;
- 7.62 workflow-hours;
- 122 scoped commits;
- zero end-to-end successes; and
- repeated operator intervention.

For each replacement qualification record:

| Metric | Required result |
|---|---|
| End-to-end result | PASS twice consecutively |
| Control-plane failures | 0 |
| Manual interventions | 0 |
| Cheap seeded-failure latency | <5 minutes |
| Heavy work after seeded cheap failure | none |
| Non-command workflow overhead | <5 minutes and <10% |
| Diagnosis latency | <10 minutes |
| Workflow size | <=250 lines |
| Script size | <=400 lines |
| New schemas/crates/protocols | 0 |

Net benefit is demonstrated only if those measurements pass. A green run by
itself is insufficient.

## Stop-loss rules

Stop immediately and do not harden further if:

- the replacement has a second control-plane failure;
- a cheap deterministic condition is discovered after heavy execution;
- implementation exceeds the size or time budget;
- any proposal adds a planner, ledger, schema, verifier, recovery mechanism,
  or trust taxonomy;
- operator action is needed between workflow phases;
- the workflow cannot explain a failure from ordinary logs and artifacts; or
- science work is blocked beyond the one-day implementation budget.

The fallback is not another architecture. It is direct execution of the
canonical commands on forest1.

## Recommendation

Proceed only with Package A, the authority reset. Do not first fix the current
global CRAP row and dispatch the old workflow again. Once the old system is
non-authoritative, implement the minimal workflow under the fixed budgets.

This route can produce net benefit because it makes TESTGATE a thin,
replaceable convenience over proven commands. Any route that requires the
current architecture to qualify before simplification repeats the sunk-cost
failure.
