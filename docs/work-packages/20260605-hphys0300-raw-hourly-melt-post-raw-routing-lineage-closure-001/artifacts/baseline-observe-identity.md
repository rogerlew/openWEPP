# HPHYS0300 Baseline Observe Identity Reused From HPHYS0299

Ran:

- Run root: `/tmp/hphys0300_full_20260605T155527Z`
- Release binary: `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
- Observe binary: `/tmp/hphys0298_wepp_forest_obs/src/wepp_hill`
- Baseline commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Lanes: pinned release without observe, instrumented observe-off, instrumented observe-on.

| Hill | Pass | Release=Off | Off=On | Partition Identity | Records | Release SHA | Off SHA | On SHA |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | True | True | True | True | 48102 | 25bbd9452e71 | 25bbd9452e71 | 25bbd9452e71 |
| H7 | True | True | True | True | 48387 | 311034a64858 | 311034a64858 | 311034a64858 |
| H39 | True | True | True | True | 48102 | 162461c5693d | 162461c5693d | 162461c5693d |
