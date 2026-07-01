# Disposition

Status: `UPDATED`

Final disposition: `EXECUTED-COMPLETE-WSHEDPERF01`.

- Reproducible scoped baseline evidence is now present for all required phases.
- Legacy baseline canonical run is recorded as non-equivalent baseline scope (`full-legacy-watershed`).
- openWEPP routed-stage scope and full practical end-to-end scope are both timed with release binaries and non-empty outputs.
- Full end-to-end path now has 3 successful stability repeats and 1 profiling run under isolated `/tmp/wshedperf01_20260701_101739` roots.

- Remaining action: none in this package.

- Subagent note:
  - Initial `comparator_suite_runner` dispatch errored because the selected
    model was at capacity; benchmark runs were executed locally with
    command-level evidence.

- Scope mismatch reminder:
  - Legacy and openWEPP scopes are not equivalent unless a fresh full legacy-equivalent hillslope generation + routing surface for pw0 is introduced. All ratio statements must call out this mismatch explicitly.
