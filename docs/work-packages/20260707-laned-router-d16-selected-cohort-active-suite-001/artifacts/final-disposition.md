# Final Disposition

Status: EXECUTED-HOLD-ACTIVE-RUN. Evidence mode: Static + Ran.

The selected-cohort evidence hold is not lifted.

This package did materialize the intended four-member cohort and produced valid
corrected H2637 active plain vs true hybrid evidence:

- Active plain: `39.64 s` user, `0:39.71` wall.
- Active hybrid: `33.33 s` user, `0:33.37` wall.
- H2637 outlet delta: `-0.439570%`.
- H2637 pass sediment concentration sums: about `-6.47424%`.

The package stops at `mn_corn_h4` active plain:

```text
lane 1 day 136 has LAI 0.01182723510043506 > 0 with missing/non-positive typed-management canhgt (rev-21 fail-closed)
```

No selector flip, durable owcmp suite posture change, science-contract change,
or Rust runtime change landed.

Next action:

- Scaffold and execute `D16-ROWCROP-CANHGT-ACTIVE-RUNTIME-PUBLICATION-001`,
  then rerun this selected cohort suite and re-adjudicate D16/default
  promotion.
