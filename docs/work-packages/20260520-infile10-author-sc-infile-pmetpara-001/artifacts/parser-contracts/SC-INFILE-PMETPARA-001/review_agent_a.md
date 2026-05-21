Evidence: Static

## Findings (Severity-Ranked)

### PMET-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:37`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:58`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:72`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:132`
- Issue: Optional-surface absence (missing `pmetpara.txt`) is a normative mode branch, but the contract does not define an explicit source/simulation field and propagation row for sidecar presence/mode state.
- Why it matters: Required parse-to-simulation propagation is incomplete for a top-level control branch (`iflget` behavior), making mode selection under-specified at runtime boundaries.
- Proposed disposition: amend

### PMET-A-002 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:67`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:81`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:97`
- Issue: `fallback_first_row_used` is defined as lookup-time derived behavior but marked immutable in runtime state.
- Why it matters: Lookup outcomes are execution-time branch results; immutable classification obscures ownership/mutability semantics and weakens guardable state modeling.
- Proposed disposition: amend

### PMET-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:123`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:106`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:119`
- Issue: Cross-file consistency constraints are stated at a high level and do not identify concrete coupled surfaces/fields (for example the management crop-name authority symbol path and normalization-width coupling).
- Why it matters: Cross-file constraints must be executable and auditable; generic statements do not give deterministic closure checks.
- Proposed disposition: amend

### PMET-A-004 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:109`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:177`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:191`
- Issue: Delimiter/quoting ambiguity for `actlnam` is tracked as HOLD, but taxonomy/guards do not define a provisional explicit failure surface for unresolved quoted/multi-token forms.
- Why it matters: Parser behavior at tokenization boundaries can diverge silently without a typed contract-level outcome.
- Proposed disposition: amend

Final recommendation: HOLD
