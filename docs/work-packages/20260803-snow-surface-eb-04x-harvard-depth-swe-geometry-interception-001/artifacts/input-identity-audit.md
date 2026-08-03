# Input Identity Audit

Status: pass

Evidence mode: **Ran**

- All 12 freeze input hashes reproduce.
- The current analysis tool SHA-256 is
  `cd7456594983238eae5351b6556391df7f30ecf9fc1288c54572a38606970bc9`
  and matches `freeze.json`.
- All eight ignored target trace hashes reproduce the committed
  `factorial-results.json` cell `trace_sha256` identities: Harvard open and
  hardwood B, L, S, and LS are exact retained outputs, not newly executed or
  substituted traces.
- The committed factorial result, normalized strata/profile observations, and
  provenance JSON are separately hash-bound in the freeze.
