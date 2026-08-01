# Independent Authority Verification A

Evidence mode: `Static`.

Verifier: independent authority reviewer A

Scope: closure of the accepted Phase A authority findings in
`SC-SNOWENERGY-001` version 6, reconciliation with
`authority-review-disposition.md`, and contract identity confirmation against
`authority-seal.json`. No EB-04R path or result-bearing evidence was read.

Disposition: `PASS`.

## Identity Verification

The amended contract declares `contract_version: 6`. Its independently
calculated SHA-256 is:

```text
364a2bad34235c105cc4b47be50e12ca34e0b9e27b2aa2fd0c6842681670ab72
```

This exactly matches `canonical_contract_sha256` in `authority-seal.json`.
The seal identifies version `6` and remains
`PENDING_DUAL_VERIFICATION`, with `phase_b_authorized: false`, as required
until both independent verifiers pass.

## Finding Closure

### `A-M1` — PASS

The accepted medium finding is fully incorporated:

- `INV-SNOWENERGY-028` binds `1e-9 m` SWE to its exact same-residual
  `1e-6 kg m^-2` area-mass expression through
  `rho_w = 1000 kg m^-3`;
- the same invariant scopes `1e-6 kg m^-2` to vapor-to-sublimation transfer
  closure and explicitly preserves hourly/daily vapor aggregation at
  `1e-9 kg m^-2`;
- `INV-SNOWENERGY-027` and `INV-SNOWENERGY-028` preserve the represented-layer
  lifecycle boundary at `1e-9 kg m^-2`, equivalent to `1e-12 m` SWE, as a
  representation predicate rather than a residual tolerance; and
- the guard-map row requires operand-specific reconstruction and rejects
  cross-predicate substitution.

The constants table and Tolerance and Numeric Notes repeat the three distinct
predicates with explicit units and prohibit generalizing the transfer
tolerance to other mass checks. This resolves the finding without changing
runtime physics or empirical rules.

### `B-M1` — PASS

The version-6 text narrowly binds `1e-6 kg m^-2` to the same SWE residual and
vapor-to-sublimation transfer identity. It does not replace the independent
vapor-aggregation or density-layer lifecycle predicates.

### `B-L1` — PASS

Canonical contract prose and tables use `1e-6 kg m^-2`. No binary64
serialization tail is promoted into normative authority.

## Contract Integration

The amendment is integrated into the canonical invariant and guard map,
constants/parameters table, numeric notes, Binding Exposure Index, resolved gap
record, and version-6 change log. Package-local reconciliation therefore does
not stand in for canonical authority.

No unresolved authority finding remains in this verification scope. Phase B
may be authorized after the second independent authority verification also
passes and the authority seal is updated by the package owner.
