# CAL-07D Terminal Verification B

Evidence class: `Ran + Static`

Verdict: `PASS / ORDER 7 HOLD RETAINED`

## Commands run

From `/home/workdir/openWEPP`:

```text
.venv/bin/python \
  docs/work-packages/20260729-canopy-cal-07d-transition-chronology-attribution-001/tools/plot.py

.venv/bin/python \
  docs/work-packages/20260729-canopy-cal-07d-transition-chronology-attribution-001/tools/validate.py
```

Result:

```text
CAL-07D figures PASS: 4 SVG plots and 4 Markdown sidecars
CAL-07D validation PASS: 61,642 BASE rows independently reconstructed;
maximum equation residual=0.000e+00; 9,996 scenario-days and every
event/crossing row reproduced; 11 CAL-07C matches reproduced;
4 SVG/sidecar pairs verified
```

```text
.venv/bin/python -m py_compile \
  docs/work-packages/20260729-canopy-cal-07d-transition-chronology-attribution-001/tools/*.py
```

Result: `PASS`.

```text
xmllint --noout <each CAL-07D SVG>
rsvg-convert <each CAL-07D SVG> -o <temporary PNG>
```

Result: all four SVG files parsed and rendered successfully.

```text
markdown-doc lint --path \
  docs/work-packages/20260729-canopy-cal-07d-transition-chronology-attribution-001
markdown-doc lint --path \
  docs/planning/canopy-phenology-assurance-roadmap.md
markdown-doc lint --path docs/work-packages/README.md
```

Result:

```text
24 files validated, 0 errors, 0 warnings
1 files validated, 0 errors, 0 warnings
1 files validated, 0 errors, 0 warnings
```

```text
git diff --check
```

Result: `PASS`.

Direct commit-object SHA-256 verification from the initial review also remains
valid for all ten dependency-manifest entries at
`11b1faab37b5d365b0c0c7051ed32a4762821239`.

## Verification matrix

| Surface | Result | Evidence |
| --- | --- | --- |
| Dependency custody | `PASS` | Ten exact commit blobs, byte sizes, and SHA-256 identities verified. |
| BASE equations/FIFO | `PASS` | 61,642 ordered member-days reconstructed independently; maximum residual `0.000e+00`. |
| Scenario isolation | `PASS` | Five 37-member scenarios plus one default rebuilt from day one with only declared substitutions. |
| Scenario summaries | `PASS` | All 9,996 `(scenario,date)` rows reproduced. |
| Fixed thresholds | `PASS` | Complete event-year extrema used once per event/member/level; threshold held across each full event window. |
| Crossing semantics | `PASS` | Direction inequalities, plateau behavior, interpolation, lower-open/upper-closed windows, and first crossing independently reapplied. |
| Crossing inventory | `PASS` | All 34,480 global crossing rows reproduced by full key and content. |
| Event tables | `PASS` | Exact 148/1,628/444/1,488 row inventories independently reproduced, including unmatched blanks and crossing counts. |
| Observation support | `PASS` | Twelve source-support and twelve event-attribution rows independently rebuilt from retained source-native tables. |
| Decision predicates | `PASS` | All seven statuses and numeric predicate values independently reduced. |
| Figure bindings | `PASS` | Four embedded source/hash/row/field/fingerprint records recomputed from bound CSVs. |
| Sidecars | `PASS` | Exact keys, counts, selection/unmatched semantics, digests, assumptions, ceilings, limitations, and accessibility retained. |
| Documentation and hygiene | `PASS` | Python syntax, Markdown lint, SVG XML/render, and diff hygiene passed. |

## Claim and closure verification

The evidence supports diagnostic attribution only. It does not identify a
correct tropical dry-forest parameterization, quantify POWER forcing bias,
validate an omitted process, equate GCC with GSI, or authorize a production
change. Order 7 therefore correctly remains on scientific hold.

No reviewer-owned closure blocker remains. Package integration still must
update `gate-evidence.md` and `finding-disposition.md`, reconcile the exact
terminal diff, and author final disposition after both terminal reviews are
present. Those records should retain the scientific Order 7 hold while
closing CAL-07D itself as a successfully executed diagnostic package.
