# openWEPP Binary Release Contract

Status: `normative`

This contract defines binary naming, sidecar requirements, and release lint
gates for openWEPP artifacts.

## Release naming

Canonical release tag prefix: `openwepp_YYMMDD*`

Accepted binary-name regexes:

- Watershed binary: `^openwepp_[0-9]{6}[a-z0-9_-]*$`
- Hillslope binary: `^openwepp_[0-9]{6}[a-z0-9_-]*_hill$`
- Replay binary: `^openwepp_[0-9]{6}[a-z0-9_-]*_replay$`

Examples:

- `openwepp_260511`
- `openwepp_260511_hill`
- `openwepp_260511_replay`
- `openwepp_260511a`
- `openwepp_260511a_hill`
- `openwepp_260511a_replay`

## Sidecars (mandatory)

Each binary must ship with a JSON sidecar at:

```text
<binary_path>.json
```

Missing sidecar is a hard error.

## Sidecar emission command surface

Canonical operator automation surface:

```text
open_wepp_runner release sidecar --binary <path> --role <watershed|hillslope|replay>
```

Requirements:

1. `--binary` and `--role` are required.
2. Role must be one of `watershed`, `hillslope`, `replay`.
3. Command writes/refreshes `<binary_path>.json`.
4. Emitted sidecar must validate against required schema/field set before
   success return.

## Sidecar schema

Schema ID:

```text
openwepp-binary-release-metadata-v1
```

Versioning policy:

- `schema_major` bump for breaking changes.
- `schema_minor` bump for additive-compatible fields.

Baseline field set mirrors existing WEPP sidecar practices before adding
openWEPP-specific fields.

Required top-level fields:

- `schema`
- `binary_name`
- `binary_role` (`watershed`, `hillslope`, `replay`)
- `release_tag`
- `source_repo`
- `source_commit`
- `built_utc`
- `sha256`
- `features` (object)
- `validation` (object)

Required feature fields:

- `hbp_supported` (boolean)
- `hbp_schema_major` (integer)
- `hbp_schema_minor` (integer)
- `hbp_pass_family` (string, expected `H*.hbp`)
- `legacy_ascii_pass_family` (string, expected `H*.pass.dat`)
- `mode2_master_pass_prompt_required` (boolean)

Required validation fields:

- `schema_valid` (boolean)
- `release_lint_level` (string)
- `validated_utc` (RFC 3339 UTC string)

## Release lint gate

Every release candidate must pass a lint gate that validates:

1. binary filenames match regex policy;
2. sidecars exist for each binary;
3. sidecar schema ID and required fields are valid;
4. watershed/hillslope pair agrees on `features.hbp_supported`.

Implementations may expose this as a command such as:

```text
open_wepp_runner release lint --release-dir <path>
```

The gate is blocking: lint failure means release is not publishable.
