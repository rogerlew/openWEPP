# Acceptance Matrix

Status: `SCAFFOLD REVIEW REQUIRED`

Evidence class: `Static`

| ID | Obligation | Acceptance evidence |
|---|---|---|
| DC-01 | Capabilities verify and consume exactly once | `generation_b_consumes_each_capability_once`: RED records two attempts; GREEN records one mutation-free pre-LIGHT verification, one rename per capability during audit, immutable consumed-root proof, and zero later consumption |
| DC-02 | Capability restart semantics fail closed | `light_failure_preserves_capabilities_and_post_consumption_restart_requires_new_dispatch`: retained capability-tree and terminal-attempt snapshots prove reuse only before consumption |
| DC-03 | Audit inventory reconstructs exactly once | `audit_reconstructs_external_inventory_once`: RED counter is two; GREEN counter is one and its proof is consumed downstream |
| DC-04 | Ledger admits exactly once | `audit_admits_ledger_once_and_verifies_without_readmission`: RED counter is two; GREEN admission counter is one, verifier admission counter is zero, and ledger bytes remain identical |
| DC-05 | Failure lifecycle is balanced | `audit_failure_records_started_then_one_terminal`: retained ledger proves STARTED precedes evaluation and exactly one typed terminal follows |
| DC-06 | Recovery is descriptor-relative | `publication_recovery_rejects_root_and_ancestor_swap`: deterministic races cannot redirect restore, rename, or delete |
| DC-07 | Stale attestations fail | `generation_b_rejects_stale_verifier_attestation`: dispatch and transaction freshness boundaries are exact |
| DC-08 | CSV and errors fail closed | `external_csv_rejects_header_drift_and_unknown_columns` plus typed-error assertions for receipt, custody, ledger, and identity |
| DC-09 | Predecessor cannot execute concurrently | Pre-production catalog/roadmap state identifies predecessor INVALID and this successor as sole active authority |
| DC-10 | Package authority is prospective | Canonical package-chain PASS from scaffold |
| DC-11 | Focused behavior is stable | Nextest, Clippy, CAL Python, hygiene gates |
| DC-12 | Critical correctness is current | Canonically admitted exact-head full workspace and anti-evasion receipts |
| DC-13 | Harvard remains sealed | Static custody review and absence of opening token |
| DC-14 | Closure is independently accepted | Dual review and dual terminal verification |
