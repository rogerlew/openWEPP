# Verification Agent B

Status: `final completed-tree verification after LF normalization / PASS`.

Evidence class: `Ran + Static`.

Verifier role: independent terminal Verification B for the final completed
EB-02 contract/evidence tree.

## Exact Candidate

Git base:
`0c8bb45890d32e5ff096fceed641ac401e06b69c`.

The deterministic candidate-tree digest includes the five owned repository
documents, the canonical contract, and every package file except the two
verifier-owned artifacts:

```text
candidate files: 35
candidate tree SHA-256:
f29f33f6a3945ece5072d3f116a54c39f6b4aa3dc01f1ff699ed6503baa7f689
```

Selected bindings:

| Artifact | SHA-256 |
|---|---|
| `SC-SNOWENERGY-001.md` | `6af03f4f58db9fbd032e7577ed8a4d298482dfaf8c8c0529d9db7fbd2a0f44c9` |
| `package.md` | `a86dd70d9b9182daabae9f27d47db0380578a281c0d32e30df9489d09d890f46` |
| `tools/execute.py` | `107e58cfb91ed1892131346c9be9d9d751e5adc401a117e9392727bbc345f8be` |
| `analytical-test-vectors.csv` | `abf0bd3265f54c32a9f2bff37a8c1acac09afab6e57472cd7a3de1621b59a742` |
| `operand-lineage.csv` | `d243b5909895e91ce98e9be016242cef8e3d919fd8f0460cdc1a9d40ed6f797f` |
| `calibration-readiness-matrix.md` | `f507fb51c07bb6ac80a907f4a480639187392ce2bc7b2db1448b7f9f1438df74` |
| Review A | `9d9f9eef817f4a86057e147f1bf24f4fca0e9cb4f42aba2d0ab20c1fa5e9c72e` |
| Review B | `74593bdb8792efd473874a401439890c87ed256f39f5e3a5618f4a8f60bb33cb` |
| review `disposition.md` | `1f93f32b201996318352063ae506c99229aa43c2148a6e04e092ddb4a210b8e7` |
| final `final-disposition.md` | `25fd0aa38166b435078206c1b5c2ff898c92652d39f663f6eff27dc3a84050ed` |
| `gate-evidence.md` | `5fe5b9412fcf67aeb816bbbfe1e1fd25194dc43046bd3963fd964f17dd6c0e5c` |
| `line-count-governance.md` | `31d17f560223de0007346391fdfd5559a447dc76585c9f852a95e02055d79b2f` |
| `required-reading-map.md` | `642efeacb929a6546d775f5ec9ed15a7706ea5e8e30adf82294ef52846e3f5e6` |
| archived `001-kickoff.md` | `6362158566f94944f7533fdc96997bfec6bedbc71acfab0adcca576c06dfa504` |
| prompt lifecycle `README.md` | `31898d037fc3569a49fac5582fd3051c57abd83bb17b72b860bbf38562ca6044` |
| root roadmap | `332ce77191194b62f667a69514a44ae8e552b587c2099cbb101094e2f5cb2649` |
| campaign roadmap | `9d3f96ee1621568f27eba84b9350a7caa9946f2c799966af2ff88790e8e52d80` |
| work-package catalog | `00d3e3377f183847f8f3767bdd4e7c698be9b84928587709a9bb4c8e81d899e8` |
| science-contract registry | `ddbcf5afaa5673b498353568a4ccb7921d40d3dc6501d4c166a22b94432b11ef` |

## Ran Checks

I ran:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python \
  /tmp/<isolated-eb02>/tools/execute.py
cmp <repository generated artifacts> <isolated generated artifacts>
python3 tools/check_sc_binding_exposure.py --strict \
  docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
bash tools/release/check_sc_unit_compliance.sh --path \
  docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
bash tools/release/check_science_contract_admission.sh \
  --base-ref HEAD --head-ref HEAD
markdown-doc lint/validate --path <contract>
markdown-doc lint/validate --path <package>
markdown-doc lint/validate --path <each owned roadmap/catalog/registry file>
git diff --check
```

Results:

- isolated execution: `PASS`, 38 vectors and two SVG figures;
- generated CSV and both SVGs: byte-identical to the candidate;
- analytical CSV line endings: `PASS`, exactly 39 LF terminators and no CR
  byte;
- analytical statuses: 37 `PASS`, exactly one prospectively declared
  `missing_thermal_provider` `HOLD`, zero `FAIL`;
- strict Binding Exposure Index: `PASS`, one consolidated row;
- scoped science-contract unit compliance: `PASS`, no findings;
- working-tree registry inventory: `PASS`, 40 admitted contracts, sorted and
  registry/frontmatter consistent; `HEAD..HEAD` is a static inventory check,
  not a changed-commit admission claim;
- Markdown lint and validation: `PASS`, zero errors or warnings;
- CSV shape: `PASS`, 38 analytical and 17 operand-lineage rows, rectangular
  and nonblank;
- SVG/accessibility: `PASS`, both XML-parse and have `role="img"`,
  `aria-labelledby`, nonempty title/description, and same-stem Markdown
  sidecars;
- diff and full owned-tree whitespace checks: `PASS`.

## Profile, Schema, And Review Closure

Static inspection confirms the contract contains the required ordered profile
surfaces, including a numbered evaluation sequence, complete branch/guard
classes, 14 authority/evidence/guard/failure-mapped invariants, producer and
consumer obligations, owner-bearing aliases, constants, per-symbol unit
governance, tolerances, readiness status, test obligations, a consolidated
Binding Exposure Index, gap ownership, and change history.

Calibration posture is truthful:

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

The readiness matrix dispositions all ten required surfaces and contains no
current-scope `BLOCKED` row. The fixed equations introduce no fitted user
parameter. The package-only `R_a,min=1e-9` demonstration value is explicitly
named `ASSUMED_FOR_EXECUTION`, passed rather than defaulted, and exercised only
on a threshold-insensitive zero-radiation case; the runtime value remains an
EB-03 prerequisite.

All ten independent findings (`EB02-RA-01..05` and `EB02-B-001..005`) are
present in the disposition table, are `accepted`, and have corrected-tree
evidence. Both refreshed reviews pass and bind this contract, executor,
vectors, disposition, and the admitted pre-closure candidate. Their reviewed
science/evidence hashes are unchanged in this completed tree.

## Reproducibility And Anti-Tautology

The generator stores immutable numeric expectations and compares independently
evaluated results using the contract tolerances. I separately reconstructed
sky view, precipitable water, clear/all-sky atmospheric longwave,
complementary sub-canopy longwave, outgoing snow emission, and net-longwave
sign from the declared operands; all reconstructed values match the expected
and observed CSV fields. Expected values are not regenerated from observed
values. Guard cases execute invalid/non-finite inputs, atmospheric
out-of-authority state, polar night, and the missing-provider hold.

The evidence remains analytical and package-local. No production or test Rust
file imports this package, and no runtime, selector, schema, output, or
consumer path changed. Producer-only evidence is not presented as runtime
activation or energy-closure evidence.

## Lifecycle, Scope, And Governance

The exact changed-file inventory contains only the declared six owned
surfaces/groups and no `.rs` file. Roadmap, campaign roadmap, catalog,
registry, package, and final disposition consistently report:

```text
COMPLETE / CONTRACT PASS / RUNTIME HOLD
```

The core reading set is exactly `421807` bytes, truthfully classified `WARN`
above 400000 and below the justification threshold. Informational counts are
current: contract 488 lines, package 286 lines, executor 598 lines. No changed
`.rs` file exists, so the Rust line-count thresholds have no target.

The final closure transition is exactly the mechanical transition admitted by
the initial terminal pass. It changed package/roadmap/catalog/final-disposition
status text, added the terminal gate row, refreshed the reading count, and
moved the kickoff prompt from `active/` to `archived/`. No active prompt
remains. The canonical contract, executor, vectors, review artifacts, finding
disposition, operand lineage, figures, and runtime hold retain their
initial-pass hashes. No equation, evidence result, finding decision, write-set,
or runtime behavior changed.

The final pre-commit normalization adds an explicit CSV
`lineterminator="\n"` and converts the vector artifact from CRLF to LF. Isolated
regeneration is byte-identical to the normalized artifact. All 38 case IDs,
inputs, expected/observed values, units, and statuses remain identical: 37
`PASS` and the one declared runtime-provider `HOLD`. Both refreshed reviewers
independently bind the normalized executor/vector hashes and retain `PASS`.

The EB-03 runtime hold remains mandatory. Production activation still requires
one coherent hourly `T_s`/`T_c`/cold-content provider, authoritative
polar-night behavior, a bound `R_a,min`, common `B/L/S/LS` consumer use,
exact-one sublimation composition, and independent energy closure.

## Verdict

`PASS`.

No blocker remains for EB-02's completed contract/evidence increment. This
verdict does not lift the runtime hold and does not claim production
implementation, real-consumer closure, empirical calibration, independent
validation, or transferability.
