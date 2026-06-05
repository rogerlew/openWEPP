# Baseline Instrumentation Patch

Status: complete

Evidence mode: static

Static:

- Diagnostic instrumentation was applied only to detached worktree `/tmp/hphys0298_wepp_forest_obs`.
- Pinned source worktree `/workdir/wepp-forest_260430_baseline` was not edited.
- Exact patch is stored in `artifacts/baseline-observe-instrumentation.patch`.
- Patched files in the detached worktree:
  - `src/winter.for`
  - `src/contin.for`
  - `src/watbal.for`
  - `src/watbal_hourly.for`

Ran:

```text
make clean && make COMPILER=gfortran wepp_hill
```

Result: diagnostic `wepp_hill` built successfully in `/tmp/hphys0298_wepp_forest_obs/src`.
