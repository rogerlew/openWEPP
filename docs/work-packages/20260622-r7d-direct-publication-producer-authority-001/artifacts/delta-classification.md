# Delta Classification

Status: executed-held.

## Focused Fixture

Ran:

- Prepared `/tmp/r7d-focus/default` and `/tmp/r7d-focus/direct` from the
  focused hillslope fixture and added `pass_parquet = "output/H5.pass.parquet"`
  to each temporary run file.
- Ran default compatibility:
  `target/release/openwepp-cli-hill --run-dir /tmp/r7d-focus/default --run-file case.run --output-dir /tmp/r7d-focus/default/output --policy compat`.
- Ran production direct:
  `target/release/openwepp-cli-hill --run-dir /tmp/r7d-focus/direct --run-file case.run --output-dir /tmp/r7d-focus/direct/output --policy compat --direct-production-executor`.
- Ran `sha256sum` over HBP, loss JSON, PASS Parquet, and WAT Parquet.

Result:

| Output | Default checksum | Direct checksum | Classification |
| --- | --- | --- | --- |
| HBP | `cbe53a3ee5ac216782fc5db87dacb1dc40ff50f51dc2d5d0cf24171da4371760` | `cbe53a3ee5ac216782fc5db87dacb1dc40ff50f51dc2d5d0cf24171da4371760` | PASS |
| loss JSON | `36efc6c896e79890c89d1593a95ae20c5d7e84f20e9cb487fe3503eb70676d5e` | `36efc6c896e79890c89d1593a95ae20c5d7e84f20e9cb487fe3503eb70676d5e` | PASS |
| PASS Parquet | `1a62c2b09aa3507a90536283ee10bf70c9b88caf4c8647c304d67e8a5bff73d3` | `1a62c2b09aa3507a90536283ee10bf70c9b88caf4c8647c304d67e8a5bff73d3` | PASS |
| WAT Parquet | `2216771cc933074b071d75879822c150fabb04f814a895c98b2b68b5c25b051e` | `2216771cc933074b071d75879822c150fabb04f814a895c98b2b68b5c25b051e` | PASS |

Focused fixture conclusion: the one-OFE case does not expose the R7D blocker.
It is necessary but insufficient evidence because topology-only direct lane
frames can alias aggregate state when there is only one lane.

## H2637

Ran:

- Created separated run files under `/tmp/r7d-h2637-baseline/runfiles/` by
  rewriting the original H2637 run-file output roots to
  `/tmp/r7d-h2637-baseline/default/output` and
  `/tmp/r7d-h2637-baseline/direct/output`.
- Ran default compatibility:
  `env -u OPENWEPP_DIRECT_RUNTIME_AUDIT -u OPENWEPP_INDEXED_SHADOW_AUDIT /usr/bin/time -f 'r7d_h2637_default_baseline\t%e\t%M' target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/r7d-h2637-baseline/runfiles/h2637_default.run --output-dir /tmp/r7d-h2637-baseline/default/output --policy compat --legacy-sidecar-discovery`.
- Ran production direct:
  `env -u OPENWEPP_DIRECT_RUNTIME_AUDIT -u OPENWEPP_INDEXED_SHADOW_AUDIT /usr/bin/time -f 'r7d_h2637_direct_baseline\t%e\t%M' target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/r7d-h2637-baseline/runfiles/h2637_direct.run --output-dir /tmp/r7d-h2637-baseline/direct/output --policy compat --legacy-sidecar-discovery --direct-production-executor`.
- Ran `sha256sum` over HBP, loss JSON, PASS Parquet, plot Parquet, and WAT
  Parquet.
- Ran DuckDB bidirectional `except all` comparisons over WAT and PASS.

Timing and memory:

| Mode | Time | Max RSS |
| --- | ---: | ---: |
| Default compatibility | `637.78 s` | `229144 KiB` |
| Production direct | `729.54 s` | `626280 KiB` |

Checksum result:

| Output | Default checksum | Direct checksum | Classification |
| --- | --- | --- | --- |
| HBP | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` | `20037fdacd21c15abbfe0ffdaf7b75f98f053d045ef42d9f4bb66673c18f6366` | FAIL |
| loss JSON | `9bdbabe532bfbc2f49d4a4ae5db24c6069e93384f306e71759c223a795a5be38` | `9bdbabe532bfbc2f49d4a4ae5db24c6069e93384f306e71759c223a795a5be38` | PASS |
| PASS Parquet | `9bc37769ec7a544641b903f038f59768c672e0f0b026333921723ebc9ae95a46` | `22fffc11f42fac16f62201e612513842e161e1ab978d22c10469265e8cdf6370` | FAIL |
| plot Parquet | `4cdb19fecd36a3f074d5c900bc687eff7ce58f80a31c9cb7e5e0f5615ac5a783` | `4cdb19fecd36a3f074d5c900bc687eff7ce58f80a31c9cb7e5e0f5615ac5a783` | PASS |
| WAT Parquet | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` | `de1d6eb557026f2fd5e0f6c7ca2554ef4d64144a44a2518be3fe7287ee687f84` | FAIL |

DuckDB row-difference result:

| Dataset | Default minus direct | Direct minus default |
| --- | ---: | ---: |
| WAT | `235961` | `235961` |
| PASS | `12419` | `12419` |

Joined WAT field-difference counts by `(sim_day_index, OFE)`:

| Field | Diff count |
| --- | ---: |
| P | `17233` |
| RM | `86125` |
| Q | `227860` |
| Ep | `235558` |
| Es | `235637` |
| Dp | `231934` |
| UpStrmQ | `209194` |
| SubRIn | `192445` |
| Total-Soil | `235961` |
| SoilWaterTotal | `235961` |
| Interception | `111895` |

Joined PASS field-difference counts by `sim_day_index`:

| Field | Diff count |
| --- | ---: |
| runvol | `12372` |
| sbrunv | `12419` |
| peakro | `0` |
| tdet | `0` |

Representative first-day WAT evidence:

| Day | OFE | Default Q | Direct Q | Default Total-Soil | Direct Total-Soil | Default Ep | Direct Ep | Default Es | Direct Es |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1 | `6.938893903907228e-15` | `45.2` | `527.6909996102706` | `474.84756426156133` | `0.9042922551571403` | `0.9042922551571403` | `0.004341685638884398` | `0.004341685638884398` |
| 1 | 2 | `0.34333870917230974` | `45.2` | `528.502953959294` | `474.84756426156133` | `0.47397552223669265` | `0.9042922551571403` | `0.372699827240713` | `0.004341685638884398` |

Representative first-day PASS evidence:

| Day | Default runvol | Direct runvol | Default sbrunv | Direct sbrunv |
| ---: | ---: | ---: | ---: | ---: |
| 1 | `107.13682236123434` | `491.3056036000001` | `3.3141809548229606` | `1.357506344804005` |
| 2 | `171.97242510995278` | `68.47843590000001` | `5.328571807211335` | `0.6650395127874607` |

Manifest evidence:

- Default compatibility:
  `execution_provenance.scheduler_kernel_executed=true`,
  `execution_provenance.publication_source=scheduler-kernel`,
  `wb13_publication.source=simulation-owned`, `wb13_publication.row_count=235961`.
- Production direct:
  `execution_provenance.scheduler_kernel_executed=false`,
  `execution_provenance.publication_source=direct-publication-frame`,
  `wb13_publication.source=direct-publication-frame`,
  `wb13_publication.row_count=235961`,
  `direct_runtime_counters.compatibility_edge_invocations=0`.

Classification: H2637 is a multi-OFE producer-state blocker. Direct production
publishes direct-source rows with the correct row count and zero compatibility
edge invocations, but its typed lane frames are seeded from aggregate/default
state rather than lane-indexed per-OFE parsed authority. This causes repeated
or seed-like hydrology across lanes and full-run WAT/PASS divergence.
