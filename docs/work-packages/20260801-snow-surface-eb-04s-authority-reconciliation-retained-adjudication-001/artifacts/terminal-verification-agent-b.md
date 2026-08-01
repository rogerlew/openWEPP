# Independent Terminal Verification B

Evidence mode: `Static + Ran` (read-only retained analysis; no model
subprocess).

Verifier: independent terminal verifier B.

Scope: accepted terminal-review findings, regenerated retained adjudication,
frozen-input and unit predicates, terminal validation, decision reconstruction,
claim limits, and exact write set. Verifier A's artifact was not read.

Decision: `PASS_WITH_NOTES`.

All accepted science/admissibility findings are closed. A fresh read-only call
to the repaired `analyze()` function completed over the full retained
population and independently reproduced
`CLOSE_NONPROMOTION_EMPIRICAL_RULE`. The notes below are documentation-evidence
freshness items and do not change the scientific disposition.

## Finding Verification

### `TA-H1` / `TB-H1` — CLOSED

Before any observation is loaded, `tools/adjudicate_retained.py` now compares
the current EB-04R tool and protocol, EB-04/EB-04E dependencies, frozen receipt,
`crates` and `tests` trees, 12 fixture trees, 12 observation files, observation
roles and filters, target and non-target selectors, and eight decision
dependencies with `execution-attempt.json`. Any mismatch raises before scoring.

The fresh analysis passed all nine grouped identity checks plus population and
selector bindings. It then rechecked 48/48 cell records and 288 retained file
identities. The EB-04R package and retained-output hashes remained identical
before and after analysis. This closes the observation/rubric/consumer identity
gap without a simulation rerun.

### `TA-M1` / `TB-M1` — CLOSED

The repaired population gate explicitly applies EB-04R's prospectively frozen
`1e-12 kg m^-2` daily vapor-aggregation tolerance in addition to canonical
version-6 predicates. The fresh analysis independently returned:

```text
maximum vapor-aggregation residual = 7.993605777301127e-15 kg m^-2
7.993605777301127e-15 <= 1e-12: PASS
```

Observation access remains downstream of that conjunction. The distinct
vapor-to-sublimation transfer tolerance remains `1e-6 kg m^-2`, with reported
maximum `8.109983287707401e-8 kg m^-2`. No cross-predicate substitution was
found.

### `TB-M2` — CLOSED

`authority_reconciliation.py --verify-seal` passes at the terminal state. It
checks the immutable authority-freeze hash, current version-6 contract hash,
dual-verified seal status, and Phase-B authorization without trying to recreate
the version-5 prospective derivation against amended contract text.
`adjudicate_retained.py --self-check` also passes. Both tools parse successfully.
The gate table now truthfully distinguishes the pre-freeze Phase-A self-check
from terminal seal verification.

### `TA-L1` / `TB-L1` — CLOSED WITH NOTE

The extraneous EB-04E `package.md` row was removed. The input manifest, freeze
receipt, and authority tool consistently bind exactly four Phase-A inputs:
pre-amendment `SC-SNOWENERGY-001`, unit governance, the unit-helper source, and
the EB-04E prospective protocol. The reading map contains those four sources
and no fifth authority source.

Note: the map labels unit governance `Conditional` rather than `Phase A
whitelist`. This does not alter the enforced whitelist, but changing that tier
label would make the four-file boundary visually exact.

## Scientific Decision Reconstruction

The fresh full retained analysis returned:

| Quantity | B | LS |
|---|---:|---:|
| Forcing-robust ordinal score | 177 | 180 |
| Forcing-robust failure count | 16 | 16 |
| Complete independent-validation lanes | 10 | 10 |
| Available robust rubric cells | 90 | 90 |

The independent reducer passes and agrees on aggregates, protected groups, new
failures, compensation findings, and all eight decision predicates. LS raises
the score, but `16 < 16` is false. Therefore the complete prospectively frozen
promotion rule fails, the stop-loss is `true`, and another calibration or
factorial round remains unauthorized. Warm-maritime conifer transfer remains
withheld and both mechanisms remain default-off. This is empirical
nonpromotion in the available population, not evidence that the implemented
process physics is invalid.

## Regression And Validation Checks

- Ran full read-only `analyze()`: physical/provenance population `PASS`, frozen
  bindings `PASS`, observations loaded only afterward, independent decision
  reconstruction `PASS`, immutable input check `PASS`, model rerun `false`.
- Ran terminal authority-seal verification: `PASS`.
- Ran retained-adjudicator self-check and Python AST parsing: `PASS`.
- Ran strict binding-exposure validation for `SC-SNOWENERGY-001`: six rows,
  `PASS`.
- Ran scoped SC unit-compliance validation: zero findings.
- Ran Markdown lint over the current package (23 Markdown files before this
  verification artifact) and four shared documents: zero findings.
- Ran `git diff --check`: `PASS`.
- `git status --porcelain -- crates tests` is empty. No production, test,
  fixture, observation, prior-package, or retained-output edit was found.
- Roadmap/catalog claims remain bounded to nonpromotion, default-off mechanisms,
  withheld warm-maritime conifer transfer, and EB-05 assurance next. EB-04R
  remains an unchanged historical HOLD.

## Open Findings / Notes

No open science, authority, provenance, physical-closure, observation,
decision, security, or write-set finding remains.

Two low documentation notes remain for final evidence refresh:

1. `artifacts/gate-results.md` and `markdown-lint-package.json` still say 18
   package Markdown files, while the independently linted terminal package had
   23 before this artifact. Refresh the count and lint receipt after both
   verifier artifacts exist.
2. Relabel `docs/specifications/unit-governance.md` as `Phase A whitelist` in
   `required-reading-map.md` for exact visual agreement with the enforced
   four-file authority boundary.

These notes do not alter any accepted finding's substantive closure or the
nonpromotion outcome. Final disposition may proceed after the ordinary
post-verifier Markdown evidence refresh.
