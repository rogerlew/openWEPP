# CQR18 Kickoff

Execute
`docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001/package.md`
end to end.

Protected boundaries:

- Preserve public API, HBP binary schema, payload CRC and bounds behavior,
  event-kind behavior, state snapshot registry validation, required-state
  obligations, parser compatibility, and typed HBP error IDs.
- Perform behavior-preserving decomposition only for the scoped live metric
  target in `payload_validator.rs`.
- Record before/after LCOV and CRAP evidence, full gates, dual reviews, dual
  verification, disposition, handoff, commit, push, and tracker update.
