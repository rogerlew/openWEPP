# Fine-Reference Hold-Lift Summary

Status: RUN-COMPLETION-PASS. Evidence mode: Ran.

The status above means every requested process run completed. The strict
fine-reference adequacy verdict is recorded separately in
`fine-reference-adequacy.md`.

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`
- Git HEAD: `25a9f52d2b6dba7d18188d2e0d0523c4f0d7f6a1`
- Git status short:

```text
M docs/ROADMAP.md
 M docs/work-packages/README.md
?? docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001/
```

| Member | Rung | Status | Failure phase | Failure day | Clamp/source | Wall | User | Solver steps | Trace rows | Outlet m3 | End storage m3 | Tail fold m3 | Pass tdet sum |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| mn_corn_h4 | baseline_fixed10 | PASS | n/a | n/a | n/a | 0:00.59 | 0.57 | 43819 | 2557 | 4473.730010803796 | 36.73695518632866 | 9.223103700550634 | 0.0 |
| mn_corn_h4 | dx20 | PASS | n/a | n/a | n/a | 0:00.58 | 0.56 | 43819 | 2557 | 4473.730010803796 | 36.73695518632866 | 9.223103700550634 | 0.0 |
| mn_corn_h4 | dx10 | PASS | n/a | n/a | n/a | 0:00.56 | 0.54 | 43819 | 2557 | 4473.730010803796 | 36.73695518632866 | 9.223103700550634 | 0.0 |
| mn_corn_h4 | dx5 | PASS | n/a | n/a | n/a | 0:00.72 | 0.70 | 51324 | 2557 | 4473.6107436690945 | 36.856222321031304 | 9.23907663324175 | 0.0 |
| mn_corn_h4 | dx2p5 | PASS | n/a | n/a | n/a | 0:01.39 | 1.37 | 71475 | 2557 | 4473.470955961825 | 36.996010028298144 | 9.33378447223084 | 0.0 |
| mn_corn_h4 | dx1p25 | PASS | n/a | n/a | n/a | 0:05.10 | 5.08 | 149657 | 2557 | 4473.220365201316 | 37.24660078880961 | 9.371413833285201 | 0.0 |
| mn_corn_h4 | dx0p625 | PASS | n/a | n/a | n/a | 0:21.82 | 21.79 | 319784 | 2557 | 4473.1528822893215 | 37.31408370078956 | 9.35568629157479 | 0.0 |

Comparisons:

| Member | Role | Candidate | Reference | Outlet L1 rel | Shape max L1 | Shape >0.05 | End storage rel | Tail fold rel | Annual sed max rel |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| mn_corn_h4 | fine_reference_adequacy | dx2p5 | dx1p25 | 5.85386e-05 | 0.0201805 | 0 | 5.55576e-05 | 8.34268e-06 | 0 |
| mn_corn_h4 | fine_reference_adequacy_dx1p25_vs_dx0p625 | dx1p25 | dx0p625 | 2.80622e-05 | 0.0209449 | 0 | 1.49614e-05 | 3.4869e-06 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | baseline_fixed10 | dx2p5 | 9.69962e-05 | 0.0433405 | 0 | 5.74342e-05 | 2.45387e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | dx20 | dx2p5 | 9.69962e-05 | 0.0433405 | 0 | 5.74342e-05 | 2.45387e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | dx10 | dx2p5 | 9.69962e-05 | 0.0433405 | 0 | 5.74342e-05 | 2.45387e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | dx5 | dx2p5 | 4.19091e-05 | 0.0233599 | 0 | 3.09918e-05 | 2.09973e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx1p25_reference | baseline_fixed10 | dx1p25 | 0.000139554 | 0.0617802 | 1 | 0.000112992 | 3.28813e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx1p25_reference | dx20 | dx1p25 | 0.000139554 | 0.0617802 | 1 | 0.000112992 | 3.28813e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx1p25_reference | dx10 | dx1p25 | 0.000139554 | 0.0617802 | 1 | 0.000112992 | 3.28813e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx1p25_reference | dx5 | dx1p25 | 9.75407e-05 | 0.0434886 | 0 | 8.65495e-05 | 2.934e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx1p25_reference | dx2p5 | dx1p25 | 5.85386e-05 | 0.0201805 | 0 | 5.55576e-05 | 8.34268e-06 | 0 |

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001/artifacts/fine-reference-summary.json`
