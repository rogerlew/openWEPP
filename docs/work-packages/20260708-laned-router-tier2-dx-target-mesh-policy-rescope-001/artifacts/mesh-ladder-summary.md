# Mesh Ladder Summary

Status: FAIL. Evidence mode: Ran.

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `9a4f9c2755723c2e312dea460ed714bb183e283968fef2f003cf7690a71d48b8`
- Git HEAD: `ec82f061a18db352ce5efab52e1b04eed5de3701`
- Git status short:

```text
M crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 M crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs
 M crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs
 M crates/openwepp-hillslope-orchestrator/src/lib.rs
 M crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
 M crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs
 M crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
 M crates/openwepp-runner/src/hillslope/laned_active.rs
 M docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
?? docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-runs/
?? docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/run_mesh_ladder.py
```

| Member | Rung | Status | Wall | User | Solver steps | Trace rows | Outlet m3 | End storage m3 | Tail fold m3 | Pass tdet sum |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| h2637 | baseline_fixed10 | PASS | 0:40.57 | 40.51 | 10479200 | 13889 | 374463.0826831916 | 3167.32249029797 | 36426.08375542731 | 23.0500822964053 |
| h2637 | dx20 | PASS | 0:39.76 | 39.69 | 10479200 | 13889 | 374463.0826831916 | 3167.32249029797 | 36426.08375542731 | 23.0500822964053 |
| h2637 | dx10 | PASS | 0:40.20 | 40.13 | 10479200 | 13889 | 374463.0826831916 | 3167.32249029797 | 36426.08375542731 | 23.0500822964053 |
| h2637 | dx5 | PASS | 0:40.00 | 39.94 | 10479200 | 13889 | 374463.0826831916 | 3167.32249029797 | 36426.08375542731 | 23.0500822964053 |
| h2637 | dx2p5 | PASS | 0:47.48 | 47.43 | 11228022 | 13889 | 374191.34221530665 | 3171.7042817507577 | 36432.69143688323 | 23.24119090198076 |
| h2637 | dx1p25 | PASS | 2:46.00 | 165.92 | 19393405 | 13889 | 375444.3590240577 | 3194.3152615475215 | 36450.24161778788 | 25.16133815364221 |
| mn_corn_h4 | baseline_fixed10 | PASS | 0:00.59 | 0.57 | 43819 | 2557 | 4473.730010803795 | 36.73695518632866 | 9.22310370055062 | 0.0 |
| mn_corn_h4 | dx20 | PASS | 0:00.59 | 0.56 | 43819 | 2557 | 4473.730010803795 | 36.73695518632866 | 9.22310370055062 | 0.0 |
| mn_corn_h4 | dx10 | PASS | 0:00.58 | 0.56 | 43819 | 2557 | 4473.730010803795 | 36.73695518632866 | 9.22310370055062 | 0.0 |
| mn_corn_h4 | dx5 | PASS | 0:00.72 | 0.70 | 51324 | 2557 | 4473.610743669093 | 36.8562223210313 | 9.23907663324174 | 0.0 |
| mn_corn_h4 | dx2p5 | PASS | 0:01.35 | 1.32 | 71475 | 2557 | 4473.470955961828 | 36.99601002829833 | 9.33378447223236 | 0.0 |
| mn_corn_h4 | dx1p25 | PASS | 0:04.90 | 4.88 | 149657 | 2557 | 4473.22036520137 | 37.24660078876023 | 9.37141383328436 | 0.0 |
| n_idaho_forest_h1 | baseline_fixed10 | PASS | 0:01.01 | 0.99 | 76900 | 1461 | 99723.5338460626 | 1240.1565430837632 | 1276.5414381164533 | 11025.38026029445 |
| n_idaho_forest_h1 | dx20 | PASS | 0:01.49 | 1.48 | 95691 | 1461 | 99725.39021033363 | 1238.3001788127115 | 1274.7747185733947 | 11022.098790837646 |
| n_idaho_forest_h1 | dx10 | PASS | 0:04.34 | 4.32 | 162332 | 1461 | 99725.84766563277 | 1237.842723513616 | 1276.6715841801395 | 11014.685729233779 |
| n_idaho_forest_h1 | dx5 | PASS | 0:20.65 | 20.63 | 403843 | 1461 | 99720.11900774238 | 1243.5713814039661 | 1277.3919273861607 | 11008.28092325774 |
| n_idaho_forest_h1 | dx2p5 | PASS | 1:41.17 | 101.14 | 984337 | 1461 | 99718.51017221969 | 1245.1802169267025 | 1275.7518812323917 | 11013.238477043733 |
| n_idaho_forest_h1 | dx1p25 | PASS | 8:03.47 | 483.38 | 2332425 | 1461 | 99717.42307921124 | 1246.3080640000105 | 1274.8143543250096 | 11013.483912104046 |
| wa_cascades_forest_h1 | baseline_fixed10 | PASS | 0:15.73 | 15.67 | 3285282 | 10960 | 1007798.7596702089 | 4835.1165998893775 | 8875.011254557006 | 35137.0467081843 |
| wa_cascades_forest_h1 | dx20 | PASS | 0:15.72 | 15.66 | 3285282 | 10960 | 1007798.7596702089 | 4835.1165998893775 | 8875.011254557006 | 35137.0467081843 |
| wa_cascades_forest_h1 | dx10 | PASS | 0:18.61 | 18.58 | 3407233 | 10960 | 455174146.416385 | 3234571.309778146 | 8879.24245651698 | 35138.499163106775 |
| wa_cascades_forest_h1 | dx5 | PASS | 1:01.90 | 61.86 | 4884752 | 10960 | 27678112025.381126 | 148799857.05027804 | 8912.719227392241 | 34695.208448201076 |
| wa_cascades_forest_h1 | dx2p5 | FAIL | 2:18.96 | 138.89 | n/a | n/a | n/a | n/a | n/a | n/a |
| wa_cascades_forest_h1 | dx1p25 | FAIL | 10:13.44 | 613.36 | n/a | n/a | n/a | n/a | n/a | n/a |

Comparisons:

| Member | Role | Candidate | Reference | Outlet L1 rel | Shape max L1 | Shape >0.05 | End storage rel | Tail fold rel | Annual sed max rel |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| h2637 | fine_reference_adequacy | dx2p5 | dx1p25 | 0.0091233 | 0.0730917 | 6 | 6.03888e-05 | 4.68726e-05 | 0.0763134 |
| h2637 | candidate_vs_dx2p5_reference | baseline_fixed10 | dx2p5 | 0.00258527 | 0.0553084 | 1 | 1.17028e-05 | 1.76476e-05 | 0.0669818 |
| h2637 | candidate_vs_dx2p5_reference | dx20 | dx2p5 | 0.00258527 | 0.0553084 | 1 | 1.17028e-05 | 1.76476e-05 | 0.0669818 |
| h2637 | candidate_vs_dx2p5_reference | dx10 | dx2p5 | 0.00258527 | 0.0553084 | 1 | 1.17028e-05 | 1.76476e-05 | 0.0669818 |
| h2637 | candidate_vs_dx2p5_reference | dx5 | dx2p5 | 0.00258527 | 0.0553084 | 1 | 1.17028e-05 | 1.76476e-05 | 0.0669818 |
| mn_corn_h4 | fine_reference_adequacy | dx2p5 | dx1p25 | 5.85386e-05 | 0.0201805 | 0 | 5.55576e-05 | 8.34268e-06 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | baseline_fixed10 | dx2p5 | 9.69962e-05 | 0.0433405 | 0 | 5.74342e-05 | 2.45387e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | dx20 | dx2p5 | 9.69962e-05 | 0.0433405 | 0 | 5.74342e-05 | 2.45387e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | dx10 | dx2p5 | 9.69962e-05 | 0.0433405 | 0 | 5.74342e-05 | 2.45387e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | dx5 | dx2p5 | 4.19091e-05 | 0.0233599 | 0 | 3.09918e-05 | 2.09973e-05 | 0 |
| n_idaho_forest_h1 | fine_reference_adequacy | dx2p5 | dx1p25 | 2.86582e-05 | 0.0166187 | 0 | 1.11708e-05 | 9.28578e-06 | 0.00252249 |
| n_idaho_forest_h1 | candidate_vs_dx2p5_reference | baseline_fixed10 | dx2p5 | 6.49065e-05 | 0.0472952 | 0 | 4.97572e-05 | 7.82021e-06 | 0.00378251 |
| n_idaho_forest_h1 | candidate_vs_dx2p5_reference | dx20 | dx2p5 | 7.21869e-05 | 0.0347288 | 0 | 6.81437e-05 | 9.67836e-06 | 0.0032515 |
| n_idaho_forest_h1 | candidate_vs_dx2p5_reference | dx10 | dx2p5 | 7.3986e-05 | 0.0281091 | 0 | 7.26746e-05 | 9.10924e-06 | 0.00285528 |
| n_idaho_forest_h1 | candidate_vs_dx2p5_reference | dx5 | dx2p5 | 4.38317e-05 | 0.0103118 | 0 | 1.59348e-05 | 1.62439e-05 | 0.00266012 |
| wa_cascades_forest_h1 | fine_reference_adequacy: dx2p5 or dx1p25 reference rung failed or lacks trace output | SKIPPED | SKIPPED | n/a | n/a | n/a | n/a | n/a | n/a |

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-summary.json`

