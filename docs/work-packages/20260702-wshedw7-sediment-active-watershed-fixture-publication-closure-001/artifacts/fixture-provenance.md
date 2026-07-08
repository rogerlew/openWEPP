# Fixture Provenance

Status: `passed`

Evidence mode: `Static:` fixture files plus `Ran:` manifest validation.

Accepted fixture:
`tests/fixtures/watershed/p102-sediment-active/`

Source hillslope substrate:
`tests/fixtures/erosion_multi_ofe_p102/`

The fixture wraps the real W7DC01 p102 two-OFE disturbed-forest hillslope in a
complete one-channel watershed. The p102 input files are linked into the
watershed run directory as `H1.*` and `pw0.*` so both the generated hillslope
run and watershed parsers consume the same committed source data.

Committed wrapper files:

- `README.md`
- `input-manifest.sha256`
- `runs/case.run`
- `runs/H1.source.run`
- `runs/pw0.str`
- `runs/pw0.chn`
- `runs/pw0.imp`
- `runs/chan.inp`

Resolved source links:

- `runs/H1.{man,slp,cli,sol}`
- `runs/pw0.{man,slp,cli,sol}`
- `runs/pmetpara.txt`
- `runs/wepp_ui.txt`
- `runs/snow.txt`
- `runs/frost.txt`

Validation:

```sh
(cd tests/fixtures/watershed/p102-sediment-active && sha256sum -c input-manifest.sha256)
```

Result: all `18` manifest entries `OK`.

No generated output files are committed.
