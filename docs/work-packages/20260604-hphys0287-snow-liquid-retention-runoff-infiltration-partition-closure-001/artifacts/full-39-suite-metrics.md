# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: Ran

# HPHYS0287 Full H1..H39 Semantic Summary

Ran:

- Root: `/tmp/hphys0287_full_release_after_review_20260604T221027Z`
- Runtime status: `/tmp/hphys0287_full_release_after_review_20260604T221027Z/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0287_full_release_after_review_20260604T221027Z/reports/semantic_status.tsv`
- Semantic pass: `0/39`

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | --- | --- | --- |
| Ep | 0/39 | 46375 | 0.742663 | 7.328460 |
| Total-Soil | 0/39 | 54884 | 61.115200 | 365.018266 |
| SoilWaterTotal | 0/39 | 54884 | 61.115200 | 365.018266 |
| Dp | 1/39 | 9104 | 0.042128 | 0.244800 |
| latqcc | 0/39 | 36576 | 0.410749 | 12.327253 |
| Q | 0/39 | 2108 | 0.552218 | 38.472185 |
| RM | 0/39 | 6633 | 0.248018 | 27.960000 |
| Snow-Water | 0/39 | 10391 | 2.899431 | 65.506840 |


## Selected Metrics JSON

```json
{
  "Dp": {
    "column": "Dp",
    "hillslope_fail_count": 38,
    "total_fail_count": 9104,
    "mean_abs_diff_mean": 0.04212801743734681,
    "max_abs_diff": 0.24479985054954173
  },
  "Ep": {
    "column": "Ep",
    "hillslope_fail_count": 39,
    "total_fail_count": 46375,
    "mean_abs_diff_mean": 0.7426628844573041,
    "max_abs_diff": 7.32846045638271
  },
  "Er": {
    "column": "Er",
    "hillslope_fail_count": 0,
    "total_fail_count": 0,
    "mean_abs_diff_mean": 0.0,
    "max_abs_diff": 0.0
  },
  "Es": {
    "column": "Es",
    "hillslope_fail_count": 1,
    "total_fail_count": 518,
    "mean_abs_diff_mean": 0.01047934994695919,
    "max_abs_diff": 1.8255012658093974
  },
  "P": {
    "column": "P",
    "hillslope_fail_count": 0,
    "total_fail_count": 0,
    "mean_abs_diff_mean": 4.567986418054738e-17,
    "max_abs_diff": 3.552713678800501e-15
  },
  "Q": {
    "column": "Q",
    "hillslope_fail_count": 39,
    "total_fail_count": 2108,
    "mean_abs_diff_mean": 0.5522184944574529,
    "max_abs_diff": 38.47218540850354
  },
  "RM": {
    "column": "RM",
    "hillslope_fail_count": 39,
    "total_fail_count": 6633,
    "mean_abs_diff_mean": 0.2480182077450816,
    "max_abs_diff": 27.959999999999997
  },
  "Snow-Water": {
    "column": "Snow-Water",
    "hillslope_fail_count": 39,
    "total_fail_count": 10391,
    "mean_abs_diff_mean": 2.899431327346222,
    "max_abs_diff": 65.50683982565039
  },
  "SoilWaterTotal": {
    "column": "SoilWaterTotal",
    "hillslope_fail_count": 39,
    "total_fail_count": 54884,
    "mean_abs_diff_mean": 61.115200374585385,
    "max_abs_diff": 365.01826604776187
  },
  "Total-Soil": {
    "column": "Total-Soil",
    "hillslope_fail_count": 39,
    "total_fail_count": 54884,
    "mean_abs_diff_mean": 61.115200374585385,
    "max_abs_diff": 365.01826604776187
  },
  "latqcc": {
    "column": "latqcc",
    "hillslope_fail_count": 39,
    "total_fail_count": 36576,
    "mean_abs_diff_mean": 0.4107493317849935,
    "max_abs_diff": 12.327252950598734
  }
}

```

## HPHYS0286 To HPHYS0287 Delta

Static:
- Selected valid-run semantic metrics remain unchanged from HPHYS0286 and the first HPHYS0287 run.
- This is expected because HPHYS0287 fixed fail-open handling for invalid projected runtime snow state, not baseline-authoritative valid-run liquid retention/release magnitude.
