# Case-4 Hybrid Cooldown Scan (Exploratory)

## Commands and exit codes

- `OPENWEPP_IWAGAKI_HYBRID_COOLDOWN_S=0 cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle' --profile quick --run-ignored ignored-only` (exit: `100`)
- `OPENWEPP_IWAGAKI_HYBRID_COOLDOWN_S=5 cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle' --profile quick --run-ignored ignored-only` (exit: `100`)
- `OPENWEPP_IWAGAKI_HYBRID_COOLDOWN_S=10 cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle' --profile quick --run-ignored ignored-only` (exit: `100`)
- `OPENWEPP_IWAGAKI_HYBRID_COOLDOWN_S=20 cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle' --profile quick --run-ignored ignored-only` (exit: `0`)

## Failure peak text (ladder failures)

- `OPENWEPP_IWAGAKI_HYBRID_COOLDOWN_S=0`: 
  `hybrid peak error at ladder step 0: 0.2278616770287263 (ref 0.008334954309852328); ladder [0.2278616770287263, 0.1546442906262445, 0.1020328027401588]`
- `OPENWEPP_IWAGAKI_HYBRID_COOLDOWN_S=5`: 
  `hybrid peak error at ladder step 0: 0.18823709792306398 (ref 0.008334954309852328); ladder [0.18823709792306398, 0.12431704651221594, 0.08013032657411565]`
- `OPENWEPP_IWAGAKI_HYBRID_COOLDOWN_S=10`: 
  `hybrid peak error at ladder step 0: 0.13110798999821136 (ref 0.008334954309852328); ladder [0.13110798999821136, 0.08141395499148471, 0.050327402891367674]`

## Pass/fail summary

- Pass/fail ladder observed: `0: FAIL`, `5: FAIL`, `10: FAIL`, `20: PASS`
- Execution stopped early on first bracketed change (`10` fail, `20` pass).

## Recommendation

Bracketed cooldown transition is between `10s` and `20s`; schedule a follow-up focused scan in that interval (e.g., `12`, `14`, `16`, `18` seconds) and capture the first passing cooldown.

## Logs

- Command log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/case4-cooldown-0s.log`
- Command log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/case4-cooldown-5s.log`
- Command log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/case4-cooldown-10s.log`
- Command log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/case4-cooldown-20s.log`
- Command run-log text: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/verification-case4-cooldown-command-log.txt`
