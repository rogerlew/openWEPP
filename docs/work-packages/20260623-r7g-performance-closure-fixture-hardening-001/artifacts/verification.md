# Verification

Evidence class: Static plus Ran.

Status: executed-held.

## Focused Gates

Ran:

```text
cargo test -p openwepp-runner r7 -- --nocapture
```

Result: pass, `12 passed`.

Covered R7G-relevant focused checks:

- R7E default candidate and rollback selection policy.
- R7C direct production source excludes compatibility entrypoints.
- R7F production direct uses `DirectProductionDayInputBuilder`.
- R7F typed day-input hot-loop excludes runtime-surface reads.
- R7C direct production fixture reports no day-input compatibility edges.

## Full Gates

Ran current same-binary H2637 modes:

- default-disabled compatibility: pass, `645.51 s / 229560 KiB`;
- rollback compatibility: pass, `637.10 s / 229016 KiB`;
- direct default candidate: fail closed, `0.94 s / 729204 KiB`;
- explicit direct production: fail closed, `0.92 s / 729200 KiB`.

Direct failure marker:

```text
R7F typed production day-input path does not yet have surface-free active snow
partition authority for lane 1
```

Full direct performance, full direct output identity, and direct H2637
manifest counter gates are blocked behind
`HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`.

## Documentation Gates

Ran:

```text
markdown-doc lint \
  --path docs/work-packages/20260623-r7g-performance-closure-fixture-hardening-001 \
  --path docs/work-packages/README.md \
  --path docs/architecture/array-native-runtime-specification.md \
  --path docs/ROADMAP.md \
  --format json
```

Result: pass, `17` files scanned, `0` errors, `0` warnings.

Ran:

```text
git diff --check
```

Result: pass.

## Gate Legitimacy Audit

- `<=10x` direct default: `BLOCKED`, not run to endpoint.
- Protected output identity: `PASS` for compatibility rollback; `BLOCKED` for
  direct full H2637 because no direct output was produced.
- No-compatibility hot-loop proof: focused source/counter tests `PASS`;
  full-H2637 direct manifest `BLOCKED`.
- Profile evidence: `BLOCKED/NOT APPLICABLE` because direct failed before hot
  loop. Profiling failure handling would not satisfy the package gate.
- Fixture matrix: documented with pass/fail/residual risk.
- Independent reconstruction: compatibility rollback checksum reconstruction
  `PASS`; direct full-H2637 reconstruction `BLOCKED`.
- Terminal state: executed hold is legitimate because the first direct blocker
  is absence of typed active snow partition state/authority, and using the
  existing compatibility-surface snow helper would violate the R7F boundary.
