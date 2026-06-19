# Artifacts

Status: executed 2026-06-19. Verdict:
`PROFILED - cut PERFDEEP05 at lane-dense sync removal`.

Required deliverables:

- `perfdeep04-profile-results.md` - profiler commands, raw-output pointers, top
  costs, and mechanism classification.
- `perfdeep04-next-package-recommendation.md` - ranked follow-up package
  boundary and rejected alternatives.
- `perfdeep04-gate-results.md` - profiler/tool gates and markdown gate.
- `perfdeep04_disposition.md` - final package disposition.

Runfiles:

- `runfiles/perfdeep04-h2637.run` - PERFDEEP03 opt-in H2637 profile runfile,
  with PERFDEEP04 output paths.
- `runfiles/perfdeep04-h2637-default.run` - default-disabled comparison runfile,
  available if a comparison profile is needed.

Raw generated files:

- `raw/` - command output and profiler summaries produced during execution.
- full binary `perf.data` files are intentionally stored under
  `/tmp/perfdeep04/profile/`, not in the repository, because they are hundreds
  of megabytes.
