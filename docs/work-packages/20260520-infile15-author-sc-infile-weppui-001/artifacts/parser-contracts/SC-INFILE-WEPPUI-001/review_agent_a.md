Evidence: Static

## Findings (Severity-Ranked)

### WUI-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:39`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:69`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:70`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:160`
- Issue: Strict-mode matrix/policy says non-ENOENT open failures are typed IO faults, but `ui_run` derivation currently collapses open-fail with absence (`0` when absent/open-fail).
- Why it matters: This is an internal executable inconsistency that can silently downgrade strict IO faults into daily-mode defaults.
- Proposed disposition: amend

### WUI-A-002 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:128`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:65`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:137`
- Issue: Contract requires requested/effective mode divergence observability, but the data model and boundary export surfaces contain only effective sentinel-derived mode state.
- Why it matters: The stated invariant cannot be executed or verified without an explicit requested-mode surface and divergence marker.
- Proposed disposition: amend

### WUI-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:73`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:84`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:126`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:157`
- Issue: `solwpv` cross-file coupling is central to hourly-mode policy, but propagation/guard surfaces do not define how multi-soil contexts are reduced to one compatibility decision.
- Why it matters: Ambiguous reduction rules can cause non-deterministic strict-vs-compat outcomes across implementations.
- Proposed disposition: amend

Final recommendation: HOLD
