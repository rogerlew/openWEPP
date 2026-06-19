# PERFDEEP07 Disposition

Status: HOLD.
Evidence mode: Static/Ran.

## Verdict

`HOLD`.

PERFDEEP07 executed the ordered disabled-path audit and repair attempt, but the
P0 default-disabled H2637 timing gate did not pass. Direct-frame hydrology work
was not started because the package explicitly forbids proceeding past a failed
disabled-path gate.

## Findings

| Finding | Disposition | Evidence |
|---|---|---|
| Default-disabled path still regressed after PERFDEEP05. | accepted / follow-up | Retained patch improved `701.95 s` to `685.85 s`, but P0 threshold is `<= 676.67 s`. |
| Dense-first lookup tax exists in request access when dense surfaces are absent. | accepted / fixed partially | Dense-absent bypass added to request and hydrology accessors. |
| `BTreeMap<String, _>` hot-table lookup contributes avoidable cost. | accepted / fixed partially | Hot lookup maps changed to `HashMap`. |
| Removing production indexed runtime entirely should make disabled path faster. | rejected | Measured worse: `753.38 s` and `755.48 s`. |
| Rebuilding or propagating indexed surfaces after every day might repair cost. | rejected | Measured `1035.90 s` and `1054.71 s`. |
| Direct-frame implementation can proceed in PERFDEEP07. | deferred / blocked | Blocked by failed P0 default-disabled gate. |

## Closure State

Not ready for PERFDEEP08. The next step is another disabled-path cleanup package
or a PERFDEEP07 continuation focused only on closing the remaining
approximately `9.18 s` gap between the retained `685.85 s` run and the
`676.67 s` threshold.
