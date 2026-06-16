# PERFOPT01 Verification A

Status: LOCAL VERIFICATION COMPLETE 2026-06-16
Evidence mode: **Ran**

Scope: fixture identity and determinism verification.

## Verification Result

PASS.

## Evidence

- Baseline vs optimized fixture comparison across OFE1-OFE5, H2637 without UI, and H2637 with UI: `anchor_mismatches = 0`.
- HBP, loss JSON, and ASCII optional-output marker files matched byte-for-byte.
- Readable Parquet files matched by `pyarrow` table equality.
- Optimized OFE5 repeated-run determinism passed:

```text
PERFOPT01_DETERMINISM_OK
```

Full comparison transcript is recorded in `perfopt01-bit-identity-and-determinism-evidence.md`.

## Limitation

This is local verification by the primary agent, not an independent delegated verifier.

