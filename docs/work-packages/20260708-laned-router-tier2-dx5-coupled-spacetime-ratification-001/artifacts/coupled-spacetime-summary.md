# Coupled Space-Time Ladder Summary

Status: PASS. Evidence mode: Ran.

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `8876fa04ca520126b958d83a7c5777da6f793e51fba4c346432f065b31647aaa`
- Git HEAD: `8faa56b43ed42f54fd40e64a94a002ad372240cc`
- Git status short:

```text
M docs/ROADMAP.md
 M docs/work-packages/README.md
?? docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/
```

| Member | Rung | Status | Target dx m | Max dt s | Failure phase | Failure day | Clamp/source | Wall | User | Solver steps | Trace rows | Outlet m3 | End storage m3 | Tail fold m3 | Pass tdet sum |
|---|---|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| mn_corn_h4 | baseline_fixed10_dt300 | PASS | fixed10 | 300.0 | n/a | n/a | n/a | 0:00.64 | 0.61 | 43819 | 2557 | 4473.730010803796 | 36.73695518632866 | 9.223103700550634 | 0.0 |
| mn_corn_h4 | dx5_dt300 | PASS | 5.0 | 300.0 | n/a | n/a | n/a | 0:00.79 | 0.77 | 51324 | 2557 | 4473.6107436690945 | 36.856222321031304 | 9.23907663324175 | 0.0 |
| mn_corn_h4 | dx2p5_dt300 | PASS | 2.5 | 300.0 | n/a | n/a | n/a | 0:01.50 | 1.48 | 71475 | 2557 | 4473.470955961825 | 36.996010028298144 | 9.33378447223084 | 0.0 |
| mn_corn_h4 | dx1p25_dt300 | PASS | 1.25 | 300.0 | n/a | n/a | n/a | 0:07.51 | 5.36 | 149657 | 2557 | 4473.220365201316 | 37.24660078880961 | 9.371413833285201 | 0.0 |
| mn_corn_h4 | dx5_dt75 | PASS | 5.0 | 75.0 | n/a | n/a | n/a | 0:01.17 | 1.15 | 152314 | 2557 | 4473.180363632298 | 37.2866023578252 | 9.333762462966805 | 0.0 |
| mn_corn_h4 | dx2p5_dt75 | PASS | 2.5 | 75.0 | n/a | n/a | n/a | 0:02.22 | 2.20 | 171648 | 2557 | 4473.193207480465 | 37.27375850965942 | 9.339549919361962 | 0.0 |
| mn_corn_h4 | dx1p25_dt75 | PASS | 1.25 | 75.0 | n/a | n/a | n/a | 0:06.76 | 6.73 | 242245 | 2557 | 4473.157108191842 | 37.30985779828603 | 9.337578661889781 | 0.0 |
| n_idaho_forest_h1 | baseline_fixed10_dt300 | PASS | fixed10 | 300.0 | n/a | n/a | n/a | 0:01.12 | 1.10 | 76900 | 1461 | 99723.53384606258 | 1240.1565430837632 | 1276.541438116453 | 11025.380260294452 |
| n_idaho_forest_h1 | dx5_dt300 | PASS | 5.0 | 300.0 | n/a | n/a | n/a | 0:22.91 | 22.87 | 403843 | 1461 | 99720.11903453514 | 1243.5713546111863 | 1277.3919840113401 | 11008.281015107408 |
| n_idaho_forest_h1 | dx2p5_dt300 | PASS | 2.5 | 300.0 | n/a | n/a | n/a | 1:46.62 | 106.58 | 984338 | 1461 | 99718.51090724528 | 1245.1794819011125 | 1275.7675683471623 | 11013.238484878631 |
| n_idaho_forest_h1 | dx1p25_dt300 | PASS | 1.25 | 300.0 | n/a | n/a | n/a | 8:30.02 | 509.85 | 2332338 | 1461 | 99717.47405980396 | 1246.2163293423257 | 1274.873651268581 | 11013.466478619624 |
| n_idaho_forest_h1 | dx5_dt75 | PASS | 5.0 | 75.0 | n/a | n/a | n/a | 0:24.28 | 24.25 | 479067 | 1461 | 99717.51551904806 | 1246.1748700983394 | 1275.3436544769881 | 11008.48575324284 |
| n_idaho_forest_h1 | dx2p5_dt75 | PASS | 2.5 | 75.0 | n/a | n/a | n/a | 1:49.42 | 109.33 | 1038448 | 1461 | 99717.48786636903 | 1246.2025227774282 | 1275.7582004463266 | 11012.502669623058 |
| n_idaho_forest_h1 | dx1p25_dt75 | PASS | 1.25 | 75.0 | n/a | n/a | n/a | 8:52.56 | 532.44 | 2382081 | 1461 | 99717.2159841919 | 1246.4744049545034 | 1275.176874007264 | 11019.40859302321 |
| wa_cascades_forest_h1 | baseline_fixed10_dt300 | PASS | fixed10 | 300.0 | n/a | n/a | n/a | 0:16.51 | 16.47 | 3284995 | 10960 | 860565.5924347457 | 4835.117072647566 | 8875.01105406181 | 35134.70249005277 |
| wa_cascades_forest_h1 | dx5_dt300 | PASS | 5.0 | 300.0 | n/a | n/a | n/a | 1:05.11 | 65.04 | 4891877 | 10960 | 860530.122226401 | 4870.587280992438 | 8913.530961605135 | 35147.595053719335 |
| wa_cascades_forest_h1 | dx2p5_dt300 | PASS | 2.5 | 300.0 | n/a | n/a | n/a | 4:34.52 | 274.42 | 8548214 | 10960 | 860494.4847690226 | 4906.224738371967 | 8930.553539079645 | 35151.290618501305 |
| wa_cascades_forest_h1 | dx1p25_dt300 | PASS | 1.25 | 300.0 | n/a | n/a | n/a | 20:13.19 | 1212.72 | 17051717 | 10960 | 860480.0378746358 | 4920.671632758908 | 8935.893443212211 | 35153.69157664854 |
| wa_cascades_forest_h1 | dx5_dt75 | PASS | 5.0 | 75.0 | n/a | n/a | n/a | 1:32.64 | 92.56 | 11375624 | 10960 | 860486.058325815 | 4914.651181576765 | 8904.423222373282 | 35148.95082163485 |
| wa_cascades_forest_h1 | dx2p5_dt75 | PASS | 2.5 | 75.0 | n/a | n/a | n/a | 5:16.60 | 316.44 | 14890835 | 10960 | 860482.2797448048 | 4918.429762587639 | 8921.778697092246 | 35153.57381749202 |
| wa_cascades_forest_h1 | dx1p25_dt75 | PASS | 1.25 | 75.0 | n/a | n/a | n/a | 21:27.99 | 1287.65 | 23293112 | 10960 | 860477.4565319989 | 4923.252975395026 | 8933.370862032369 | 35155.19426294733 |

Comparisons:

| Member | Role | Candidate | Reference | Outlet L1 rel | Shape max L1 | Shape >0.05 | End storage rel | Tail fold rel | Annual sed max rel |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| mn_corn_h4 | fine_reference_adequacy_dt300 | dx2p5_dt300 | dx1p25_dt300 | 5.85386e-05 | 0.0201805 | 0 | 5.55576e-05 | 8.34268e-06 | 0 |
| mn_corn_h4 | fine_reference_adequacy_dt75 | dx2p5_dt75 | dx1p25_dt75 | 1.47068e-05 | 0.0116183 | 0 | 8.00345e-06 | 4.37041e-07 | 0 |
| mn_corn_h4 | candidate_vs_reference_dt300 | dx5_dt300 | dx2p5_dt300 | 4.19091e-05 | 0.0233599 | 0 | 3.09918e-05 | 2.09973e-05 | 0 |
| mn_corn_h4 | candidate_vs_reference_dt75 | dx5_dt75 | dx2p5_dt75 | 2.1205e-05 | 0.0142255 | 0 | 2.84757e-06 | 1.28312e-06 | 0 |
| mn_corn_h4 | timestep_control_dx5 | dx5_dt300 | dx5_dt75 | 9.62138e-05 | 0.0315264 | 0 | 9.54181e-05 | 2.09925e-05 | 0 |
| mn_corn_h4 | timestep_control_dx2p5 | dx2p5_dt300 | dx2p5_dt75 | 6.21105e-05 | 0.0246954 | 0 | 6.15787e-05 | 1.27824e-06 | 0 |
| mn_corn_h4 | timestep_control_dx1p25 | dx1p25_dt300 | dx1p25_dt75 | 2.21467e-05 | 0.0343551 | 0 | 1.40245e-05 | 7.50148e-06 | 0 |
| n_idaho_forest_h1 | fine_reference_adequacy_dt300 | dx2p5_dt300 | dx1p25_dt300 | 2.80963e-05 | 0.0166187 | 0 | 1.02695e-05 | 8.85385e-06 | 0.0025208 |
| n_idaho_forest_h1 | fine_reference_adequacy_dt75 | dx2p5_dt75 | dx1p25_dt75 | 1.91787e-05 | 0.0159605 | 0 | 2.69287e-06 | 5.75778e-06 | 0.00151623 |
| n_idaho_forest_h1 | candidate_vs_reference_dt300 | dx5_dt300 | dx2p5_dt300 | 4.37473e-05 | 0.0103118 | 0 | 1.59278e-05 | 1.60891e-05 | 0.0026601 |
| n_idaho_forest_h1 | candidate_vs_reference_dt75 | dx5_dt75 | dx2p5_dt75 | 1.43867e-05 | 0.0116165 | 0 | 2.73887e-07 | 4.10589e-06 | 0.00246832 |
| n_idaho_forest_h1 | timestep_control_dx5 | dx5_dt300 | dx5_dt75 | 3.40608e-05 | 0.0147606 | 0 | 2.57867e-05 | 2.02878e-05 | 2.0645e-05 |
| n_idaho_forest_h1 | timestep_control_dx2p5 | dx2p5_dt300 | dx2p5_dt75 | 2.54918e-05 | 0.0166021 | 0 | 1.01328e-05 | 9.27848e-08 | 0.000180106 |
| n_idaho_forest_h1 | timestep_control_dx1p25 | dx1p25_dt300 | dx1p25_dt75 | 2.02622e-05 | 0.0135634 | 0 | 2.55612e-06 | 3.00329e-06 | 0.00270898 |
| wa_cascades_forest_h1 | fine_reference_adequacy_dt300 | dx2p5_dt300 | dx1p25_dt300 | 2.0838e-05 | 0.00919752 | 0 | 1.66939e-05 | 6.17044e-06 | 0.00636869 |
| wa_cascades_forest_h1 | fine_reference_adequacy_dt75 | dx2p5_dt75 | dx1p25_dt75 | 1.21543e-05 | 0.00723149 | 0 | 5.57339e-06 | 1.33951e-05 | 0.0221317 |
| wa_cascades_forest_h1 | candidate_vs_reference_dt300 | dx5_dt300 | dx2p5_dt300 | 4.27055e-05 | 0.0100052 | 0 | 4.11803e-05 | 1.96702e-05 | 0.018436 |
| wa_cascades_forest_h1 | candidate_vs_reference_dt75 | dx5_dt75 | dx2p5_dt75 | 1.22048e-05 | 0.00536522 | 0 | 4.36628e-06 | 2.00548e-05 | 0.0181162 |
| wa_cascades_forest_h1 | timestep_control_dx5 | dx5_dt300 | dx5_dt75 | 5.1219e-05 | 0.0113084 | 0 | 5.09173e-05 | 1.05243e-05 | 0.00872331 |
| wa_cascades_forest_h1 | timestep_control_dx2p5 | dx2p5_dt300 | dx2p5_dt75 | 1.6154e-05 | 0.0130986 | 0 | 1.41033e-05 | 1.01396e-05 | 0.018583 |
| wa_cascades_forest_h1 | timestep_control_dx1p25 | dx1p25_dt300 | dx1p25_dt75 | 9.06283e-06 | 0.0109888 | 0 | 2.98283e-06 | 2.91493e-06 | 0.00760141 |

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/coupled-spacetime-summary.json`
