# R2A Default-Disabled Regression Gate

Status: queued.
Evidence mode: not run.

PERFDEEP09 closed the default-disabled blocker with final H2637 reps:
`634.61 s`, `635.65 s`, `636.58 s`; median `635.65 s`.

R2A must preserve:

- all PERFDEEP/direct-runtime opt-ins disabled by default;
- H2637 protected identity under PERFDEEP09 policy;
- final default-disabled H2637 median `<= 676.67 s`.

Record binary SHA, command, environment, output identity, min/median/max
seconds, RSS, and comparison to the PERFDEEP09 threshold.
