# Verification Agent A

Status: `final pre-commit LF-normalized completed-tree verification / PASS`.

Evidence class: `Static + Ran independent reconstruction and isolated
regeneration`.

Verifier role: independent terminal Verification A for the exact completed
tree after the admitted closure-only lifecycle transition and final mechanical
LF normalization. The two verifier-owned artifacts are excluded from the
hash-bound subject because writing either artifact necessarily changes its own
hash.

## Exact Verified Candidate

Git base:
`0c8bb45890d32e5ff096fceed641ac401e06b69c`.

| Load-bearing artifact | SHA-256 |
|---|---|
| `SC-SNOWENERGY-001.md` | `6af03f4f58db9fbd032e7577ed8a4d298482dfaf8c8c0529d9db7fbd2a0f44c9` |
| `package.md` | `a86dd70d9b9182daabae9f27d47db0380578a281c0d32e30df9489d09d890f46` |
| `tools/execute.py` | `107e58cfb91ed1892131346c9be9d9d751e5adc401a117e9392727bbc345f8be` |
| `analytical-test-vectors.csv` | `abf0bd3265f54c32a9f2bff37a8c1acac09afab6e57472cd7a3de1621b59a742` |
| `operand-lineage.csv` | `d243b5909895e91ce98e9be016242cef8e3d919fd8f0460cdc1a9d40ed6f797f` |
| `canopy-sky-view-derivation.md` | `f15a8e683916002af1bcc6585f13613ceff34aa409db82cccc57785fb3252bf6` |
| `source-and-state-reconciliation.md` | `168ce7dae58dfaa82c4bc9519d16661fab7405b7c0e61224feacd6190c4a44f1` |
| `calibration-readiness-matrix.md` | `f507fb51c07bb6ac80a907f4a480639187392ce2bc7b2db1448b7f9f1438df74` |
| `review_agent_a.md` | `9d9f9eef817f4a86057e147f1bf24f4fca0e9cb4f42aba2d0ab20c1fa5e9c72e` |
| `review_agent_b.md` | `74593bdb8792efd473874a401439890c87ed256f39f5e3a5618f4a8f60bb33cb` |
| `disposition.md` | `1f93f32b201996318352063ae506c99229aa43c2148a6e04e092ddb4a210b8e7` |
| `gate-evidence.md` | `5fe5b9412fcf67aeb816bbbfe1e1fd25194dc43046bd3963fd964f17dd6c0e5c` |
| `exact-diff-reconciliation.md` | `3efe6a8c876c0435b2f2a7673b8933deb12594676d704a458475aeaa879094b5` |
| `line-count-governance.md` | `31d17f560223de0007346391fdfd5559a447dc76585c9f852a95e02055d79b2f` |
| final `final-disposition.md` | `25fd0aa38166b435078206c1b5c2ff898c92652d39f663f6eff27dc3a84050ed` |
| required-reading map | `642efeacb929a6546d775f5ec9ed15a7706ea5e8e30adf82294ef52846e3f5e6` |
| prompt README | `31898d037fc3569a49fac5582fd3051c57abd83bb17b72b860bbf38562ca6044` |
| archived kickoff | `6362158566f94944f7533fdc96997bfec6bedbc71acfab0adcca576c06dfa504` |
| campaign roadmap | `9d3f96ee1621568f27eba84b9350a7caa9946f2c799966af2ff88790e8e52d80` |
| root roadmap | `332ce77191194b62f667a69514a44ae8e552b587c2099cbb101094e2f5cb2649` |
| work-package catalog | `00d3e3377f183847f8f3767bdd4e7c698be9b84928587709a9bb4c8e81d899e8` |
| science-contract registry | `ddbcf5afaa5673b498353568a4ccb7921d40d3dc6501d4c166a22b94432b11ef` |

## Independent Science Verification

Result: `PASS`.

- Reconstructed the FSM2 elimination independently:
  `P_0=1-C=exp(-k_ext VAI_eff)` and
  `f_sky=exp(-1.6 k_ext VAI_eff)` imply
  `f_sky=(1-C)^1.6`. The vectors distinguish this hemispherical
  transformation from the rejected direct alias `1-C`.
- Confirmed that effective daily cover already carries the seasonal foliar
  trajectory and structural-cover floor. The contract does not add LAI again,
  does not treat structural-cover fraction as stem-area index, and does not
  insert height into a homogeneous Beer-law equation without authority.
- Independently reconstructed the Dilley-O'Brien clear-sky equation,
  Unsworth-Monteith cloud mixture, complementary sky/canopy exchange,
  Stefan-Boltzmann snow emission, and positive-toward-snow net-longwave sign.
  Immutable expected fluxes agree within `1e-6 W m^-2`; fraction identities
  agree within `1e-9`.
- Confirmed hourly atmospheric evaluation with held daily vapor/cloud state.
  The independent two-hour reconstruction reproduces the nonzero
  `2.411912116322 W m^-2` bias from substituting daily-mean temperature.
- Confirmed variable atmospheric effective emissivity and exactly-unity
  effective canopy/snow emissivity. Derived atmospheric emissivity outside
  `[0,1]` fails without clamping.
- Confirmed explicit validity limits: equivalent horizontally homogeneous
  one-layer canopy, random orientation, isotropic diffuse sky, and no
  directional crown, gap/edge/trunk, terrain-horizon, or anisotropic-sky
  claim.

## Guards, Vectors, And Reproducibility

Result: `PASS`.

Ran the package executor from an isolated copy:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python \
  /tmp/eb02-verify-a-lf.qhAY1r/pkg/tools/execute.py
```

Result:

```text
PASS: regenerated 38 analytical vectors and 2 accessible SVG figures
```

The regenerated CSV and both SVGs are byte-identical to the candidate:

- vectors:
  `abf0bd3265f54c32a9f2bff37a8c1acac09afab6e57472cd7a3de1621b59a742`;
- sky-view SVG:
  `6a5c977e06a7702f3427c96920347e71eb8f2a595195c507fa3771a5b178f6ef`;
- longwave SVG:
  `db08aed8b689e92e330674654c78b076e6733a4ecc9e824210468722e3928c01`.

The CSV has 38 unique cases: 37 `PASS`, exactly one prospectively declared
`missing_thermal_provider` `HOLD`, no `FAIL`, and no unexpected hold. Invalid
and non-finite cover, temperature, vapor, radiation, cloud, and flux cases
execute. The polar-night case explicitly labels its threshold
`ASSUMED_FOR_EXECUTION`; the contract leaves authoritative `R_a,min` binding
to EB-03.

The normalized CSV contains 39 LF terminators and no CRLF sequence. Replacing
each LF with CRLF in memory reproduces the previously verified byte hash
`1b5a8efa80c24a69739e7286d12ef8a2925e0900e17be6997eeefd46e17f1860`.
Parsing the LF and reconstructed CRLF forms yields 38 rows identical
field-for-field. The explicit `lineterminator="\n"` changes portability only;
it changes no case, numeric value, status, tolerance, guard, or oracle.

Both SVGs parse as XML, contain `role="img"`, `<title>`, and `<desc>`, and have
same-stem Markdown sidecars. The sidecars distinguish deterministic analytical
illustrations from observations, forecasts, calibration, or runtime evidence.

## Review, Hold, And Lifecycle Verification

Result: `PASS`.

- Exact-tree Review A and Review B both refresh to `PASS` for the completed,
  LF-normalized candidate. All ten findings are `accepted`, corrected, and
  re-verified; none is rejected, deferred, or left open.
- The runtime hold is legitimate and remains mandatory. EB-02 creates
  equation/interface authority only and changes no production consumer.
  EB-03 owns coherent `T_s`, `T_c`, cold content, polar-night cloud behavior,
  numeric `R_a,min`, and common `B/L/S/LS` consumer closure.
- Producer-only analytical evidence is not presented as runtime activation,
  empirical calibration, independent validation, or transferability proof.
- The final package, campaign roadmap, root roadmap, catalog, disposition, and
  gate ledger consistently state `COMPLETE / CONTRACT PASS / RUNTIME HOLD`.
  The completed kickoff exists only under `prompts/archived/`, the prompt
  README says no active prompt remains, and the active prompt directory is
  empty.
- The closure transition changed only admitted lifecycle/status records,
  archived the prompt, added the terminal-verification gate row, and refreshed
  the required-reading byte count. The later pre-commit normalization changed
  only the package-local writer's explicit LF terminator, the generated CSV
  line endings, the executor line count, and the two review records that prove
  that mechanical change. Contract, operand lineage, science reconciliation,
  derivation, figures, finding dispositions, and runtime hold retain their
  prior hashes and meanings.
- Calibration fields are orthogonal and truthful:
  `science_implementation_status=NOT_IMPLEMENTED`,
  `calibration_evidence_status=NOT_APPLICABLE`, and
  `identifiability_status=NOT_APPLICABLE`.

## Gates And Write Set

Result: `PASS`.

Ran:

```text
python3 tools/check_sc_binding_exposure.py --strict \
  docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
bash tools/release/check_sc_unit_compliance.sh \
  --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
bash tools/release/check_science_contract_admission.sh \
  --base-ref HEAD --head-ref HEAD
markdown-doc lint/validate on the contract, package, both roadmaps,
  work-package catalog, and science-contract registry
git diff --cached --check
git diff --check
```

Results: strict Binding Exposure `PASS`; unit compliance `PASS`; registry
inventory `A0_ADMITTED` with 40 contracts; Markdown lint/validation zero
errors and warnings; staged and unstaged diff hygiene `PASS`. Independent
scans also pass Python AST syntax, rectangular nonblank CSV rows, SVG/sidecar
requirements, and trailing whitespace over the complete owned tree.

`git status --porcelain=v1 -uall` contains only the five exact documentation
and registry files plus the new canonical contract and complete EB-02 package
tree authorized by `package.md`. There is no `.rs`, production runtime, test,
selector, schema, parser, fixture, runfile, output, dependency, or unrelated
change. Rust, comparator, runtime-consumer, and conservation-runtime gates are
therefore legitimately not applicable to this contract-only increment.

## Verdict

`PASS`.

No blocker remains in the exact LF-normalized completed tree. Dual review,
finding disposition, dual initial terminal verification, final lifecycle
records, and this final pre-commit refresh are consistent. The EB-03
production runtime hold remains intact.
