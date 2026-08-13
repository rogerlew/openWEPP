# Owned-File Manifest

Status: `RECONCILED / terminal verification PASS / prompt archived`

Evidence mode: `Static + Ran`

Base: `4f5bb1c599a683b63be56ecd9e7296f8faf01ed0`

Every changed path belongs to one of these declared roots:

- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` —
  V7 canonical authority;
- `docs/specifications/science-contracts/index.md` — approved/active registry;
- `docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts/openwepp_c3_woody_v3_definition.json`
  — byte-identical successor definition copy;
- this complete package tree — plan, prompt, authority selections, oracle,
  vectors, definition, reviews, gate evidence, verification placeholders, and
  handoff;
- `docs/work-packages/README.md` — package catalog lifecycle;
- `tests/integration/vegetation_boundary_authority_contract.rs` — V3 authority,
  digest, independent-vector, guard, and poison tests.

No production crate, Cargo manifest/lockfile, runtime selector, consumer,
deployment, or publication path is changed. The active kickoff prompt remains
in place until both terminal verifiers pass.

The package measured 844 KiB before verifier artifacts and remains under 1 MiB;
it contains no nested `target` directory or file larger than 10 MiB.
Empty/superseded launcher-wrapper directories were moved intact to
`/home/workdir/openwepp-task-trash/v3-nextest-wrapper-misfires-20260812`; all
substantive failed, interrupted, and successful gate logs remain in-package.
