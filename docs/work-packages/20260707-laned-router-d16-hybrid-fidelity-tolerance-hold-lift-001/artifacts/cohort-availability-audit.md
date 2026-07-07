# Cohort Availability Audit

Status: EXECUTED-HOLD-COHORT-AUTHORITY. Evidence mode: Static + Ran.

## OWCMP Manifests

Ran `tools/owcmp/owcmp env --manifest` for:

- `tools/owcmp/suites/minnesota-corn-ksflag1.json`
- `tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json`
- `tools/owcmp/suites/wa-cascades-mofe-ksflag0.json`

Result: all env checks passed. The run roots and existing WEPP output surfaces
exist.

Ran `tools/owcmp/owcmp manifest run --manifest` for each manifest.

Result: all three explicitly report that `cohort-inventory` manifests are
preflight declarations and are not runnable comparison pairs. Evidence:
`artifacts/owcmp-manifest-run-preflight.log`.

## Repo-Local Runfiles

`artifacts/repo-runfile-inventory.txt` contains four repo-local hillslope
runfiles:

- `tests/fixtures/dff_ws1_native_forest/hjandrews_conifer_forest/p2.run.toml`
- `tests/fixtures/disturbed_burn/forest_high_severity_clay_loam/p4.run.toml`
- `tests/fixtures/disturbed_burn/forest_high_severity_loam/p313.run.toml`
- `tests/fixtures/laned_shadow_h2637/p2637.run.toml`

## Routing-Coefficient Authority Search

Ran:

```bash
rg -n "routing_coefficients" tests/fixtures \
  /wc1/runs/un/unpalatable-rind/wepp/runs \
  /wc1/runs/ar/arboreal-dendrite/wepp/runs \
  /wc1/runs/al/algebraic-radium/wepp/runs -g '*.man'
```

Result: zero matches. Evidence:
`artifacts/routing-coefficients-search.txt` (empty).

## Active Preflight

Copied each repo-local runfile fixture into package scratch space and ran:

```bash
OPENWEPP_LANED_ACTIVE=1 target/release/openwepp-cli-hill \
  --run-dir <scratch> --run-file <runfile> --output-dir <scratch>/output
```

Results:

| Fixture | Exit | Result |
|---|---:|---|
| `dff_ws1` | `1` | fail-closed: missing/inconsistent `route_*` authority symbols. |
| `disturbed_clay` | `1` | fail-closed before active route audit: non-finite climate field `tdpt=NaN`. |
| `disturbed_loam` | `1` | fail-closed: missing/inconsistent `route_*` authority symbols. |
| `repo_h2637` | `1` | fail-closed: missing/inconsistent `route_*` authority symbols. |

Evidence logs: `artifacts/active-preflight/*.log`.

## Audit Disposition

The required active-runnable cohort does not exist in the current repo/session.
The available owcmp cohorts prove path inventory only; they do not provide an
active plain-vs-hybrid openWEPP comparison pair, and their management files do
not carry the active Lane-D routing coefficient extension.

Default promotion cannot be ratified in this package without creating a
cohort-construction/authority package first.
