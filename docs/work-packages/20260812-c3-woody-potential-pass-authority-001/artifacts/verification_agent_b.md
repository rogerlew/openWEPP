# Verification Agent B

Status: `PASS / no unresolved material finding`

Evidence mode: `Static + Ran`

Verification target: exact worktree bytes relative to base
`4f5bb1c599a683b63be56ecd9e7296f8faf01ed0` on 2026-08-12. This verifier
independently read the repository, science-contract, standards, and
work-package instructions and did not rely on the producer's conclusion.

## Static Verification

### Canonical V3 Authority

`SC-VEGETATION-001@7` is `approved/active` and canonically binds all seven
required authority families:

1. Mixed leaf/stem radiation transports over conditional `L+S`, applies
   clumping once as `K_eff=Omega*K`, derives actual sunlit leaf area, preserves
   whole-column upward diffuse coupling, and returns absorption to leaf-sun,
   leaf-shade, and stem owners by area-times-absorptivity without admitting
   stem PAR.
2. Neutral canopy-surface wind is derived from reference wind as `u_star`; the
   distinct semantic leaf, wet, and stem wind operands equal `u_star` while
   their conductances retain distinct characteristic dimensions.
3. The root-to-stem hydraulic path and gravitational head are exactly
   vegetation height and `1000*height_m` millimetres water, with one common
   root node and explicit layer-local soil/root conductance operands.
4. The V3 occupancy state contains exactly the 15 lexicographically bound
   fields, including scalar `root_node_potential_mm`. V2 migration accepts
   only a nonempty bitwise-identical root-potential vector and otherwise
   reports every unresolved occupancy field without averaging or synthesis.
5. Class `beta=1` is internal maximum-demand evaluation only. Accepted Stage A
   is owner-uncapped but hydraulically coupled through a determined
   six-unknown/six-residual system with distinct sun/shade beta factors, both
   class loss and flux equalities, and both downstream continuity equalities.
   Persisted `beta_hyd` is only the exact Emax-weighted diagnostic/warm start.
6. Atkin leaf N/T10 is the sole `Rd25` source. The admitted peaked Rd response
   is subtracted once from gross assimilation and the identical class-scaled
   amount is debited once from the carbon ledger; `rd_leaf_n_rate`, silent
   clamping, and a second leaf-maintenance debit are prohibited.
7. Numerical failures have deterministic
   `identity/schema -> domain -> bracket -> singular -> iteration` precedence
   and typed, finite, operation-applicable diagnostics for every required solve
   identity, with no usable last iterate or partial candidate.

The canonical model definition and its historical copy are byte-identical at
SHA-256
`7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`.
They bind the independent fixture at
`1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109`
and generator at
`7b137c1aa9ed0912caf4d14c779eca1819014b4217156d36f98619f06daabd1a`.

### Independent Vectors And Poisons

The Python oracle does not call Rust or obtain expected values from the Rust
implementation. Its committed fixture covers whole-column VIS/NIR direct and
diffuse radiation, mixed and degenerate optical reductions, neutral
aerodynamics, complete uncapped E11--E15 coupling, alternate warm starts,
dry/frozen and redistribution branches, exact root migration, respiration,
all required solve failures, and precedence. The fixture contains exactly 40
named poisons; all have `executed=true`, quantitative alternatives are
discriminating, and typed alternatives identify `owning_validator`.

The Rust authority test independently reconstructs radiation owner closure,
friction-velocity/conductance identities, imported energy constants,
biochemical identities, primitive E14 layer conductances and area, all six
coupled residuals, stand-ground requests, exact-once respiration, migration,
finite/applicable failure diagnostics, the full precedence order, and the
complete 40-poison inventory.

### Protected Bytes, Review, And Scope

- V1 remains
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.
- Every V2 definition remains
  `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`.
- Both science reviews preserve their earlier NO-GO history and end in GO on
  the current exact identities. Every A/B finding is accepted, corrected, and
  finally resolved; none is deferred, rejected, or undispositioned.
- The exact terminal diff contains only the vegetation contract/index, V3
  authority definition/fixtures/tests, package/catalog evidence, and package
  logs. No production crate, Cargo manifest/lockfile, runtime selector,
  consumer, hydrology/BGC owner, deployment, publication, or activation path
  changed. The existing public transaction therefore remains fail-closed.
- The corrected terminal reconciliation records the measured 844 KiB package
  size before verifier artifacts and the package remains under 1 MiB, with no
  nested `target` tree or file larger than 10 MiB.
- The kickoff prompt remains in `prompts/active/`, as required until both
  terminal verifiers pass.

## Heavy Evidence And Reuse Audit

The comparator evidence contains an uninterrupted, `/home`-backed
`cargo nextest run --workspace --profile full` with exit code zero and summary
`2481 tests run: 2481 passed (51 slow), 33 skipped` in 3,318.773 seconds;
wrapper duration was 3,329 seconds and its unique scratch directory was
removed. Earlier manual interrupts and root-filesystem `ENOSPC` remain visible
as additive failed/incomplete evidence and are not relabeled as test failures
or passes. Workspace warnings-denied Clippy, workspace doctests, dependency
policy, formatting, admission, anti-evasion, AUTH11, unit compliance, and the
focused authority suite also have raw passing logs.

After the heavy run, one whitespace-only generator line and trailing whitespace
in a captured doctest log were removed. The generator digest and both binding
definition copies were consequently updated, while the fixture, equations,
contract sections, computed values, production Rust, and all other workspace
test inputs remained unchanged. The current generator reproduces the exact
unchanged fixture, and both reviewers plus current focused execution rechecked
the affected identity lane. Reuse of the unaffected heavy results therefore
satisfies the testing strategy's rule that changed inputs be identical or
demonstrably excluded; rerunning the 55-minute workspace suite would not test a
changed executable or scientific input.

## Commands Run By Verifier B

- `.venv/bin/python .../artifacts/reference_calculator.py`: PASS; reproduced
  exactly 49,915 fixture bytes and the same fixture digest.
- `cargo nextest run --test vegetation_boundary_authority_contract --profile
  quick`: PASS, 17/17.
- `cargo clippy --test vegetation_boundary_authority_contract -- -D warnings`:
  PASS.
- `check_science_contract_admission.sh --base-ref 4f5bb1c... --worktree`:
  PASS, `A0_ADMITTED contracts=45 science_surfaces=0`, authority SHA-256
  `f4e3e5280f46fbc881fb5f766b67e007d08d72f485c4abe0144da4fa8a46a97b`.
- `check_sc_unit_compliance.sh` for `SC-VEGETATION-001`: PASS.
- `check_authority_suite_antievasion.sh`: PASS.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract
  --profile quick`: PASS, 3/3.
- `cargo fmt --all -- --check` and `git diff --check`: PASS.
- `markdown-doc lint` for the package, contract, science-contract index, and
  package catalog: PASS, zero errors and zero warnings.
- Definition-copy comparison, protected V1/V2 digest reconstruction, exact
  changed-path audit, package-size/hygiene check, and prompt-state inspection:
  PASS.

## Terminal Verdict

`PASS`. No material science, schema, fixture-independence, gate-legitimacy,
review-disposition, write-set, protected-byte, or truthfulness finding remains.
Verifier B authorizes prompt archival and final lifecycle disposition after
the other independent terminal verifier also returns PASS. This verdict
releases implementation authority only; it makes no runtime activation,
consumer-cutover, canopy-snow, soil-transformation, calibration, empirical
validation, identifiability, or transferability claim.
