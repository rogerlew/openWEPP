Evidence: Static

## Findings (Severity-Ranked)

### CHN-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:70`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:101`
- Issue: Field specification enumerates many externally relevant symbols (`ver`, `nchan`, `ipeak`, `lw`, all `chn*`, `ctl*`, `rc*` fields), but propagation map is grouped into coarse bundles rather than symbol-level rows.
- Why it matters: This does not satisfy per-field propagation traceability required by parser-contract requirements, and it obscures field-level ownership, guard binding, and downstream usage.
- Proposed disposition: amend

### CHN-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:96`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:97`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:123`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:161`
- Issue: Derived externally relevant fields (`has_rating_curve`, `control_override_applied`) are defined, but they do not have explicit dedicated propagation/boundary-export rows.
- Why it matters: Runtime consumers cannot rely on a stable contract for these branch-control semantics, increasing risk of parser/runtime divergence.
- Proposed disposition: amend

### CHN-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:37`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:169`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:177`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:136`
- Issue: Strict-vs-compat datver/support policy is still partially non-executable: matrix/policy describe compat acceptance, but taxonomy/contracted outcomes do not define explicit compatibility warning types for accepted non-canonical variants.
- Why it matters: Unsupported/compat behavior precision is a hard parser-contract requirement; without explicit typed outcomes, implementations will drift.
- Proposed disposition: amend

Final recommendation: HOLD
