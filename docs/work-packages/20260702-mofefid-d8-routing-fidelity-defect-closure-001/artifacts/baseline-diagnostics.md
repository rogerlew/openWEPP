# D8 Baseline Diagnostics

Evidence classes:

- `Static:` source/code/contract inspection.
- `Ran:` command execution in this package.

## Source Hashes

Ran:

```console
sha256sum references/copyrighted/Papanicolaou2018-supplemental/wrcr23071-sup-0002-2017wr021109-ds01/Figure_4.xlsx \
  references/copyrighted/Papanicolaou2018-supplemental/wrcr23071-sup-0002-2017wr021109-ds01/3.1_Validation_Input.docx
```

Result:

| Source | sha256 |
|---|---|
| `Figure_4.xlsx` | `2bf68787de6a715049ee635c154c640214936fd1181d08c8f7da7a34892d2fe8` |
| `3.1_Validation_Input.docx` | `0aee14555a3f5394aef89c9b6623fc13644273a676bb316e76ca5b6e148f9362` |

These match D01 `source-manifest.md`.

## Entry D7 Metrics

Ran before D8 edits:

```console
cargo nextest run -p openwepp-hillslope-orchestrator dval
.venv/bin/python tools/dval/compare_dval.py --case <1..4> --fig4 .../Figure_4.xlsx --crate-dir .
```

Entry result:

| Case | D7 `NS_trace` | peak ratio | openWEPP `t_peak` | rise 10-90% | D7 verdict |
|---|---:|---:|---:|---:|---|
| 1 | 0.868455 | 1.066 | 18000 s | 4999.6 s vs 3579.9 s | partial |
| 2 | 0.453954 | 0.747 | 10800 s | 3930.6 s vs 3021.0 s | operand-limited |
| 3 | 0.537727 | 0.547 | 3600 s | 1449.3 s vs 1631.5 s | caveated |
| 4 (`k_o=200`) | 0.300882 | 0.789 | 28 s | 20.6 s vs 20.9 s | operand-limited timing/rise reproduce |

## D8 Corrected Metrics

Ran after D8 code corrections:

```console
.venv/bin/python tools/dval/compare_dval.py --case <1..4> --fig4 .../Figure_4.xlsx --crate-dir .
.venv/bin/python tools/dval/compare_dval.py --case 4 --ko 200 --fig4 .../Figure_4.xlsx --crate-dir .
```

Result:

| Case | D8 `NS_trace` | peak ratio | openWEPP `t_peak` | rise 10-90% | D8 closure |
|---|---:|---:|---:|---:|---|
| 1 | 0.868483 | 1.066 | 18000 s | 4999.7 s vs 3579.9 s | Green-Ampt operand-limited |
| 2 | 0.453954 | 0.747 | 10800 s | 3930.6 s vs 3021.0 s | `Ks` operand-limited |
| 3 | 0.537727 | 0.547 | 3600 s | 1449.3 s vs 1631.5 s | comparator-surface / operand boundary |
| 4 (`k_o=200`) | 0.262677 | 0.837 | 37 s | 29.4 s vs 20.9 s | shock-capture numerics boundary |

The Case 4 change is intentional: D8 corrected solver hydrograph sampling to
interpolate within solver steps. The D7 Case 4 "timing/rise reproduce" result
was partly a sampled-hydrograph attribution artifact.

## Shadow-First Check

Ran:

```console
rg -n "ofe_routing" crates/openwepp-runner \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime \
  crates/openwepp-hillslope-orchestrator/src/runtime_inputs \
  crates/openwepp-hillslope-orchestrator/src/hydrology
```

Result: no matches (`rg` exit 1). The routing subsystem remains shadow-first;
no production runner/direct-runtime/hydrology path calls it.

## Line Count

Ran:

```console
wc -l crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs \
  crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs \
  crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs \
  crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs \
  crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs \
  crates/openwepp-hillslope-orchestrator/examples/dval_case.rs
```

Largest touched Rust file: `kinematic_wave.rs`, 1002 lines. No 2000-line warning.
