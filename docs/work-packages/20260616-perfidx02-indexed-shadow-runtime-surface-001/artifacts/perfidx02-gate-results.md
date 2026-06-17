# PERFIDX02 Gate Results

Status: PASS 2026-06-16
Evidence mode: **Ran**

## Build

```text
RUSTFLAGS='-C force-frame-pointers=yes -C debuginfo=1' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
Finished release profile [optimized] target(s) in 57.50s
```

## Required Gates

```text
cargo fmt --all -- --check
PASS

git diff --check
PASS

cargo clippy --workspace --all-targets -- -D warnings
PASS

cargo test --workspace
PASS

cargo deny check
advisories ok, bans ok, licenses ok, sources ok
```

## PERFIDX02-Specific Runtime Gates

```text
PERFIDX02_H2637_SHADOW_FIXED elapsed_s=892.67 user_s=892.09 sys_s=0.42 maxrss_kb=237224
PERFIDX02_UI_SHADOW case=h2637_with_ui elapsed_s=908.83 user_s=908.12 sys_s=0.54 maxrss_kb=236356
PERFIDX02_UI_AUDIT case=h2637_with_ui elapsed_s=1515.61 user_s=1514.73 sys_s=0.47 maxrss_kb=236468
```

Ladder audit+shadow:

```text
PERFIDX02_LADDER case=ofe1 elapsed_s=10.88 user_s=10.85 sys_s=0.01 maxrss_kb=21884
PERFIDX02_LADDER case=ofe2 elapsed_s=22.29 user_s=22.26 sys_s=0.02 maxrss_kb=23468
PERFIDX02_LADDER case=ofe3 elapsed_s=31.12 user_s=31.09 sys_s=0.02 maxrss_kb=25460
PERFIDX02_LADDER case=ofe4 elapsed_s=50.43 user_s=50.36 sys_s=0.05 maxrss_kb=26940
PERFIDX02_LADDER case=ofe5 elapsed_s=49.57 user_s=49.52 sys_s=0.03 maxrss_kb=26476
```

Determinism:

```text
PERFIDX02_DETERMINISM case=ofe5 run=1 elapsed_s=27.54 user_s=27.50 sys_s=0.03 maxrss_kb=27940
PERFIDX02_DETERMINISM case=ofe5 run=2 elapsed_s=27.59 user_s=27.54 sys_s=0.04 maxrss_kb=27812
```

## Disposition

All required gates passed. No storage authority flip occurred.
