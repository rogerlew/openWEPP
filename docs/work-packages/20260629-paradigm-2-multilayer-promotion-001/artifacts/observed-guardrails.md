# Observed Guardrails

Status: `RAN-PASS`

Evidence class: Ran + Static.

This artifact records the real cross-SNOTEL/cancov snow-neutral guardrail rerun
for the production-supported opt-in selector.

## Command

```bash
.venv/bin/python tools/snowfreeze_observed/paradigm2_stage3_decouple_water_temperature.py \
  --output-dir target/paradigm2_multilayer_promotion/observed \
  --package-artifacts-dir docs/work-packages/20260629-paradigm-2-multilayer-promotion-001/artifacts \
  --hill-binary target/release/openwepp-cli-hill
```

The first attempt used system Python and failed before scoring because `pyarrow`
was unavailable. The command above is the successful run, using the freshly
rebuilt release binary.

## Result

| Gate | Result |
| --- | --- |
| Current no-env default robust profile | `15` fails / `179` score |
| Promoted opt-in robust profile | `15` fails / `179` score |
| Better robust cells | `0` |
| Worse robust cells | `0` |
| Runoff/timing worse robust cells | `0` |
| Elapsed | `111.857 s` |

The snow-neutral hard gate passed exactly: promoted arm == current default with
no worse robust cells.

Raw machine artifact:
`artifacts/paradigm2-stage3-decouple-observed-guardrails.json`.
