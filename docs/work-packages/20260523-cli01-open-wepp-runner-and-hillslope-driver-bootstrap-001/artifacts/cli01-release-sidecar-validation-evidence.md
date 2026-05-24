# CLI01 Release-Sidecar Validation Evidence

Status: complete
Evidence mode: Static + Ran

## Static
- Sidecar writer and validator surfaces:
  - `write_release_sidecar_for_binary`
  - `validate_release_sidecar`
  - `lint_release_directory`
- Contract-required validation object fields implemented and validated:
  - `schema_valid`
  - `release_lint_level`
  - `validated_utc`

## Ran
1. Generated runtime sidecar for active hillslope binary:

```text
/home/workdir/openWEPP/target/debug/openwepp-cli-hill.json
```

Excerpt:

```json
{
  "schema": "openwepp-binary-release-metadata-v1",
  "binary_name": "openwepp-cli-hill",
  "binary_role": "hillslope",
  "validation": {
    "schema_valid": true,
    "release_lint_level": "contract_v1",
    "validated_utc": "2026-05-24T05:20:52.244619123Z"
  }
}
```

2. Validated release-lint gate on synthetic release directory:

```text
open_wepp_runner release lint --release-dir /tmp/cli01_release_lint_uQLT9H
exit=0
```

Directory contents validated:

```text
openwepp_260523_cli01
openwepp_260523_cli01.json
openwepp_260523_cli01_hill
openwepp_260523_cli01_hill.json
```

3. Contract integration test evidence:

```text
cargo test --test cli01_runner_hillslope_integration
...
cli01_contract_conformance_generated_release_sidecar_is_schema_valid ... ok
```
