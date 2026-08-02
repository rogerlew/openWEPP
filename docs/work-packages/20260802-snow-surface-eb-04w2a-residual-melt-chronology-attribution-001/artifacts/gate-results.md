# Gate Results

Status: `HOLD / EXPECTED SCIENCE-CLOSURE FAILURE`.

Evidence mode: **Ran + Static**.

| Gate | Result | Evidence |
|---|---|---|
| Package tool self-check | PASS | `EB-04W2A self-check: PASS` |
| Package tool compilation | PASS | `.venv/bin/python -m py_compile` with target-only cache |
| Frozen tool identity | PASS | Receipt and current tool SHA-256 both `fa5399db...0ad2` |
| Result-bearing executions | PASS | Eight unique cells; all receipt return codes `0` |
| Retained direct mass closure | PASS | Maximum `2.221e-15 m` against `1e-12 m` |
| Retained Stage-3 energy closure | PASS | Maximum `6.094e-08 J m^-2` against `1e-6 J m^-2` |
| Snowbench SWE closure | **FAIL** | Maximum `0.0708 m`; one lost snowfall event in every lane/model pair |
| Evidence-role separation | PASS | Direct findings admitted; albedo-dependent results explicitly withdrawn |
| Figure inventory and XML parse | PASS | Four SVGs and four same-stem Markdown sidecars |
| Figure visual inspection | PASS WITH WITHDRAWAL NOTICE | All four rendered; invalid harness panels/row carry visible notices |
| Documentation lint | PASS | `markdown-doc`: 29 files, 0 errors, 0 warnings |
| Whitespace | PASS | `git diff --check` |
| Production/protected write set | PASS | Only package tree and three authorized roadmap/catalog files changed |
| Security impact | PASS | No runtime, dependency, network, parser, or secret-handling change |
| Dual review/disposition | PASS | Both reviewers pass truthful `HOLD / PARTIAL ADMISSION`; all findings resolved |
| Dual terminal verification | PASS | Both verifiers pass the final hold record and scoped lifecycle gates |

Rust workspace tests were not selected because W2A made no Rust, contract,
manifest, test, fixture, observation, or production-input change. The static
source diagnosis does not substitute for the EB-04W2B implementation gates.

The failed snowbench closure means acceptance criteria 3 and 10 cannot admit a
scientific harness result even though all processes returned success. Under the
package stop condition, terminal disposition is `HOLD`, not a waived pass.
