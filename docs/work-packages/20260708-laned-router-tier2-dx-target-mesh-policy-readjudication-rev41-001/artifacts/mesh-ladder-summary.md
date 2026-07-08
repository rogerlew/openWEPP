# Mesh Ladder Summary

Status: PASS. Evidence mode: Ran.

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`
- Git HEAD: `abc69bdda5458dd5389902e61a7626213aaf54cb`
- Git status short:

```text
M docs/ROADMAP.md
 M docs/work-packages/README.md
?? docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/
```

| Member | Rung | Status | Failure phase | Failure day | Clamp/source | Wall | User | Solver steps | Trace rows | Outlet m3 | End storage m3 | Tail fold m3 | Pass tdet sum |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| h2637 | baseline_fixed10 | PASS | n/a | n/a | n/a | 0:43.07 | 43.02 | 10471938 | 13889 | 371256.0302455183 | 3167.322375757055 | 36426.08442024077 | 22.630504471427155 |
| h2637 | dx20 | PASS | n/a | n/a | n/a | 0:42.06 | 42.01 | 10471938 | 13889 | 371256.0302455183 | 3167.322375757055 | 36426.08442024077 | 22.630504471427155 |
| h2637 | dx10 | PASS | n/a | n/a | n/a | 0:41.85 | 41.78 | 10471938 | 13889 | 371256.0302455183 | 3167.322375757055 | 36426.08442024077 | 22.630504471427155 |
| h2637 | dx5 | PASS | n/a | n/a | n/a | 0:41.95 | 41.89 | 10471938 | 13889 | 371256.0302455183 | 3167.322375757055 | 36426.08442024077 | 22.630504471427155 |
| h2637 | dx2p5 | PASS | n/a | n/a | n/a | 0:49.42 | 49.38 | 11224761 | 13889 | 371251.64161629276 | 3171.711004977372 | 36432.68642135361 | 22.576177118738737 |
| h2637 | dx1p25 | PASS | n/a | n/a | n/a | 2:54.26 | 174.19 | 19391930 | 13889 | 371229.03800435783 | 3194.314616914706 | 36450.239818884336 | 22.566899591129847 |
| mn_corn_h4 | baseline_fixed10 | PASS | n/a | n/a | n/a | 0:00.59 | 0.57 | 43819 | 2557 | 4473.730010803796 | 36.73695518632866 | 9.223103700550634 | 0.0 |
| mn_corn_h4 | dx20 | PASS | n/a | n/a | n/a | 0:00.60 | 0.57 | 43819 | 2557 | 4473.730010803796 | 36.73695518632866 | 9.223103700550634 | 0.0 |
| mn_corn_h4 | dx10 | PASS | n/a | n/a | n/a | 0:00.62 | 0.60 | 43819 | 2557 | 4473.730010803796 | 36.73695518632866 | 9.223103700550634 | 0.0 |
| mn_corn_h4 | dx5 | PASS | n/a | n/a | n/a | 0:00.76 | 0.74 | 51324 | 2557 | 4473.6107436690945 | 36.856222321031304 | 9.23907663324175 | 0.0 |
| mn_corn_h4 | dx2p5 | PASS | n/a | n/a | n/a | 0:01.42 | 1.40 | 71475 | 2557 | 4473.470955961825 | 36.996010028298144 | 9.33378447223084 | 0.0 |
| mn_corn_h4 | dx1p25 | PASS | n/a | n/a | n/a | 0:05.05 | 5.02 | 149657 | 2557 | 4473.220365201316 | 37.24660078880961 | 9.371413833285201 | 0.0 |
| n_idaho_forest_h1 | baseline_fixed10 | PASS | n/a | n/a | n/a | 0:00.98 | 0.96 | 76900 | 1461 | 99723.53384606258 | 1240.1565430837632 | 1276.541438116453 | 11025.380260294452 |
| n_idaho_forest_h1 | dx20 | PASS | n/a | n/a | n/a | 0:01.54 | 1.52 | 95691 | 1461 | 99725.39021033366 | 1238.3001788127115 | 1274.774718573394 | 11022.098790837645 |
| n_idaho_forest_h1 | dx10 | PASS | n/a | n/a | n/a | 0:04.34 | 4.32 | 162332 | 1461 | 99725.84766563277 | 1237.842723513611 | 1276.6715841801406 | 11014.685729233779 |
| n_idaho_forest_h1 | dx5 | PASS | n/a | n/a | n/a | 0:21.19 | 21.17 | 403843 | 1461 | 99720.11903453514 | 1243.5713546111863 | 1277.3919840113401 | 11008.281015107408 |
| n_idaho_forest_h1 | dx2p5 | PASS | n/a | n/a | n/a | 1:42.35 | 102.32 | 984338 | 1461 | 99718.51090724528 | 1245.1794819011125 | 1275.7675683471623 | 11013.238484878631 |
| n_idaho_forest_h1 | dx1p25 | PASS | n/a | n/a | n/a | 8:08.71 | 488.61 | 2332338 | 1461 | 99717.47405980396 | 1246.2163293423257 | 1274.873651268581 | 11013.466478619624 |
| wa_cascades_forest_h1 | baseline_fixed10 | PASS | n/a | n/a | n/a | 0:15.98 | 15.93 | 3284995 | 10960 | 860565.5924347457 | 4835.117072647566 | 8875.01105406181 | 35134.70249005277 |
| wa_cascades_forest_h1 | dx20 | PASS | n/a | n/a | n/a | 0:16.03 | 15.98 | 3284995 | 10960 | 860565.5924347457 | 4835.117072647566 | 8875.01105406181 | 35134.70249005277 |
| wa_cascades_forest_h1 | dx10 | PASS | n/a | n/a | n/a | 0:18.56 | 18.51 | 3407141 | 10960 | 860561.166298837 | 4839.54320855543 | 8879.241049102877 | 35136.486449413875 |
| wa_cascades_forest_h1 | dx5 | PASS | n/a | n/a | n/a | 1:02.85 | 62.79 | 4891877 | 10960 | 860530.122226401 | 4870.587280992438 | 8913.530961605135 | 35147.595053719335 |
| wa_cascades_forest_h1 | dx2p5 | PASS | n/a | n/a | n/a | 4:20.62 | 260.53 | 8548214 | 10960 | 860494.4847690226 | 4906.224738371967 | 8930.553539079645 | 35151.290618501305 |
| wa_cascades_forest_h1 | dx1p25 | PASS | n/a | n/a | n/a | 19:14.96 | 1154.78 | 17051717 | 10960 | 860480.0378746358 | 4920.671632758908 | 8935.893443212211 | 35153.69157664854 |

Comparisons:

| Member | Role | Candidate | Reference | Outlet L1 rel | Shape max L1 | Shape >0.05 | End storage rel | Tail fold rel | Annual sed max rel |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| h2637 | fine_reference_adequacy | dx2p5 | dx1p25 | 6.33241e-05 | 0.00822469 | 0 | 6.03691e-05 | 4.68812e-05 | 0.000411112 |
| h2637 | candidate_vs_dx2p5_reference | baseline_fixed10 | dx2p5 | 2.49109e-05 | 0.00514851 | 0 | 1.1721e-05 | 1.76325e-05 | 0.0024064 |
| h2637 | candidate_vs_dx2p5_reference | dx20 | dx2p5 | 2.49109e-05 | 0.00514851 | 0 | 1.1721e-05 | 1.76325e-05 | 0.0024064 |
| h2637 | candidate_vs_dx2p5_reference | dx10 | dx2p5 | 2.49109e-05 | 0.00514851 | 0 | 1.1721e-05 | 1.76325e-05 | 0.0024064 |
| h2637 | candidate_vs_dx2p5_reference | dx5 | dx2p5 | 2.49109e-05 | 0.00514851 | 0 | 1.1721e-05 | 1.76325e-05 | 0.0024064 |
| mn_corn_h4 | fine_reference_adequacy | dx2p5 | dx1p25 | 5.85386e-05 | 0.0201805 | 0 | 5.55576e-05 | 8.34268e-06 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | baseline_fixed10 | dx2p5 | 9.69962e-05 | 0.0433405 | 0 | 5.74342e-05 | 2.45387e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | dx20 | dx2p5 | 9.69962e-05 | 0.0433405 | 0 | 5.74342e-05 | 2.45387e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | dx10 | dx2p5 | 9.69962e-05 | 0.0433405 | 0 | 5.74342e-05 | 2.45387e-05 | 0 |
| mn_corn_h4 | candidate_vs_dx2p5_reference | dx5 | dx2p5 | 4.19091e-05 | 0.0233599 | 0 | 3.09918e-05 | 2.09973e-05 | 0 |
| n_idaho_forest_h1 | fine_reference_adequacy | dx2p5 | dx1p25 | 2.80963e-05 | 0.0166187 | 0 | 1.02695e-05 | 8.85385e-06 | 0.0025208 |
| n_idaho_forest_h1 | candidate_vs_dx2p5_reference | baseline_fixed10 | dx2p5 | 6.48991e-05 | 0.0472952 | 0 | 4.975e-05 | 7.66483e-06 | 0.00378252 |
| n_idaho_forest_h1 | candidate_vs_dx2p5_reference | dx20 | dx2p5 | 7.21795e-05 | 0.0347288 | 0 | 6.81364e-05 | 9.83373e-06 | 0.0032515 |
| n_idaho_forest_h1 | candidate_vs_dx2p5_reference | dx10 | dx2p5 | 7.39786e-05 | 0.0281091 | 0 | 7.26673e-05 | 8.95387e-06 | 0.00285528 |
| n_idaho_forest_h1 | candidate_vs_dx2p5_reference | dx5 | dx2p5 | 4.37473e-05 | 0.0103118 | 0 | 1.59278e-05 | 1.60891e-05 | 0.0026601 |
| wa_cascades_forest_h1 | fine_reference_adequacy | dx2p5 | dx1p25 | 2.0838e-05 | 0.00919752 | 0 | 1.66939e-05 | 6.17044e-06 | 0.00636869 |
| wa_cascades_forest_h1 | candidate_vs_dx2p5_reference | baseline_fixed10 | dx2p5 | 8.41193e-05 | 0.0272542 | 0 | 8.21673e-05 | 6.41812e-05 | 0.0579996 |
| wa_cascades_forest_h1 | candidate_vs_dx2p5_reference | dx20 | dx2p5 | 8.41193e-05 | 0.0272542 | 0 | 8.21673e-05 | 6.41812e-05 | 0.0579996 |
| wa_cascades_forest_h1 | candidate_vs_dx2p5_reference | dx10 | dx2p5 | 7.88729e-05 | 0.0245771 | 0 | 7.70528e-05 | 5.92933e-05 | 0.0218909 |
| wa_cascades_forest_h1 | candidate_vs_dx2p5_reference | dx5 | dx2p5 | 4.27055e-05 | 0.0100052 | 0 | 4.11803e-05 | 1.96702e-05 | 0.018436 |

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/mesh-ladder-summary.json`
