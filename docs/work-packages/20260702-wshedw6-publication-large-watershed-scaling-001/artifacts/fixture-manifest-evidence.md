# Fixture Manifest Evidence

Status: `passed`

Evidence mode: `Ran:`

## Onshore-Xenophobia

Fixture path: `tests/fixtures/watershed/onshore-xenophobia/`.

Manifest path: `tests/fixtures/watershed/onshore-xenophobia/input-manifest.sha256`.

Manifest creation command:

```sh
cd tests/fixtures/watershed/onshore-xenophobia
LC_ALL=C find runs \( -type f -o -type l \) -print | sort | xargs sha256sum > input-manifest.sha256
```

Validation command:

```sh
cd tests/fixtures/watershed/onshore-xenophobia
sha256sum --quiet -c input-manifest.sha256
```

Result: `PASS`.

Manifest line count:

```text
7847 input-manifest.sha256
```

Notes:

- Manifest includes `runs/case.run`, all `pN.source.run` launch files, legacy
  `pN.*` source inputs, `pw0.*`, sidecars, climate symlinks, and
  `runs/shared/onshore-xenophobia.cli`.
- Symlink entries hash the target file content, matching the runner's read path.

## Carnivorous-Adobo

Fixture path: `tests/fixtures/watershed/carnivorous-adobo/`.

Manifest path:
`tests/fixtures/watershed/carnivorous-adobo/input-manifest.sha256`.

Validation command:

```sh
cd tests/fixtures/watershed/carnivorous-adobo
sha256sum --quiet -c input-manifest.sha256
```

Result: `PASS`.

Manifest line count:

```text
208 input-manifest.sha256
```

Notes:

- Manifest includes `runs/case.run`, all `pN.source.run` launch files, legacy
  `pN.*` source inputs, `pw0.*`, and sidecars.
