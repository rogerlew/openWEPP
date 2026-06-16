# PERFHO02 Gate Results

Status: PASS 2026-06-16
Evidence mode: **Ran**

## Scope Gate

No production Rust files were edited. No science contracts were edited.

## Build / Profiler Gates

```text
RUSTFLAGS='-C force-frame-pointers=yes -C debuginfo=1' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: PASS.

Initial `perf` probe:

```text
perf stat -e task-clock,cycles,instructions -- target/release/openwepp-cli-hill --help
```

Result: initially BLOCKED by `perf_event_paranoid=4`; GDB fallback used for first closure.

After the host sysctl update:

```text
cat /proc/sys/kernel/perf_event_paranoid
```

Result:

```text
0
```

```text
perf stat -e task-clock,cycles,instructions -- target/release/openwepp-cli-hill --help
```

Result: PASS.

```text
perf record -F 99 --call-graph fp -o /tmp/perfho02/perf-h2637-post-perfopt.data -- timeout --signal=INT 90s target/release/openwepp-cli-hill ...
```

Result: PASS for bounded profiling sample; timeout exit `124` expected; `9,586` samples captured.

```text
perf stat -e task-clock,cycles,instructions,branches,branch-misses,cache-references,cache-misses -- timeout --signal=INT 30s target/release/openwepp-cli-hill ...
```

Result: PASS for bounded hardware-counter sample.

```text
gdb -q target/release/openwepp-cli-hill
```

Result: PASS for fallback sampling. Raw log: `/tmp/perfho02/gdb-h2637-post-perfopt.txt`.

## Documentation Gate

```text
git diff --check
```

Result: PASS.

## Rust Closure Gates

Not run for PERFHO02 because the package is characterization-only and did not edit production Rust. PERFOPT01 already ran the full Rust closure loop for the current optimized code path. PERFHO02's current-scope acceptance is profiler-backed attribution and documentation closure.
