# SNOWDENSITY-10.3.5a Kickoff Prompt

You are working locally in `/home/workdir/openWEPP`. Execute
`docs/work-packages/20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/`
end-to-end.

Autonomy: complete the package through contract amendment, crate implementation,
tests, package evidence, review, verification, and final disposition without
asking for next steps unless a named hard blocker is reached.

Subagent authorization: this package explicitly authorizes spawning/delegating to
read-only review and verification subagents for dual package review, clean-room
provenance review, no-production-wiring review, and final verification. Expected
outputs are package-local `review_agent_a.md`, `review_agent_b.md`,
`verification_agent_a.md`, and `verification_agent_b.md`. Subagents have
read-only access unless a later operator request explicitly grants bounded write
access.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/package.md`

Conditional, required for this package because it amends canonical snow/freeze
authority and implements new candidate physics:

- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

On-demand:

- `/home/workdir/openWEPP/docs/planning/snow-frost-fidelity-strategy.md`
- `/home/workdir/openWEPP/references/annotated_bibliography.md`
- `/home/workdir/openWEPP/references/copyrighted/source_pdfs/harder2013.pdf`
- `/home/workdir/MetPy` only as BSD-3 numeric/reference cross-check for standard
  primitives.

Required-reading budget: current required local pre-read set is 648053 bytes
(`WARN`, below the 800000-byte requires-justification threshold). Record the
reading map in `artifacts/required-reading-map.md`.

## Execution Rules

- Contract-first sequence is mandatory:
  1. amend `SC-SNOWFREEZE-001`;
  2. add contract-derived tests or source-level contract assertions;
  3. record pre-implementation contract gate evidence;
  4. implement `crates/openwepp-meteorology`.
- Do not implement production snow/frost runtime wiring in this package.
- Do not edit `snow.hourly.stmtim.rst_c`, legacy `RST` behavior, parser/runfile
  selectors, public output schemas, snowbench production routing, or defaults.
- Do not read, port, paraphrase, or copy CHM/GPL code.
- MetPy may be used only as numeric/reference cross-check for standard
  primitives; do not translate MetPy implementation.
- Do not add site-specific calibration, fallback wrappers, silent defaults,
  broad `Box<dyn Error>` production errors, or `.unwrap()`/`.expect()` in
  production paths.
- Invalid domains and solver non-convergence must return typed errors.
- A phase may be marked complete only when all required gates for that phase have
  current direct evidence. Otherwise close `HOLD` with the blocker named.

## Required Work

1. Complete `artifacts/required-reading-map.md`.
2. Amend `SC-SNOWFREEZE-001` for a candidate-only Harder-Pomeroy
   psychrometric phase method, including inputs, units, invariants, rollback, and
   no-site-calibration obligations.
3. Add contract-derived tests or source-level checks proving the candidate
   authority and rollback language.
4. Add `crates/openwepp-meteorology` as a workspace member with pure checked
   functions and typed errors.
5. Implement psychrometric primitives and Harder-Pomeroy `Ti` plus
   rainfall-fraction mapping from cited authority.
6. Add crate-local tests for reference values, round trips, solver vectors,
   monotonicity, bounds, invalid domains, and non-convergence behavior.
7. Complete `artifacts/clean-room-provenance.md`.
8. Run and record no-production-wiring scans.
9. Run and record gates:
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - `cargo deny check`
10. Complete review, disposition, verification, line-count governance,
    worker handoff, and final package disposition.

## Closure

Close `COMPLETE-10-3-5A-METEOROLOGY-CRATE` only when every current-scope gate has
direct current evidence and no accepted review finding remains unresolved.
Otherwise close with a named `HOLD-...` blocker.
