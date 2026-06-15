# CRAP Before

Status: complete
Evidence mode: Ran

Command:

```sh
cargo crap --workspace --lcov docs/work-packages/20260615-cqr02-hbp-layout-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr02-hbp-layout-parser-complexity-001/artifacts/crap_before.json
```

Target rows:

| Function | Line | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `parse_layout` | 6 | 165.0 | 81.03683492496589 | 350.65231338239096 |

Note: the first raw before export was captured after initial characterization but before production source edits. Additional focused pre-refactor characterization then raised `layout_parser.rs` line coverage to 87.82816229116943% before decomposition; final closure is represented by the after reports.
