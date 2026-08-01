# Terminal Frozen-Protocol Audit

Status: `HOLD_PHYSICAL_OR_PROVENANCE_GATE`

Evidence class: `Ran + independent terminal review`

This audit supersedes the decision claim emitted by the frozen package tool.
It does not alter the frozen protocol, tool, attempt, or retained outputs.

## Blocking Contradiction

The frozen protocol requires the daily vapor-to-sublimation identity within
`1e-9 kg m^-2`. The executed consumer and generated report instead apply
`1e-6 kg m^-2`. Twelve cells exceed the frozen bound; the maximum is
`8.109983287707401e-8 kg m^-2` in `snotel_niwot_co/S`, about 81 times the frozen
tolerance.

The implemented value is dimensionally consistent with converting the
contract's `1e-9 m` SWE mass-closure tolerance using water density:
`1000 kg m^-3 * 1e-9 m = 1e-6 kg m^-2`. That strongly indicates a protocol
unit-transcription error. It cannot be corrected after results exist because
the explicit frozen text is the preregistered decision authority.

## Consequence

- Runtime execution remains 48/48 complete with exact selector and file
  provenance.
- The other WAT/trace/layer, conservation, finite-number, sign, chronology,
  mechanism-reach, and no-mutation checks remain useful diagnostic evidence.
- The complete frozen physical gate did not pass.
- Observation loading should not have occurred; all generated ordinal scores,
  effects, interactions, protected-group results, and the generated
  `CLOSE_NONPROMOTION_EMPIRICAL_RULE` decision are inadmissible for promotion or
  nonpromotion.
- The truthful EB-04R outcome is `HOLD_PHYSICAL_OR_PROVENANCE_GATE`.
- No rerun is authorized under this one-shot protocol.

The machine-readable failed-cell inventory is
`terminal-frozen-protocol-audit.json`.
