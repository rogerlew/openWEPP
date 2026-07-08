# Command Evidence

Status: executed.
Evidence mode: Ran.

## Commands

Ran:

```text
git status --short --branch
```

Result:

```text
## main...origin/main
 M crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs
 M crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
 M crates/openwepp-runner/src/hillslope/laned_active.rs
 M docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
 M docs/specifications/wepp-input-files/specs/plant-file.spec.md
 M docs/work-packages/README.md
 M tests/integration/laned_shadow_h2637.rs
?? docs/work-packages/20260708-laned-router-conditional-default-activation-001/
?? docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001/
```

Disposition: runner files, `SC-OFEROUTE-001`, `laned_shadow_h2637.rs`, and the
conditional-default-activation package are pre-existing unrelated worktree
changes. This package did not edit them.

Ran:

```text
git diff --check
```

Result: exit code `0`, no whitespace errors.

Ran:

```text
wctl doc-lint
```

Result:

```text
Running default: markdown-doc lint --staged --format json
{
  "summary": {
    "files_scanned": 0,
    "errors": 0,
    "warnings": 0
  },
  "findings": []
}
```

Disposition: staged-default wrapper scanned zero files because no files were
staged. Focused path lint was run directly below.

Ran:

```text
markdown-doc lint --path docs/specifications/wepp-input-files/specs/plant-file.spec.md --format json
markdown-doc lint --path docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001 --format json
markdown-doc lint --path docs/work-packages/README.md --format json
```

Result:

```text
plant-file.spec.md: 1 file scanned, 0 errors, 0 warnings
package directory: 19 files scanned, 0 errors, 0 warnings
docs/work-packages/README.md: 1 file scanned, 0 errors, 0 warnings
```

Ran:

```text
rg -n "ow-lanuse-1|routing_coefficients|landuse=4|landuse=3" \
  docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md \
  docs/contracts/openwepp-management-lanuse-authority-contract.md \
  crates/openwepp-input-contract/src/parsers/management.rs \
  tests/integration/infile_management_parser_contract.rs \
  tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man \
  tests/fixtures/disturbed_native_route_coefficients/p1.man
```

Result: source cross-check found matching native datver, sentinel, routing
marker, parser, test, and fixture references across all queried files.
