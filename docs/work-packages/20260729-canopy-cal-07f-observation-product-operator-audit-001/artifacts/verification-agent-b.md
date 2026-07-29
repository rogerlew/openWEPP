# CAL-07F Terminal Verification B

Evidence class: `Ran + Static`

Terminal verdict: `PASS`

## Independent commands and results

### Package validator

Ran:

```text
.venv/bin/python \
  docs/work-packages/20260729-canopy-cal-07f-observation-product-operator-audit-001/tools/validate.py
```

Result:

```text
CAL-07F validation PASS: 6 dependencies, 731 daily rows, 24 source
transitions, 888 relative comparisons, 296 absolute comparisons, 74 member
summaries, 3 figure/sidecar pairs, decision=DO_NOT_RECOMMEND
```

I reran this after the validator's terminal anti-evasion correction.

### Syntax and diff hygiene

Ran:

```text
.venv/bin/python -m py_compile \
  docs/work-packages/20260729-canopy-cal-07f-observation-product-operator-audit-001/tools/*.py
git diff --check
```

Result: both passed with no output. Package-local Python files are 785, 266,
and 495 lines; none reaches the 2,000-line warning threshold.

### Independent source and metric reconstruction

Using separate read-only CSV reconstruction code, I verified:

| Inventory or metric | Reconstructed result |
| --- | ---: |
| Dependency rows with matching size and SHA-256 | 6/6 |
| Daily product rows | 731 |
| Source transition rows | 24 |
| Relative comparison rows | 888 |
| Absolute comparison rows | 296 |
| Member summary rows | 74 |
| Distinct frozen members | 37 |
| Product rank rows | 37 |
| Complete members in either frozen seasonal product lane | 0 |
| Members passing uncertainty in both products | 0 |
| Members passing direction coherence in both products | 0 |
| Product Spearman rank correlation | 1.0 |
| Product top-quartile overlap | 100% |

The best joint penalized score was independently selected as `GSI-4831`:

- `gcc_mean`: 9/12 seasonal crossings, 1/12 CI hits, 59.124713-day penalized
  mean absolute residual;
- `gcc_90`: 9/12 seasonal crossings, 0/12 CI hits, 65.874713-day penalized
  mean absolute residual.

The independent event-level reconstruction also matched the retained
availability and medians:

| Event | Crossing availability | Median residual |
| --- | ---: | ---: |
| 2024 `gcc_mean` falling T10/T25/T50 | 37/37 each | -61.452 / -61.761 / -49.832 days |
| 2024 `gcc_90` falling T10/T25/T50 | 37/37 each | -63.452 / -59.761 / -37.832 days |
| 2024 rising T10/T25/T50, both products | 37/37, 1/37, 0/37 | approximately +45 / +12 / unavailable days |
| 2025 `gcc_mean` falling T10/T25/T50 | 37/37 each | -61.766 / -48.467 / -39.805 days |
| 2025 `gcc_90` falling T10/T25/T50 | 37/37 each | -90.766 / -91.467 / -60.805 days |
| 2025 rising T10/T25/T50, both products | 37/37, 8/37, 0/37 | +82.638 / +120.071 / unavailable days |

The four CAL-07D counterfactual scenarios independently reduced to zero
qualifying parameter-plausibility scenarios. The final seven-row decision
screen contains two passes, four failures, and
`CALIBRATION_ROUND=DO_NOT_RECOMMEND`.

### Seasonal-selection sensitivity

I independently removed the midpoint seasonal partition and allowed the
nearest same-direction crossing anywhere in the same calendar year. This
permissive stress test produced 37/37 complete members in each product, but
zero members passed the joint uncertainty and direction requirements. The
best member retained only 1/12 and 0/12 CI hits. This confirms that the
stop-loss does not depend on excluding wrong-season crossings.

### Figures and documentation

Ran:

- XML parsing and title/description/role checks through the package validator;
- raster rendering of all three SVGs with `rsvg-convert`;
- visual inspection of the rendered product-curve, residual-distribution, and
  calibration-screen figures; and
- independent resolution of relative Markdown links across the CAL-07F
  package and changed roadmap/catalog files.

Results:

- 3/3 SVGs rendered successfully;
- 3/3 carried accessible title, description, and image-role metadata;
- 3/3 had complete and accurate Markdown sidecars; and
- zero relative links were missing.

## Boundary verification

The terminal worktree changes remain within the package's declared write set:
CAL-07F, the CAL-07E acquisition-deferral record, the canopy roadmap, and the
work-package catalog. No production Rust, contract, ADR, dependency, runtime,
or protected source artifact changed.

The roadmap and catalog preserve Order 7 as not passed, call the evidence
provisional, reject another calibration round, and scope the limitation to
the assessed Bezà tropical dry-forest lane. The CAL-07E acquisition list
records all three items as deferred and retains concrete future reactivation
conditions.

## Closure statement

The machine-readable evidence, independent reconstruction, figures, and
written claims agree. No unsupported calibration, validation,
transferability, biological-operator, or replacement-process claim is made.
CAL-07F passes independent terminal verification with the stop-loss
disposition `DO_NOT_RECOMMEND`.

## Post-integration terminal recheck

Evidence class: `Ran + Static`

After closure integration, I reran:

```text
.venv/bin/python \
  docs/work-packages/20260729-canopy-cal-07f-observation-product-operator-audit-001/tools/validate.py
.venv/bin/python -m py_compile \
  docs/work-packages/20260729-canopy-cal-07f-observation-product-operator-audit-001/tools/*.py
git diff --check
git diff --cached --check
markdown-doc lint --staged
```

Results:

- the strengthened package validator passed with
  `decision=DO_NOT_RECOMMEND`;
- Python syntax and both diff-hygiene checks passed;
- staged Markdown lint validated 27 files with zero errors or warnings;
- the package contains exactly 38 nonignored files;
- no changed path falls outside the declared write set;
- the active prompt directory has zero files and the kickoff is archived;
- package, final disposition, roadmap, and catalog consistently say
  `complete / do not calibrate / ecosystem-model limitation adjudicated`;
- the validator requires all closure records, both review/verification pairs,
  archived prompt state, and terminal package status;
- all three regenerated SVGs contain no trailing whitespace; and
- updated line counts are 785, 270, and 511, all below the warning threshold.

Post-integration verdict: `PASS`. No final mismatch remains.
