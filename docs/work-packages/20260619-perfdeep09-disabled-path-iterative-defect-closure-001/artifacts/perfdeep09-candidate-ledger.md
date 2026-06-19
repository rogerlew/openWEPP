# PERFDEEP09 Candidate Ledger

Status: complete.
Evidence class: Static + Ran.

| Candidate | Mechanism | Result | Disposition | Rationale |
|---|---|---|---|---|
| PERFDEEP08 diagnostic-hook cache | disabled diagnostic hooks | `691.93 s`, RSS `229444 KB` | rejected baseline history | slower than PERFDEEP07 `685.85 s`; not repeated |
| Registry reverse lookup `HashMap` | replace private `SymbolRegistry` `BTreeMap` reverse lookup | `689.30 s`, RSS `229352 KB` | rejected and reverted | slower than PERFDEEP09 control `682.65 s`; PASS raw checksum drift also made it identity-risky |
| Perennial decomposition one-pass indexed-overflow guard | replace seven full state-map scans with one slot/crop pass while preserving root-order error precedence | screen `634.61 s`; final median `635.65 s` | retained | clears P0 gate with focused tests, protected identity, and full closure gates |
