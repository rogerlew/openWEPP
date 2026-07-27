# Execution Incident 001

Status: `CLOSED BEFORE POPULATION`

Evidence class: `Ran`

The first observed attempt passed `prepare`, `build_executor`, and
`build_production_runner`, then stopped at `native_proof`. The script requested
the nonexistent release path `expected_probe`; Cargo's declared binary is
`expected-probe`. No Hubbard population, freeze, or Harvard content command
ran.

The append-only attempt was preserved without deletion at
`/home/workdir/cal04b-objects-failed-native-proof-001`. Its failure receipt,
stdout/stderr, output manifest, and three preceding PASS receipts remain the
canonical incident evidence.

Correction: `tools/native-proof.py` now resolves the exact declared
`expected-probe` binary path. The corrected attempt uses a new empty
`/home/workdir/cal04b-objects` root and must execute the complete observed DAG
from `prepare`; no receipt is reused or rewritten.

