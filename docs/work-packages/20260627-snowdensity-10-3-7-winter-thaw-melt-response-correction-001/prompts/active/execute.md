# Execute SNOWDENSITY-10.3.7

Read `package.md` first. Execute the package end-to-end under the contract-first
sequence:

1. Amend `SC-SNOWFREEZE-001` before code.
2. Add contract-derived tests.
3. Implement the opt-in `coe_winter_thaw_state_loss_v1` correction.
4. Rerun paired thaw-ablation diagnostics against `legacy_coe`.
5. Continue until the package either satisfies every exit criterion or closes
   with a named `HOLD` blocker.

Do not stop after scaffolding, selector parsing, focused unit tests, or a partial
diagnostic. Closure requires current-run evidence for package gates, dual
review/disposition, dual verification, line-count governance, and a truthful
worker handoff.
