# PySnobal Run Evidence

Status: executed-hold

Evidence mode: Static + Ran.

Static:

- PySnobal repository path: `/workdir/pysnobal` resolved by the harness to
  `/home/workdir/pysnobal`.
- Python executable used: `/tmp/pysnobal-g0-venv/bin/python`.
- Harness output summary:
  `docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/artifacts/pysnobal_site_summary.json`.

Ran:

```text
python /tmp/pysnobal-g0-venv/bin/python
numpy 1.26.4
pandas 2.2.0
pysnobal /workdir/pysnobal/pysnobal/__init__.py
run_snobal <function run_snobal ...>
```

Ran: one-site Site 1 harness completed with route
`PROCEED-SNOWFROST-FIDELITY-G` for the Phase 3 one-site scope only. The
all-site route below supersedes it for package disposition.

Ran: all-site harness returned `PYSNOBAL_HARNESS_EXIT=1` and wrote route
`HOLD-PYSNOBAL-SANITY-FAILURE`. The all-site status was:

| Site | Lane status |
| --- | --- |
| Site 1 Sleepers South | 3 PASS |
| Site 2 Sleepers W9 | 3 PASS |
| Site 3 SCAN Mandan | 3 PASS |
| Site 4 GGD498 Morris | `Tg=0.0` PASS, `Tg=-2.5` PASS, `Tg=-0.5` FAIL |
| Site 5 Reynolds Creek | 3 PASS |

Ran: the failed Site 4 lane wrote:

```text
[pysnobal/c_snobal/libsnobal/sati.c:17] ERROR: Input temperature (tk): -153.450833 is less than zero
```

Ran: the Site 4 `Tg=-0.5` exported forcing is finite at the file boundary, and
the adjacent `Tg=0.0` and `Tg=-2.5` lanes pass under the same meteorological
forcing. The blocker is therefore recorded as a PySnobal lane-specific
sanity/numerical failure, not as a Rust schema failure.
