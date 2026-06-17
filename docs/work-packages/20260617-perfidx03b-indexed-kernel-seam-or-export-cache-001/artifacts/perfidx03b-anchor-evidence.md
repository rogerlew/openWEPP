# PERFIDX03B Anchor Evidence

Ran: full current anchor executions on 2026-06-17.

## H2637

Temporary runfiles:

- `/tmp/perfidx03b/runfiles/h2637_current.run`
- `/tmp/perfidx03b/runfiles/h2637_with_ui_current.run`

Output roots:

- `/tmp/perfidx03b/anchor/current/h2637`
- `/tmp/perfidx03b/anchor/current/h2637_with_ui`

Results:

| Case | Result | Elapsed s | User s | Sys s | Max RSS KB |
| --- | --- | ---: | ---: | ---: | ---: |
| H2637 without UI | PASS | 874.73 | 874.19 | 0.38 | 228504 |
| H2637 with UI | PASS | 873.62 | 873.06 | 0.41 | 229316 |

Both manifest files exist:

- `/tmp/perfidx03b/anchor/current/h2637_manifest/manifest.json`
- `/tmp/perfidx03b/anchor/current/h2637_with_ui_manifest/manifest.json`

## OFE1-OFE5 Ladder

Temporary runfiles:

- `/tmp/perfidx03b/runfiles/ofe1_current.run`
- `/tmp/perfidx03b/runfiles/ofe2_current.run`
- `/tmp/perfidx03b/runfiles/ofe3_current.run`
- `/tmp/perfidx03b/runfiles/ofe4_current.run`
- `/tmp/perfidx03b/runfiles/ofe5_current.run`

Results:

| Case | Result | Elapsed s | User s | Sys s | Max RSS KB |
| --- | --- | ---: | ---: | ---: | ---: |
| OFE1 | PASS | 5.15 | 5.12 | 0.01 | 21340 |
| OFE2 | PASS | 10.77 | 10.75 | 0.01 | 21416 |
| OFE3 | PASS | 15.79 | 15.76 | 0.02 | 22936 |
| OFE4 | PASS | 25.72 | 25.60 | 0.11 | 24088 |
| OFE5 | PASS | 25.26 | 25.22 | 0.03 | 24404 |

Each ladder case wrote required output files and a manifest under
`/tmp/perfidx03b/anchor/current/`.

## Notes

The first H2637 attempt produced outputs but failed manifest writing because the
manifest directory was absent. The clean rerun with pre-created manifest
directories passed and is the accepted evidence.

