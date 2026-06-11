# Disposition

Status: complete-after-follow-ons
Evidence mode: Static + Ran

Decision:
- Close FROSTVAL01 after the 2026-06-11 rerun on the repaired substrate.
- Report frost activation and closure-under-frost success for the fresh rerun only.
- Preserve the original `executed-hold` artifacts as historical evidence for the
  first blocked run.

Why the original run held:
- Milestone 1 requires proving frost activation before trusting closure-under-frost.
- 37/43 single-OFE targets are blocked by `HS-RUNTIME-E-062` before activation evidence can be measured.
- In the 6 runnable targets, ksflag on/off runs show no frost-activation signal (`frozwt` zero and on/off deltas zero).
- Runnable targets also show large annual closure residuals and classify as `frost-break`.

What the 2026-06-11 rerun completed:
- Built `openwepp-cli-hill` with `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Ran all 43 single-OFE prefixes frost-on and frost-off under
  `/tmp/frostval01_rerun_20260611T020951Z`.
- Final status: 43/43 frost-on exits clean, 43/43 frost-off exits clean.
- Activation: 43/43 `frsoil.active=true`, 43/43 nonzero `frozwt`, 43/43 nonzero
  paired `Q` and `latqcc` deltas.
- Closure: corrected full-WAT annual identity over 258 rows closes with max abs
  residual `3.2173375075217336e-11 mm`.

What the original execution completed:
- End-to-end attempted execution for all 43 single-OFE targets.
- Paired ksflag on/off reruns for all reachable targets.
- Activation and closure ledgers with explicit blocked/deferred accounting.
- Legacy totalwatsed3 comparator audit evidence.
- Runnable-subset totalwatsed3 audit evidence (6-prefix subset).

What is deferred:
- Frost depth magnitude/parity remains outside FROSTVAL01 and is tracked by
  FDMC01/FDHP01.
- MOFE/routing and forest `ksatadj` remain outside this standard-`ksflag`
  single-OFE validation.
