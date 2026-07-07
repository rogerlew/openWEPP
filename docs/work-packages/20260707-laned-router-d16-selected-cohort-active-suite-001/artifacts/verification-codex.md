# Verification

Status: COMPLETE. Evidence mode: Static + Ran.

## Checks

| Check | Result | Evidence |
|---|---:|---|
| Materialization manifest has four selected members | PASS | `selected-cohort-materialization.json` lists `h2637`, `mn_corn_h4`, `n_idaho_forest_h1`, `wa_cascades_forest_h1`. |
| Runfiles separate plain/hybrid outputs | PASS | `*.plain.run.toml` writes to `output-plain`; `*.hybrid.run.toml` writes to `output-hybrid`. |
| H2637 active plain executed | PASS | `active-suite-command-log.json`, `h2637-plain.time.log`. |
| H2637 active hybrid executed with implicit stepping | PASS | `active-suite-command-log.json` records `hybrid_implicit_flag_ok = true`; manifest records `hybrid_implicit_stepping = true`. |
| First external selected member active plain executed | FAIL | `mn_corn_h4` fails on Rev-21 `canhgt` guard before manifest publication. |
| Suite completion claim avoided | PASS | Package, README, and final disposition use `EXECUTED-HOLD-ACTIVE-RUN`, not complete. |

## Residual Risk

The package does not prove whether the two forest external members would pass
both active modes in the formal runner because execution stops on the first
selected active-run failure. That is acceptable for this hold disposition: the
selected cohort cannot close until `mn_corn_h4` active plain runs.
