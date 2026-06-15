# Parser Profile Compliance Checklist

Status: complete
Evidence mode: Static + Ran

- [x] No public parser API changes.
- [x] No new compatibility fallback or silent default.
- [x] No broad error swallowing.
- [x] HBP error codes and error-message details covered by focused characterization remain stable.
- [x] Binary read order remains stage-equivalent to the original parser.
- [x] Header, directory, table, file, stored-block, and raw-block checksum windows remain explicit.
- [x] Schema 1.x and 2.x branch behavior remains separated and typed.
- [x] Focused parser suite passed after refactor.
