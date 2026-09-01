# V16 covered support-receipt retained pre-red

Evidence mode: `Ran` plus `Static`

Command:

```text
RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test erosion_single_ofe_p61_sediment
```

Retained result: `FAIL`, one test, `193.22 s`. The run crossed the former exact
rounded-high mirror refusal and stopped at
`176400000000000..178200000000000 ns` with `VEG-E-123: invalid or mismatched
LSE support receipt`.

A temporary environment-gated audit, removed after capture, independently
parsed the V11 staged complete-owner bytes. Through the immediately preceding
`174600000000000..176400000000000 ns` support, the receipt LSE digest equaled
the staged native V3 top-level state digest and the receipt soil digest equaled
the staged native V2 soil-state digest. The failing support emitted no
snow-free-selector audit, locating the rejection on the covered path.

Static inspection found that snow-free real-consumer execution selects the
receipt beginning from either the exact staged legacy bytes or the exact
staged native V3 bytes and rejects any third state. Covered owner finalization
still always used `beginning.inner.lse_state`, so its receipt cannot join a
staged native V3/V4 LSE owner at the covered transition. The authorized fix is
selection parity only. It must not change physics, tolerances, the exact
60-second floor, mass/energy closure, custody, rollback, or publication.

## Post-correction disposition

The unchanged 64 MiB-stack command was rerun after the staged-byte selection
parity correction. It advanced past `176400000000000..178200000000000 ns` and
failed after `394.137 s` at the next
`178200000000000..178260000000000 ns` support, the exact 60-second floor. The
new limiter is
`SC-SNOWENERGY-E-FIXED-POINT-001: bounded covered fixed-point iteration did not
converge` in imported V10 covered execution. Thus the support-receipt mismatch
is cleared; p61 remains red on a distinct fixed-point convergence failure.
