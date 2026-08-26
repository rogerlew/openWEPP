# Discrete terminal support-root workspace qualification hold

Status: `RAN / WORKSPACE QUALIFICATION HOLD`.

This artifact records the terminal clean-commit workspace comparator for
`CHILD1-DISCRETE-SUPPORT-ROOT-001`. It does not change the independently
reviewed physical no-admissible-root HOLD.

## Exact clean execution

Ran: at exact clean commit
`97accd99a62d9e4418d2eb7533c4474fe405427d`, with an empty
`git status --porcelain`, the delegated comparator executed:

```text
nix develop --command tools/dev/heavy cargo nextest run --workspace --no-fail-fast
```

Nextest run `a8ea9537-4240-4bab-a8aa-3673b514d6e5` completed in
730.850 seconds (801,634 ms including wrapper time), status 100. It ran 3,360
tests: 3,258 passed, 102 failed, and six skipped. The raw receipt is
`/tmp/exact_workspace_comparator_workspace.log`.

Ran/comparator: all 101 failures from the intake comparator remain present.
Their names and normalized signatures are unchanged. One formerly passing
external-runtime guard added a prohibited 102nd failure:

```text
openwepp::vegetation_boundary_authority_contract
v9_oracle_successor_is_exactly_bound_and_v8_is_immutable
```

Its exact failure is
`dynamic object checksum mismatch: libcrypto.so.3`. The frozen V9 descriptor
requires SHA-256
`0cd331307536a397ab9c83c6dbeeb3474d0a5114f397ce03d1762adb96d3c781`,
while `/usr/lib/x86_64-linux-gnu/libcrypto.so.3` at terminal execution hashes
to
`23265e4027cb6439687be04311a0f37e27f29a23bfa4c750c49725da14f986bb`.

Static: the failing integration test and the complete protected V9 calculator,
definition, vectors, and runtime-descriptor tree are byte-identical between
intake `221e94ef3e6ccf646f732bf104b0fb563208d338` and evidence commit
`97accd99a62d9e4418d2eb7533c4474fe405427d`. No Child-1 source byte caused
the mismatch. The protected guard/runtime evidence and the host dynamic object
are not modified or waived here.

## Qualification disposition

The owner required zero new failure names and normalized signatures. The
observed external-runtime guard failure violates that terminal criterion even
though the Child-1 diff did not cause it. Therefore the workspace comparator
is non-qualifying and this terminal checkpoint records `WORKSPACE
QUALIFICATION HOLD`.

The 90 already recorded workspace debts remain exactly 81 Assurance V2
failures plus nine source/registry guards. This external-runtime incident is a
separate observed qualification blocker and is not silently added to, waived
with, or repaired inside that 90-failure census.

## Line-count governance

Static: `snow_stage3_v11_terminal_execution.rs` increased from 1,895 lines at
intake to 2,137 lines, crossing the 2,000-line WARN threshold but remaining
below the 3,000-line required-refactor threshold. The test-only real endpoint
DTO and evaluator remain beside the private candidate binder for this bounded
HOLD so owner binding is not widened into a production API. If the discrete
model is revisited, split that `cfg(test)` evidence into a child module before
reuse, centralize the endpoint result/closure assembly and WB14-ceiling
mapping identified by ownership review, and rerun the applicable exact-source
gates. This HOLD does not authorize that follow-on.

## Claim boundary

Production remains `BelowCarrierDomain`. No constitutive equation, constant,
600-ms floor, public API/output, owner publication, restart/receiver/runner,
selector, Stage-3 activation, CoE, successor contract, Batch V2, Child 3/4, or
cutover behavior is changed. Physical implementation
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999` remains the last fully qualified
physical implementation.
