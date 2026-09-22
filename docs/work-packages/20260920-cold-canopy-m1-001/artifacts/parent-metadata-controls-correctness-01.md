Static review of [m1_private_provider_metadata_controls.py](/workdir/openWEPP/docs/work-packages/20260920-cold-canopy-m1-001/artifacts/m1_private_provider_metadata_controls.py).

- **High — negative controls are tautological.** Lines 62–65 only construct unequal Python tuples and assert `bad != receipts`. They do not parse a receipt, invoke provider admission, verify run/cycle/support/source joins, return `VEG-E-144`, or prove zero owner/publication mutation. They cannot satisfy the contract’s missing/foreign/reordered/stale receipt controls.

- **High — the KAT does not represent an actual provider payload.** It uses a calendar containing only `origin`, zero implementation digest, synthetic `11…11` payload digest, and synthetic `22…22` GSI-source digest. It therefore tests generic framing mechanics, not the canonical M1 payload or authenticated source correspondence.

- **High — two required domains are absent.** There is no calendar-receipt KAT and no parent-forcing-receipt KAT, including no full-projection digest, 30-member ordered GSI-receipt collection, or parent-record binding.

- **Medium — canonicalization controls are absent.** The script does not exercise duplicate/unknown keys, key order, escaping, integer/bit encodings, JSON Pointer rules, source-adapter output, cycle/initial-phase extraction, or payload mutation. Default `json.loads` also cannot detect duplicate keys without an `object_pairs_hook`.

- **Medium — fixture controls are partial.** They check broad contiguity and the first breakpoint but do not independently expand 4,320 records, verify every 60-second support, enumerate both cycles’ breakpoints, or prove projected scalar equality.

The hardcoded run/GSI digest constants may remain useful regression smoke tests, but their independent derivation is unrecorded. The author’s reported PASS is exploratory and does not satisfy the prereview or contract-derived control gate.

**Verdict: HOLD for qualifying controls.** Retain this artifact only as a diagnostic framing KAT. Qualification requires actual final payload known answers independently reconstructed, all four receipt domains, and real validator rejection tests asserting `VEG-E-144` plus unchanged staging/publication state.
