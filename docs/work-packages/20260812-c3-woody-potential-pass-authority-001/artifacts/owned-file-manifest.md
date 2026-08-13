# Owned-File Manifest

Status: `RECONCILED / V5 terminal verification PASS / prompts archived`

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

## V5 Continuation

The V3 terminal reconciliation above remains historical. The reopened bounded
V5 write set contains:

- V9 amendments to `SC-VEGETATION-001` and its lifecycle registry;
- this package's V5 intake, selection, operand-lineage, vector, prompt, review,
  gate, verification, disposition, and handoff artifacts;
- immutable V5 definition/fixture/generator bytes and established byte-
identical definition copies;
- an implementation-independent V5 authority verifier and package
  catalog/lifecycle entries. Ordinary Rust fixture consumption is assigned to
  the resumed implementation package because Rust is outside this write set.

Production Rust, Cargo manifests/lockfile, runtime selectors, consumers,
deployment, and publication remain outside this authority package. Exact
terminal reconciliation confirms this list against frozen V5 bytes; it does
not alter the V3 reconciliation above. The V5 package is approximately 1.4 MiB
and contains no nested `target` directory or file larger than 10 MiB.
