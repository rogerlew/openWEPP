# Mesh-Policy Ratification

Evidence mode: Ran.

Status: `EXECUTED-HOLD-DX5-UNRATIFIED`

`dx5` is not production-ratified by this package because at least one blocking coupled comparison failed.

## Runtime Cost

| Rung | Aggregate real-cohort user seconds |
|---|---:|
| `baseline_fixed10_dt300` | `18.18` |
| `dx5_dt300` | `88.68` |
| `dx5 / fixed10` | `4.8778878` |

Cost is priced evidence only and is not used as a fidelity blocker.

## Blocking Comparisons

### fine_reference_adequacy_dt300

| Member | Candidate | Reference | Outlet rel | Shape L1 | End storage rel | Tail fold rel | Annual sed rel | Verdict | Failures |
|---|---|---|---:|---:|---:|---:|---:|---|---|
| mn_corn_h4 | `dx2p5_dt300` | `dx1p25_dt300` | `5.8538553e-05` | `0.020180511` | `5.5557609e-05` | `8.3426752e-06` | `0` | FAIL | max_shape_l1 0.020180511 > 0.016666667 |
| n_idaho_forest_h1 | `dx2p5_dt300` | `dx1p25_dt300` | `2.8096277e-05` | `0.016618657` | `1.0269508e-05` | `8.8538471e-06` | `0.0025207994` | PASS | none |
| wa_cascades_forest_h1 | `dx2p5_dt300` | `dx1p25_dt300` | `2.0838e-05` | `0.0091975171` | `1.6693879e-05` | `6.1704411e-06` | `0.0063686912` | PASS | none |

### fine_reference_adequacy_dt75

| Member | Candidate | Reference | Outlet rel | Shape L1 | End storage rel | Tail fold rel | Annual sed rel | Verdict | Failures |
|---|---|---|---:|---:|---:|---:|---:|---|---|
| mn_corn_h4 | `dx2p5_dt75` | `dx1p25_dt75` | `1.4706828e-05` | `0.011618305` | `8.0034482e-06` | `4.3704066e-07` | `0` | PASS | none |
| n_idaho_forest_h1 | `dx2p5_dt75` | `dx1p25_dt75` | `1.917872e-05` | `0.015960538` | `2.6928708e-06` | `5.7577772e-06` | `0.0015162266` | PASS | none |
| wa_cascades_forest_h1 | `dx2p5_dt75` | `dx1p25_dt75` | `1.2154324e-05` | `0.0072314889` | `5.5733867e-06` | `1.3395141e-05` | `0.022131684` | FAIL | annual_sediment_max_rel 0.022131684 > 0.0066666667 |

### candidate_vs_reference_dt300

| Member | Candidate | Reference | Outlet rel | Shape L1 | End storage rel | Tail fold rel | Annual sed rel | Verdict | Failures |
|---|---|---|---:|---:|---:|---:|---:|---|---|
| mn_corn_h4 | `dx5_dt300` | `dx2p5_dt300` | `4.1909062e-05` | `0.023359856` | `3.0991848e-05` | `2.0997347e-05` | `0` | PASS | none |
| n_idaho_forest_h1 | `dx5_dt300` | `dx2p5_dt300` | `4.3747306e-05` | `0.0103118` | `1.5927778e-05` | `1.6089107e-05` | `0.0026600994` | PASS | none |
| wa_cascades_forest_h1 | `dx5_dt300` | `dx2p5_dt300` | `4.2705481e-05` | `0.010005206` | `4.1180296e-05` | `1.9670168e-05` | `0.018435997` | PASS | none |

### candidate_vs_reference_dt75

| Member | Candidate | Reference | Outlet rel | Shape L1 | End storage rel | Tail fold rel | Annual sed rel | Verdict | Failures |
|---|---|---|---:|---:|---:|---:|---:|---|---|
| mn_corn_h4 | `dx5_dt75` | `dx2p5_dt75` | `2.1205039e-05` | `0.014225464` | `2.8475651e-06` | `1.2831169e-06` | `0` | PASS | none |
| n_idaho_forest_h1 | `dx5_dt75` | `dx2p5_dt75` | `1.4386673e-05` | `0.011616493` | `2.7388736e-07` | `4.1058916e-06` | `0.0024683212` | PASS | none |
| wa_cascades_forest_h1 | `dx5_dt75` | `dx2p5_dt75` | `1.220481e-05` | `0.0053652166` | `4.3662791e-06` | `2.0054842e-05` | `0.018116206` | PASS | none |

### timestep_control_dx1p25

| Member | Candidate | Reference | Outlet rel | Shape L1 | End storage rel | Tail fold rel | Annual sed rel | Verdict | Failures |
|---|---|---|---:|---:|---:|---:|---:|---|---|
| mn_corn_h4 | `dx1p25_dt300` | `dx1p25_dt75` | `2.2146656e-05` | `0.034355056` | `1.4024492e-05` | `7.5014786e-06` | `0` | PASS | none |
| n_idaho_forest_h1 | `dx1p25_dt300` | `dx1p25_dt75` | `2.0262236e-05` | `0.013563382` | `2.556123e-06` | `3.003285e-06` | `0.0027089781` | PASS | none |
| wa_cascades_forest_h1 | `dx1p25_dt300` | `dx1p25_dt75` | `9.0628258e-06` | `0.010988795` | `2.9828294e-06` | `2.9149285e-06` | `0.0076014146` | PASS | none |

### timestep_control_dx2p5

| Member | Candidate | Reference | Outlet rel | Shape L1 | End storage rel | Tail fold rel | Annual sed rel | Verdict | Failures |
|---|---|---|---:|---:|---:|---:|---:|---|---|
| mn_corn_h4 | `dx2p5_dt300` | `dx2p5_dt75` | `6.2110516e-05` | `0.024695394` | `6.1578653e-05` | `1.2782373e-06` | `0` | PASS | none |
| n_idaho_forest_h1 | `dx2p5_dt300` | `dx2p5_dt75` | `2.5491814e-05` | `0.016602126` | `1.013276e-05` | `9.278485e-08` | `0.00018010597` | PASS | none |
| wa_cascades_forest_h1 | `dx2p5_dt300` | `dx2p5_dt75` | `1.6154047e-05` | `0.01309864` | `1.4103321e-05` | `1.0139629e-05` | `0.018582993` | PASS | none |

### timestep_control_dx5

| Member | Candidate | Reference | Outlet rel | Shape L1 | End storage rel | Tail fold rel | Annual sed rel | Verdict | Failures |
|---|---|---|---:|---:|---:|---:|---:|---|---|
| mn_corn_h4 | `dx5_dt300` | `dx5_dt75` | `9.6213762e-05` | `0.031526393` | `9.5418066e-05` | `2.0992467e-05` | `0` | PASS | none |
| n_idaho_forest_h1 | `dx5_dt300` | `dx5_dt75` | `3.4060819e-05` | `0.01476056` | `2.5786651e-05` | `2.0287784e-05` | `2.064502e-05` | PASS | none |
| wa_cascades_forest_h1 | `dx5_dt300` | `dx5_dt75` | `5.1219035e-05` | `0.011308417` | `5.0917338e-05` | `1.0524303e-05` | `0.0087233127` | PASS | none |

## Report-Only Controls

None. All same-`dx` timestep controls are gate-class comparisons in this package.

## Blockers

- mn_corn_h4 fine_reference_adequacy_dt300: max_shape_l1 0.020180511 > 0.016666667
- wa_cascades_forest_h1 fine_reference_adequacy_dt75: annual_sediment_max_rel 0.022131684 > 0.0066666667

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/mesh-policy-ratification.json`
