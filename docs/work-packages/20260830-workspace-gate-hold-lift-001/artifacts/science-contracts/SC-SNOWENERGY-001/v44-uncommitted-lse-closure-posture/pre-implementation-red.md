# V44 uncommitted LSE closure-posture pre-implementation red

Status: `EXPECTED RED`

Evidence mode: `Ran + Static`

## Authority

`SC-SNOWENERGY-001@44` prospectively binds `INV-SNOWENERGY-068` and
`OBL-SNOWENERGY-C-036`. Retained r116
(`/tmp/wghl_001d_v43_64m_r116.log`, SHA-256
`b4046720e3719f4736408833bfa1fc32a23e9bfc95aca8219c760bef9c59aadb`)
proved that strict weighted-OFE closure executes before a private nonlinear
coordinate can rebuild reciprocal-longwave/shortwave/sensible/vapor exchange.
The first 60-second forest-tile residual was
`423.500682899798 J m^-2 tile`; later trials changed sign and magnitude, so no
fixed ledger term or tolerance amendment is authorized.

## Expected-red command

Ran:

```text
nix develop -c cargo test --test snow_terminal_enthalpy_event_numerics_contract v44_ -- --nocapture
```

Result: `1 passed; 1 failed; 30 filtered out`.

- Contract authority test: `PASS`.
- Production/source obligation: expected `FAIL` because the sealed evaluation
  kind did not yet select uncommitted-private versus strict-authentic closure
  posture and the five V44 behavior vectors did not yet exist.

The red names the missing posture selector, strict weighted-OFE replay guard,
corrected-exchange one-map evidence, exact-once CN soil-coordinate evidence,
and no-authentic-admission/no-publication poison. Production code was unchanged
when this red was recorded.
