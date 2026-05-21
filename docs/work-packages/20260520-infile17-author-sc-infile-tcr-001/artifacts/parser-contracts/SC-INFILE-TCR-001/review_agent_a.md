Evidence: Static

## Findings (Severity-Ranked)

### TCR-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:81`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:97`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:149`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:182`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:186`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:104`
- Issue: The contract enforces override guards that depend on channel-slope/topology context (`G-TCR-005`, `G-TCR-009`), but the field/propagation model does not define explicit source/simulation rows for those required cross-file inputs.
- Why it matters: Guard closure is not executable if required dependency surfaces are not modeled and propagated explicitly; this violates parser-contract completeness and leaves override safety dependent on implicit state.
- Proposed disposition: amend

### TCR-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:135`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:166`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:180`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:124`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:125`
- Issue: Domain/relational invariant handling is encoded as hard semantic failure (`TCR-E-004`) without explicit compat-mode branch behavior, while the paired spec currently records compat warning/preserve-flow behavior for the same conditions.
- Why it matters: Strict-vs-compat behavior is underspecified/inconsistent across contract vs spec authority, which can produce divergent parser outcomes and non-reproducible compatibility behavior.
- Proposed disposition: amend

Final recommendation: HOLD
