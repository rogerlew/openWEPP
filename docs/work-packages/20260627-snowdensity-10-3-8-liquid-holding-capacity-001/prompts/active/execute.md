# Execute SNOWDENSITY-10.3.8

You are in `/home/workdir/openWEPP`. Execute
`docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/`
end to end.

Required reading:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3 step 6
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `references/copyrighted/noaa_6392_DS1.md` liquid-water retention/transmission
  sections
- `references/annotated_bibliography.md` R-55 and R-36

Execute contract-first:

1. Amend `SC-SNOWFREEZE-001`.
2. Add contract-derived tests.
3. Implement the opt-in candidate.
4. Run diagnostic event-window and coupled WAT gates.
5. Close package only with current-scope evidence for conservation and coupled
   WAT. If either gate fails or cannot run, close `HOLD` with the concrete
   blocker.

Do not default-activate, tune fixture constants, edit fixture inputs, add
parser/runfile/user CLI selectors, change public schemas, or change melt
coefficients, radiation, canopy, phase partition, density compaction, frost,
sub-canopy longwave, rain heat, Qwet/frzftp, or compatibility-runtime behavior.
