# Implementation And Test Evidence

Status: complete

Evidence mode: Ran

The orchestrator exposes a versioned, fingerprint-bound JSON snapshot and
restore API, rejects corrupt/unknown/wrong-lane/out-of-order state, and commits
only a validated candidate. Tests cover dormant then snowfall reappearance,
solid/liquid separation, exact closure, restart equivalence, and failed restore.
The runner owns optional per-lane state only for the explicit selector and emits
schema-v7 state, ledger, lifecycle, fingerprint, support, energy, and censoring
fields. A JSONL consumer parses the real row, reconstructs mass, and rejects the
snowfall/liquid alias.

Exact-head quick, frost, and full-workspace results are transcribed in
`gate-results.md`; all passed.
