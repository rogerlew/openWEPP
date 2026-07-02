# Codex Review - MOFEFID-D01

Outcome: **HOLD as written; ADR-0033 not ratified in this pass.**

The D3 friction kernels are acceptable shadow-only code: the physical-domain
equations match the local R-63 markdown for eqs. (2)-(7), the module is not
wired into any phase span, and the focused/full crate gates are green. The
hold is on D2/D1 governance: one validation-case operand is wrong, the
copyrighted-source dependency is not package-manifested, and ADR-0033 currently
overstates what ratification authorizes before `SC-OFEROUTE-001` exists.

## Evidence Classes

Static:

- Read package, ADR-0033, validation-case artifact, MOFEFID strategy, root/work-package/science-contract/crate/test/fixture AGENTS guidance.
- Inspected `origin/main..HEAD` diff and searched for all `ofe_routing` callers.
- Verified `SC-OFEROUTE-001` is not present in `docs/specifications/science-contracts/contracts/` or the registry.
- Verified `references/copyrighted/**` is intentionally ignored by Git.

Ran:

- `python3 -m json.tool docs/work-packages/20260702-mofefid-d01-ofe-routing-scaffold-001/artifacts/validation-cases.json`
- Local ignored-source spot checks against `/home/workdir/openWEPP/references/copyrighted/Papanicolaou2018.md`,
  `.../Papanicolaou2018-supplemental/.../3.1_Validation_Input.docx`, and
  `.../Figure_4.xlsx`.
- `sha256sum` for the local ignored Papanicolaou paper/markdown/docx/xlsx cache files.
- `cargo fmt --check`
- `cargo test -p openwepp-hillslope-orchestrator ofe_routing::friction -- --nocapture`
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
- `cargo test -p openwepp-hillslope-orchestrator`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Findings

### D01-CX-001 - Accepted, Ratification-Blocking

Static/Ran: `validation-cases.json` records Case 3 as `plot_m: [6.1, 3.6]` at
`docs/work-packages/20260702-mofefid-d01-ofe-routing-scaffold-001/artifacts/validation-cases.json:7`.
The exact source named by the artifact, local ignored
`3.1_Validation_Input.docx`, extracts as `Plot Dimensions : 6.1 m x 1.8 m`
for Case 3. Cases 1/2/4 matched the extracted source on the checked operands.

Why it matters: Case 3 is the vegetation-patchiness / OFE-cascade validation
case. A doubled width/area changes any later conversion between unit-width,
plot, and outlet hydrograph quantities, so D2 cannot be accepted as the
fixture basis for D5/D-val until this is corrected or justified by a second
explicit source.

Required disposition: change the Case 3 fixture to the source value or add a
source-cited explanation for the 3.6 m value and the intended width/area basis.

### D01-CX-002 - Accepted, Ratification-Blocking Until Manifested

Static/Ran: `validation-cases.json:2-3` and `package.md:13-16` cite
`3.1_Validation_Input.docx` and `Figure_4.xlsx` as the source/observed-series
authority. The bibliography also points to
`references/copyrighted/Papanicolaou2018-supplemental/`
(`references/annotated_bibliography.md:815`), but `.gitignore:23-27` makes
`references/copyrighted/**` local-cache only. In the D01 worktree, Git only
tracks `references/copyrighted/.gitkeep`; I found the cited files only in the
ignored local cache under `/home/workdir/openWEPP/references/copyrighted/`.

Why it matters: copyright governance explains why the source-native files are
not committed, but D2 still needs independent provenance. As written, a fresh
checkout of the branch cannot reproduce or audit the extracted validation-case
operands, and the package records no checksums, acquisition/install note, or
package-local manifest for the ignored source files. My local cache hashes were:
`Papanicolaou2018.pdf` `6deea7032a3263607da46dbbe8aec5b995cb9773d632dd411688693d2ba2abf8`,
`Papanicolaou2018.md` `caebad9c359c16ac8d504670494ada3c815d2e066b26e5baf810138e90f5413e`,
`3.1_Validation_Input.docx` `0aee14555a3f5394aef89c9b6623fc13644273a676bb316e76ca5b6e148f9362`,
and `Figure_4.xlsx` `2bf68787de6a715049ee635c154c640214936fd1181d08c8f7da7a34892d2fe8`.

Required disposition: add a package-local provenance manifest or fixture
README that names the ignored source paths, expected hashes, installation
location, and which operands/series are deliberately derived versus merely
referenced. Do not duplicate copyrighted series unless a separate governance
decision permits it.

### D01-CX-003 - Accepted, Ratification-Blocking As Worded

Static: ADR-0033 says ratification authorizes the "solver + cascade stages"
(`docs/decisions/0033-ofe-by-ofe-overland-flow-routing.md:52-53`) while the
decision body only says to author `SC-OFEROUTE-001`
(`docs/decisions/0033-ofe-by-ofe-overland-flow-routing.md:38-39`). The package
similarly says ratification "authorizes the solver stages" while naming
`SC-OFEROUTE-001` as the contract home
(`docs/work-packages/20260702-mofefid-d01-ofe-routing-scaffold-001/package.md:10-12`).
No `SC-OFEROUTE-001` file, registry row, invariant table, guard map, unit
ledger, or contract-derived tests exist yet.

Why it matters: the root and science-contract playbooks require canonical
`SC-*` authority before process-physics implementation. An accepted ADR can
ratify the representation and activation-policy decision, but it cannot stand
in for the process contract that will govern KWE/TVD equations, CFL, domain
guards, per-OFE hydrograph handoff, and conservation stops.

Required disposition: either author and register `SC-OFEROUTE-001` before
D4 solver code, or narrow ADR-0033's ratification wording so it authorizes
only the representation/activation decision and the next contract-first D4
package. After that narrowing, ADR ratification would be reasonable.

## Deferred Candidate

D01-DX-001 - Static, deferred to D4: the public friction helpers use raw `f64`
inputs and some invalid-domain cases canonicalize to `0.0` or `0.33`
(`friction.rs:24-195`, especially `reynolds_number`, `froude_number`,
`vegetation_momentum_absorption`, and `chezy_from_friction`). I do not treat
this as a D3 defect because the functions are currently shadow-only and the
physical-domain formulas match R-63. Before runtime wiring, `SC-OFEROUTE-001`
and the D4 solver boundary should define fail-closed finite/nonnegative/unit
guards so production code does not rely on formula-level silent defaults.

## Accepted Checks

- Static/Ran: R-63 local markdown confirms the implemented forms for skin
  resistance, form resistance, wave resistance including the `Fr < 0.5`
  proportional ramp, vegetation resistance, additive `f_eq`, and Chezy
  conversion. D3 is a faithful physical-domain transcription.
- Static: `rg` found no callers of `ofe_routing` outside the module and tests;
  default hillslope execution is not wired to the new kernels.
- Ran: `cargo test -p openwepp-hillslope-orchestrator` passed 154/154,
  including the six new friction tests.
- Ran: both authority guards passed.

## Ratification Recommendation

Do **not** ratify ADR-0033 as currently written. After CX-001 and CX-002 are
fixed, CX-003 can be resolved either by creating the canonical
`SC-OFEROUTE-001` authority now or by narrowing the ADR acceptance to the
representation/activation decision only, with D4 explicitly beginning by
authoring the contract before solver implementation.
