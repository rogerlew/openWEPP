# Disposition

Status: complete

Evidence mode: Static + Ran.

## Decision

`FQ4-FROST-KSFLAG-ACTIVATION-001` is closed.

The in-envelope root cause was an overbroad frost activation gate:
`frost.options.frost_file_present=0` suppressed frozen-soil coupling even when
the parsed missing-file default controls supplied valid standard frost controls
with `frost.options.wintRed=1`. `SC-SNOWFREEZE-001` v53 now specifies that frost
file presence is provenance only; `wintRed=1` plus active thermal/runtime
triggers activates `frsoil`.

## Acceptance Criteria

- Frost activation: satisfied (`43/43` prefixes `frsoil.active=true` and
  nonzero `frozwt`).
- Conductivity bite: satisfied on p8 (`infcap_frz=9.17e-11` vs
  `ssc=9.17e-06` in final manifest).
- ksflag/frost on/off no longer identical: satisfied on p8 paired run.
- Corrected closure ledger: satisfied; FROSTVAL01 `frost-break` withdrawn and
  full WAT identity closes with max abs residual
  `3.2173375075217336e-11 mm`.
- Contract-derived tests: satisfied.
- No comparator matching or protected-boundary compensation: satisfied.
- Dual review and dual verification: satisfied.

## Finding Disposition

- Accepted Review A finding 1: inactive tests now use `wintRed=0`.
- Accepted Review A finding 2: FROSTVAL01 `frost-break` explicitly withdrawn.
- Accepted Review B finding 1: manifest `frsoil.active` fixed to match
  corrected activation semantics.
- Follow-up Review B finding 2: runoff magnitude re-baselining noted for any
  future magnitude characterization.
- Follow-up Claude review finding F2: frost depth-model magnitude remains
  outside this activation/conservation closure. The amended review treats the
  strong annual-crop `kfactor=1e-5` conductivity bite as legacy-faithful
  concrete-frost behavior, not an openWEPP magnitude defect. The remaining
  carry-forward is the openWEPP freeze-index frost-depth proxy versus the
  legacy heat-flow frost-depth chain, under existing `GAP-SNOWFREEZE-002`;
  this is recorded in `worker-handoff.md`.
- Rejected-as-closure-blocker Claude review finding F3: the post-fix `43/43`
  WAT population result does not close `FQ1-P11`; p11 may be frost-masked
  rather than fixed. This is recorded in `worker-handoff.md`.
- Rejected-as-closure-blocker Claude review finding F4: comparator ownership
  was inferred from the unambiguous provenance-flag gate and runtime evidence
  rather than a `wepp_260606_hill` frost-depth comparator run. This remains
  acceptable under the package envelope; comparator magnitude sanity checking is
  recorded as future work, not an activation-closure prerequisite.

No undispositioned blocking findings remain.
