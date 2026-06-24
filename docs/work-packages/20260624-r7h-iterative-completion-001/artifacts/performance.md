# Performance Evidence

Evidence class: Ran.

Fixture:

- Run dir: `/tmp/openwepp_farpoint01_h2637/without_ui/runs`
- Run file: `h2637.run`
- OFE-days: `235961`
- R7H budget: `<=91.2 s`

Current-code matrix:

| Mode | CLI flag | Exit | Seconds | RSS KiB | Manifest/counters | Result |
| --- | --- | ---: | ---: | ---: | --- | --- |
| default compatibility | none | `0` | `590.23` | `227924` | `selected=compatibility`; `publication_source=scheduler-kernel` | pass rollback reference |
| explicit rollback compatibility | `--compatibility-runtime` | `0` | `600.20` | `229164` | `selected=compatibility`; `publication_source=scheduler-kernel` | pass rollback reference |
| direct default-candidate, pre-fix | `--direct-default-candidate` | `0` | `112.99` | `1083024` | `compatibility_edge_invocations=0`; direct publication frame | fail timing |
| direct default-candidate, perf sample pre-fix | `--direct-default-candidate` under `perf record -F 99 -g` | `0` | `115.52` | `1083020` | `12606` samples captured | fail timing/profile |
| direct default-candidate, after fine-layer guard allocation fix | `--direct-default-candidate` | `0` | `61.40` | `1082876` | `compatibility_edge_invocations=0`; `day_frame_commits=235961`; direct publication frame | pass timing |
| explicit direct production, after fine-layer guard allocation fix | `--direct-production-executor` | `0` | `64.19` | `1083260` | `compatibility_edge_invocations=0`; direct publication frame | pass timing |
| explicit direct production, day-6 trace attempt | `--direct-production-executor` with `OPENWEPP_R7G_FROST_TRACE_*` | `0` | `61.56` | `1083640` | no trace file; typed branch has no R7G trace hook | diagnostic only |
| explicit direct production, day-only trace attempt | `--direct-production-executor` with `OPENWEPP_R7G_FROST_TRACE_DAY=6` | `0` | `64.75` | `1083636` | no trace file; typed branch has no R7G trace hook | diagnostic only |

Iterations:

- Release binary after fix:
  `ab94626bcd202c5254af865a99bc6f4f626057f425911259c50fb1929375aacb`.
- Direct default and explicit direct protected-output checksums match exactly
  after the performance fix.
- Default compatibility and explicit rollback protected-output checksums match
  exactly after the performance fix.
- Direct timing is green against the R7H `<=91.2 s` budget. R7H remains blocked
  by protected-output parity, not endpoint performance.
