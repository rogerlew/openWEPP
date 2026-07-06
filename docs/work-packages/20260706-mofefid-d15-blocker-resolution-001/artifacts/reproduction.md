# Reproduction

Status: **EXECUTED-BASELINE-FAIL**.

Evidence mode: Ran.

## Baseline Failure

Ran:

```sh
cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture
```

Result: **FAIL** before correction, `0` passed / `1` failed / `1` skipped,
`128.212 s`.

Failure:

```text
native-routed H2637 must run with Lane D shadow enabled:
RuntimeSurfaceFailure { surface: "r7c_direct_production_executor",
detail: "HS-SIMPIPE-E-001 direct publication sink failed:
laned shadow cascade: NegativeOutletBin" }
```

Static diagnosis:

- The D15 rerun's temporary diagnostic context already identified the failing
  day as day `88`, with source active through hour `24`.
- `laned_shadow.rs` intended an active-source-span plus `6 h` drain tail, but
  then capped the route window at one day. For hour-24 source, that cap removed
  the entire drain tail.
- `seam_rate_at` returns zero outside the 24-hour source window, so allowing a
  routing tail after hour 24 drains already-routed water without adding or
  smearing source depth.
