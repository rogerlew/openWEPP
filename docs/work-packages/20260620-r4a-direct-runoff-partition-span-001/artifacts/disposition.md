# R4A Disposition

Status: complete.
Evidence mode: Static + Ran.

Final verdict:
`COMPLETE-R4A-DIRECT-RUNOFF-PARTITION-SPAN`.

R4A scaffolded and executed the first direct hydrology-process span. The span
implements a narrow SC-RUNOFFPART-authoritative runoff-partition closure slice,
mutates only direct runtime state, produces direct downstream operands, and
shadow-projects the direct runoff result.

## Finding Disposition

| Finding | Disposition | Evidence |
|---|---|---|
| Review A blocking findings | None | R4A formula and tests map to recorded SC-RUNOFFPART operand lineage. |
| Review B blocking findings | None | All current-scope gates have direct evidence; no scheduler, compatibility API, publication, schema, science-contract, or default-activation edits. |

## Gates

Ran:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- focused R4A tests: PASS.
- runner direct-runtime counter tests: PASS.
- no-compatibility proof: PASS.
- default-disabled H2637 gate: PASS, median `644.01 s <= 676.67 s`.
- protected output identity: PASS.
- scoped markdown lint: PASS.
- `git diff --check`: PASS.

## Boundary Statement

R4A does not migrate full WB12/WB14, Green-Ampt infiltration, interception,
snowmelt/irrigation liquid assembly, WB18/WB19, peak runoff, erosion,
publication, output schemas, scheduler paths, or default activation. It closes
one process equation slice inside the direct runtime.
