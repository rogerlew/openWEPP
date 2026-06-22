# Timing And RSS Evidence

Status: complete.

## Release Build

Ran:

```text
/usr/bin/time -f 'release_build\t%e\t%M' \
  cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result:

```text
release_build  54.09  1123404
```

Binary hashes:

```text
d41f3b71fc12b0e534186dc6569e973187e6e1b0d83517d0cc7d447858f87946  target/release/openwepp-cli-hill
92f6b848263eb77a11b2198f7e96bdbd696d8de8da0f7b1a22fe4935dcf26b59  target/release/openwepp-cli-hill.json
```

## H2637 Same-Binary Timing/RSS

Environment:

- `OPENWEPP_DIRECT_RUNTIME_AUDIT` unset.
- `OPENWEPP_INDEXED_SHADOW_AUDIT` unset.
- Run directory: `/tmp/perfho01/run-dirs/h2637`.
- Run file: `/tmp/perfmig01-final/runfiles/h2637_same_current.run`.
- Policy: `compat`.
- Legacy sidecar discovery enabled.

Default compatibility command:

```text
/usr/bin/time -f 'r7c_h2637_default_rep1\t%e\t%M' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/r7c-h2637-final/default/rep1/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery
```

Result:

```text
sidecar-warning: MOFE01-MG-W-001 EROD14 Wave-2 qin is seeded from water-transfer provenance only; true sediment-coupled qin/qout and particle-fraction handoff remains MOFE01 M-G follow-on scope.
r7c_h2637_default_rep1  642.77  228804
```

Direct-production command:

```text
/usr/bin/time -f 'r7c_h2637_direct_production_rep1\t%e\t%M' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/r7c-h2637-final/direct-production/rep1/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery \
  --direct-production-executor
```

Result:

```text
r7c_h2637_direct_production_rep1  753.76  625132
```

Disposition:

- R7C proves an opt-in production direct executor path exists and executes
  all OFE-days through direct frames.
- Direct production is slower than default compatibility for this run:
  `+110.99 s` and about `1.17x` default wall time.
- Direct production RSS is much higher than default compatibility:
  `625132 KB` versus `228804 KB`.
- This is not release-ready and is not a default-activation candidate.
  Performance closure remains R7G scope.

## Output Checksum Disposition

Default compatibility output checksums:

```text
HBP   44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8
loss  9bdbabe532bfbc2f49d4a4ae5db24c6069e93384f306e71759c223a795a5be38
PASS  9bc37769ec7a544641b903f038f59768c672e0f0b026333921723ebc9ae95a46
plot  4cdb19fecd36a3f074d5c900bc687eff7ce58f80a31c9cb7e5e0f5615ac5a783
WAT   c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474
```

Direct-production output checksums:

```text
HBP   20037fdacd21c15abbfe0ffdaf7b75f98f053d045ef42d9f4bb66673c18f6366
loss  9bdbabe532bfbc2f49d4a4ae5db24c6069e93384f306e71759c223a795a5be38
PASS  22fffc11f42fac16f62201e612513842e161e1ab978d22c10469265e8cdf6370
plot  4cdb19fecd36a3f074d5c900bc687eff7ce58f80a31c9cb7e5e0f5615ac5a783
WAT   de1d6eb557026f2fd5e0f6c7ca2554ef4d64144a44a2518be3fe7287ee687f84
```

Disposition:

- `loss` and `plot` checksums match between default compatibility and direct
  production.
- HBP, PASS, and WAT checksums differ.
- R7C does not claim byte/Arrow/publication parity. R7D must resolve direct
  publication producer authority and parity before any default-activation or
  release-readiness package can proceed.
