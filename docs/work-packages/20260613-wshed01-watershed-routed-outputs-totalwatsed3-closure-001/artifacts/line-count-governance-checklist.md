# Line Count Governance Checklist

Status: T-B executed

Evidence mode: Ran

W-B edited the impoundment parser and runner test only. No production file
crossed the 2000-line warning threshold.

Observed line counts after W-B:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2031 | WARN; W-B did not edit this file. W-C should avoid growing it further or plan split. |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | 1431 | Test file below WARN. |
| `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs` | 1390 | Below WARN. |
| `crates/openwepp-watershed-output/src/writers.rs` | 1712 | Below WARN, close to threshold. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs` | 1934 | Below WARN, close to threshold. |

Command:

```bash
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs \
  crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs \
  docs/contracts/openwepp-watershed-runfile-contract.md
```

Observed line counts after W-C:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2066 | WARN; W-C limited growth and moved WAT aggregation to `watershed_wat.rs`. |
| `crates/openwepp-runner/src/watershed_wat.rs` | 574 | New module below WARN. |
| `crates/openwepp-watershed-output/src/writers.rs` | 1904 | Below WARN, close to threshold. |
| `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs` | 2029 | WARN test file; no production refactor required. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | 1404 | Below WARN. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs` | 1102 | Below WARN. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs` | 931 | Below WARN. |

Command:

```bash
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-runner/src/watershed_wat.rs \
  crates/openwepp-watershed-output/src/writers.rs \
  tests/integration/ws11_channel_routing_physics_equivalence_contract.rs
```

Observed line counts after W-D:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2072 | WARN; W-D only added new row-seed defaults. Avoid further growth unless binding truly belongs in the CLI. |
| `crates/openwepp-runner/src/watershed_wat.rs` | 911 | Below WARN. |
| `crates/openwepp-watershed-output/src/writers.rs` | 2043 | WARN; subsequent increments should avoid growth or split before adding more writer logic. |
| `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs` | 1327 | Below WARN. |

Command:

```bash
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-runner/src/watershed_wat.rs \
  crates/openwepp-watershed-output/src/writers.rs \
  crates/openwepp-sim-contract/src/units_mod/output_catalog.rs
```

T-A line-count disposition:

- No production source files were edited.
- Existing WARN files remain T-B watchpoints:
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` and
  `crates/openwepp-watershed-output/src/writers.rs`.
- Subsequent implementation should put dedicated totalwatsed3 logic in a new module/binary path
  instead of growing the watershed CLI or watershed writer further.

Observed line counts after T-B:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/totalwatsed3.rs` | 1241 | New dedicated module below WARN. |
| `crates/openwepp-runner/src/bin/openwepp-cli-totalwatsed3.rs` | 159 | New dedicated binary below WARN. |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2062 | WARN; T-B removed totalwatsed3 aggregation ownership but file remains above 2000. |
| `crates/openwepp-watershed-output/src/writers.rs` | 2002 | WARN; T-B touched writer seed/output mapping. Split before further growth where practical. |
| `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs` | 1330 | Below WARN. |

Command:

```bash
wc -l crates/openwepp-runner/src/totalwatsed3.rs \
  crates/openwepp-runner/src/bin/openwepp-cli-totalwatsed3.rs \
  crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-watershed-output/src/writers.rs \
  crates/openwepp-sim-contract/src/units_mod/output_catalog.rs
```

T-B line-count disposition:

- No touched production file is near the 3000-line hard split threshold.
- Two WARN files remain: `openwepp-cli-watershed.rs` and `writers.rs`.
- T-C should avoid growing either file unless it includes a focused split or a
  narrow, justified mapping change.
