# Verification Agent B

Status: `completed-local-substitution`

Evidence mode: `Ran:` local verification and `Static:` artifact review.

Verification result: W7R acceptance evidence matches the package exit criteria.

Checks:

- A committed full watershed fixture exists:
  `tests/fixtures/watershed/p102-sediment-active/`.
- The fixture produces actual nonzero sediment from generated hillslope pass
  artifacts.
- Public watershed execution uses typed dispatch and typed publication.
- `--jobs 1` and `--jobs 4` outputs are decoded-identical.
- Sediment reconstruction rejects zero-fill and simple detachment/deposition
  aliases.
- Final gates are recorded in `gate-results.md`.

No verification blocker remains.
