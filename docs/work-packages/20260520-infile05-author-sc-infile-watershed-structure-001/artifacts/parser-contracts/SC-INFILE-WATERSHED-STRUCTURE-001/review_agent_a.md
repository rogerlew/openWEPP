Evidence: Static

## Findings (Severity-Ranked)

### WST-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:62`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:83`
- Issue: Externally relevant contributor fields are specified per-symbol (`nhleft`, `nhrght`, `nhtop`, `ncleft`, `ncrght`, `nctop`, `nileft`, `nirght`, `nitop`), but propagation is collapsed into one grouped row (`contributor IDs (nh*,nc*,ni*)`) instead of explicit per-field propagation rows.
- Why it matters: `parser-contract-requirements.md` requires a propagation row for every externally relevant field; grouped propagation hides symbol-level ownership/mutability/guard coverage and weakens parser-to-runtime traceability.
- Proposed disposition: amend

### WST-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:62`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:125`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:134`
- Issue: `nhmax`/max-hillslope-reference closure is enforced in cross-file constraints and error taxonomy (`STR-E-009`) but is not defined as an explicit derived field in the field table and is not propagated as a first-class runtime field.
- Why it matters: A guardable invariant is referenced without an explicit data-model surface, which creates implementation ambiguity and weakens verification of topology-coverage closure.
- Proposed disposition: amend

### WST-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:37`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:114`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:159`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:163`
- Issue: Compatibility-mode acceptance of legacy no-datver form is required to be observable via typed warnings, but warning-class outcomes are not represented in the contract’s validation/error taxonomy or guard-failure mapping.
- Why it matters: Compatibility behavior is under-specified for executable parser outcomes; implementers cannot consistently encode strict-vs-compat observability.
- Proposed disposition: amend

Final recommendation: HOLD
