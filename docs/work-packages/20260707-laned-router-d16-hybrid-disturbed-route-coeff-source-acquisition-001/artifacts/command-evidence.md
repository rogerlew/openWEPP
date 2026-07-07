# Command Evidence

Status: EXECUTED. Evidence mode: Ran.

## Scaffold-Time Checks

Ran during scaffold:

```text
git diff --check
exit 0

markdown-doc lint --path docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001 --format plain
✅ 15 files validated, 0 errors, 0 warnings

markdown-doc lint --path docs/work-packages/README.md --format plain
✅ 1 files validated, 0 errors, 0 warnings
```

## Execution Checks

WEPPpy:

```text
cd /home/workdir/wepppy
wctl run-pytest tests/test_managements_module.py -q
15 passed, 2 warnings

cd /home/workdir/wepppy
wctl run-pytest tests/disturbed/test_route_coefficients.py -q
6 passed, 2 warnings

cd /home/workdir/wepppy
wctl run-pytest tests/disturbed/test_disturbed_matrix.py tests/disturbed/test_route_coefficients.py tests/test_2_validate_managements.py tests/test_managements_module.py -q
114 passed, 2 warnings

wctl doc-lint --path wepppy/nodb/mods/disturbed/README.md
1 file validated, 0 errors, 0 warnings

wctl doc-lint --path wepppy/nodb/mods/disturbed/ENDUSER.md
1 file validated, 0 errors, 0 warnings

wctl doc-lint --path docs/adrs/ADR-0014-disturbed-openwepp-route-coefficients.md
1 file validated, 0 errors, 0 warnings

git -C /home/workdir/wepppy diff --check
exit 0
```

openWEPP:

```text
cd /home/workdir/openWEPP
cargo fmt --check
exit 0

cargo clippy --workspace --all-targets -- -D warnings
exit 0

cargo nextest run --workspace --profile full
1439 tests run: 1439 passed (5 slow), 4 skipped

cargo deny check
advisories ok, bans ok, licenses ok, sources ok

cargo test -p openwepp-hillslope-orchestrator disturbed_native_route_coefficients -- --nocapture
1 passed

cargo test --test infile_management_parser_contract native_ow_lanuse_1_accepts_native_cropland_sentinel_with_routing_coefficients -- --nocapture
1 passed

cargo test --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients -- --nocapture
1 passed

git diff --check
exit 0

markdown-doc lint --path docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001 --format plain
18 files validated, 0 errors, 0 warnings

markdown-doc lint --path docs/work-packages/README.md --format plain
1 file validated, 0 errors, 0 warnings

markdown-doc lint --path docs/contracts/openwepp-management-lanuse-authority-contract.md --format plain
1 file validated, 0 errors, 0 warnings
```
