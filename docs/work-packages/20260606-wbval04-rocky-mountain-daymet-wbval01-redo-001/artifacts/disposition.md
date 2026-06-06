# Disposition

Status: executed-hold

Evidence mode: mixed `Static:` and `Ran:`

Final disposition: executed-hold on valid-climate openWEPP invariant failures.

Ran:

- Climate precondition audit passed: zero current CLI rows have
  `rad > baseline sunmap.r3`.
- Release `openwepp-cli-hill` build passed at source commit
  `5b23ef27d398e69bf754be730d28fce63a38c131`.
- WBVAL04 ran all `22` single-OFE hillslopes.
- `18/22` emitted WAT.
- `4/22` failed closed with `HKERNEL-WB11-PERC-E-003` at J-95.
- All `18` WAT emitters are conservation-break for years `2..6` under the
  complete declared identity.

Static:

- No Rust production code, canonical contracts, Rust tests, WEPPpy files, or
  `/wc1` inputs were edited.
- `pw0` remains outside the single-OFE closure scope.
- Dual review, finding disposition, and dual verification are complete with no
  undispositioned findings.

Disposition rationale:

- WBVAL04 achieved the WBVAL01 redo objective: the upstream climate precondition
  now passes and the full single-OFE validation population has fresh run
  results.
- The package cannot close as complete because valid-climate fail-closed and
  conservation residual violations remain.
- The remaining work is outside WBVAL04's validation-only correction authority
  and is routed to defect-shaped follow-ons:
  `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` and
  `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`.

Closure rule satisfied: no review findings remain undispositioned.
