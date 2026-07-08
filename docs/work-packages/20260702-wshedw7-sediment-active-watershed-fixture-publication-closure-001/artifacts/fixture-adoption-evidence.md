# Fixture Adoption Evidence

Status: `passed`

Evidence mode: `Static:` committed fixture review plus `Ran:` release replay.

W7R adopts
`tests/fixtures/watershed/p102-sediment-active/` as the sediment-active
watershed acceptance fixture.

The fixture is a complete one-channel watershed wrapper around the committed
W7DC01 p102 two-OFE source substrate. It does not commit generated pass/HBP
outputs and does not edit sediment values.

Release replay:

- `--jobs 1`: passed, `wall=0:00.78`
- `--jobs 4`: passed, `wall=0:00.74`
- public `totalwatsed3` sediment is nonzero:
  `tdet=584.2332653870001`, `tdep=282.14618621700004`,
  `sed_del=0.08391307754719238`

Detailed provenance is in `fixture-provenance.md`.
