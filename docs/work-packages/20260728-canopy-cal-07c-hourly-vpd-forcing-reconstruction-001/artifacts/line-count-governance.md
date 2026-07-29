# CAL-07C Line-Count Governance

Evidence class: `Ran`

Command:

```text
find docs/work-packages/20260728-canopy-cal-07c-hourly-vpd-forcing-reconstruction-001 -type f \( -name '*.rs' -o -name '*.py' -o -name '*.md' \) -print0 | xargs -0 wc -l
```

Largest source/documentation files:

| File | Lines | Disposition |
| --- | ---: | --- |
| `tools/plot.py` | 489 | PASS |
| `tools/prepare_inputs.py` | 486 | PASS |
| `tools/analyze.py` | 400 | PASS |
| `tools/validate.py` | 245 | PASS |
| `package.md` | 177 | PASS |
| `tools/executor/src/main.rs` | 152 | PASS |

No `.rs` file exceeds the 2,000-line warning threshold. No 3,000-line
refactor gate is triggered.
