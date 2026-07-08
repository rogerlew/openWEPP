# Fixture Manifest Evidence

Status: `passed`

Evidence mode: `Ran:`

Accepted fixture:
`tests/fixtures/watershed/p102-sediment-active/`

Manifest:
`tests/fixtures/watershed/p102-sediment-active/input-manifest.sha256`

Validation:

```sh
(cd tests/fixtures/watershed/p102-sediment-active && sha256sum -c input-manifest.sha256)
```

Result: all `18` entries `OK`.

The manifest hashes resolved symlink contents for p102 source inputs and direct
contents for the generated watershed wrapper files.
