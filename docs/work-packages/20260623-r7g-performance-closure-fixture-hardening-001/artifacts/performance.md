# Performance Evidence

Evidence class: Ran.

Status: executed-held.

## Constants

- H2637 OFE-days: `235961`.
- Legacy WEPP wall time: `9.12 s`.
- Legacy OFE-day: `38.65 us`.
- `<=10x` wall-time budget: `91.2 s`.
- `<=10x` OFE-day budget: `386 us`.

## Release Build

Ran:

```text
/usr/bin/time -f 'r7g_release_build\t%e\t%M' \
  cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result:

```text
r7g_release_build  63.15  1133108
```

Binary hashes:

```text
319fb7e143cff473222336ddb20eea3779697b14e6c983a3c59314db3a3ad239  target/release/openwepp-cli-hill
ef1a3d18f513c4d3b52067e0db33aaf8ebfc3b937168c24645b27cc8f3db2586  target/release/openwepp-cli-hill.json
```

## Same-Binary H2637 Matrix

| Mode | Command flag | Seconds | RSS KiB | us/OFE-day | x legacy | Exit |
|---|---|---:|---:|---:|---:|---:|
| default-disabled compatibility | none | `645.51` | `229560` | `2735.66` | `70.78x` | `0` |
| rollback compatibility | `--compatibility-runtime` | `637.10` | `229016` | `2700.02` | `69.86x` | `0` |
| direct default candidate | `--direct-default-candidate` | `0.94` failure-path only | `729204` | not meaningful | not meaningful | `1` |
| explicit direct production | `--direct-production-executor` | `0.92` failure-path only | `729200` | not meaningful | not meaningful | `1` |

## Disposition

Default-disabled and rollback compatibility completed with the known
`MOFE01-MG-W-001` sidecar warning and identical protected output checksum
maps.

Direct default candidate and explicit direct production both failed closed
before H2637 hot-loop timing with:

```text
CLIHILL-E-011 runtime surface failure for r7c_direct_production_executor:
HS-SIMPIPE-E-001 direct publication day-input builder failed:
CLIHILL-E-011 runtime surface failure for r7c_direct_production_executor:
HS-SIMPIPE-E-001 R7F typed production day-input path does not yet have
surface-free active snow partition authority for lane 1
```

The `0.94 s` and `0.92 s` direct rows are startup/failure-path measurements,
not endpoint performance. R7G cannot evaluate `<=10x` direct default
performance until `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`
is lifted.
