# INIMPL08 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260521-inimpl08-support-peridot-2023-3-slope-format-001/package.md


You are executing `20260521-inimpl08-support-peridot-2023-3-slope-format-001`.

Objectives:
1. Ratify and encode Peridot `2023.3` hillslope `.slp` grammar in openWEPP spec
   and parser contract.
2. Implement parser support in `crates/openwepp-input-contract/src/parsers/slope.rs`
   without regressing canonical `97.5` behavior.
3. Add fixtures/tests for valid and invalid `2023.3` files.
4. Run full parser-package gates and record evidence.
5. Produce review/disposition/verification closeout artifacts.

Constraints:
- Keep canonical WEPP symbol continuity (`datver`, `nelem`/`nwsofe`, `azm`,
  `fwidth`, `nslpts`, `slplen`, `xinput`, `slpinp`); use aliases instead of
  replacement naming.
- Correctness over completion: unresolved high-severity findings remain `HOLD`.
- No silent correction of malformed input; all invariant violations must map to
  typed errors.
- Treat `2023.3` slope support as hillslope `.slp` scope only; do not fold in
  channel `2025.8` or `.slps` bundle parsing in this package.

Required outputs:
- Updated spec and contract docs.
- Parser implementation changes.
- Fixture and integration test additions.
- `artifacts/wave-gate-evidence.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/inimpl08_disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
