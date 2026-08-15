# Review Agent A — Terminal Rust Release Review

Evidence class: `Static exact-commit + Ran exact-commit + Reused exact-source gates`

Reviewed commit: `b42fad45c92f1ac0d7144de29fd6e68e8bddf2ff`

Verdict: `PASS / GO`.

The review used a detached worktree at the exact reviewed Git object. The
shared checkout advanced after isolation; no later bytes are assessed or used
as evidence here. This review adds documentation only.

## Findings

No material finding.

The closure-11 duplicate line-count evidence defect is corrected, generation
26 supplies the previously missing science-impact bindings, and every accepted
review finding has an explicit passing correction or a disposition covering
the same finding family. No new runtime, numerical, serialization, taxonomy,
rollback, authority-map or evidence defect was found.

## Exact Source-Equivalence And Gate Reuse

Rust gate evidence from exact commit
`862eec744bdb2e06989bcf74f0daae3e706af6fe` is reusable for the reviewed
commit. Git object identity proves that the complete Rust implementation,
crate-local tests, integration tests and build graph did not change:

| Surface | `862eec744` object | `b42fad45c` object |
|---|---|---|
| `crates/` tree | `c6a583921bb8527ccfa83e2081b4a23041e7372f` | `c6a583921bb8527ccfa83e2081b4a23041e7372f` |
| `tests/` tree | `f438299654fdbbd60129a0dc6586eec883ab48fb` | `f438299654fdbbd60129a0dc6586eec883ab48fb` |
| root `Cargo.toml` | `b58749e3213443b0f796ceb09c18175e4f4b754d` | `b58749e3213443b0f796ceb09c18175e4f4b754d` |
| root `Cargo.lock` | `218eda78c595af0cc5f71ff98a5ec2e15fded0b7` | `218eda78c595af0cc5f71ff98a5ec2e15fded0b7` |

`git diff --name-only 862eec744...b42fad45c` names only three package
evidence files, two review artifacts and
`tools/release/authority-policy/impact-map.json`. It names no Rust source,
crate manifest, lockfile, integration test, Nextest configuration or toolchain
file.

The following exact-`862eec744` gates recorded by the closure-11 Rust review
therefore apply byte-for-byte to `b42fad45c`:

- focused custody authority/real-owner integration: 28/28 PASS;
- complete `openwepp-hillslope-orchestrator` quick suite: 562/562 PASS;
- owning-crate all-target/all-feature Clippy with `-D warnings`: PASS;
- workspace formatting: PASS; and
- source diff hygiene through the closure-11 increment: PASS.

This is exact-object reuse, not inference from similar source or a generic
build-only validator.

## Closure-11 Finding And Complete Disposition Re-Audit

The sole closure-11 Rust finding and the independently reported
`B-TERMINAL-CLOSURE12-MEDIUM-001` are the same evidence defect: two conflicting
rows for `direct_runtime/runoff.rs`. Commit `97ea4accea256ca65123f3f32dac136b45b67cf8`
removes the obsolete non-WARN row. The exact current inventory now contains one
row per affected file, its counts match `wc -l`, every 2,000-line file has a
WARN rationale plus follow-on split intent, and no file reaches 3,000 lines.

The final counts include:

- `surface_liquid_ingress_tests.rs`: 2,998, with a binding split-before-growth
  requirement;
- `surface_liquid_closure.rs`: 2,678;
- `runoff.rs`: 2,852, one WARN row with a direct-runtime decomposition plan;
  and
- every other affected file exactly once.

`review-finding-disposition.md` accepts and closes the duplicate-row finding.
It also retains accepted/passing dispositions for all earlier runtime,
receiver, E003/E009/E010/E011, endpoint, canonical identity/order, frozen
vector, rollback and line-governance findings. No finding is rejected,
silently deferred or deleted. The package and final disposition remain
truthfully pending terminal review at the reviewed commit; they do not make a
premature custody-lift or Child-3 completion claim.

## Generation-26 Impact Binding Review

Generation 26 is a valid bounded increment from generation 25. It retains
`SCHEMA_ONLY_NONBLOCKING`, the ADR-0039 policy digest and fail-closed unknown-
path escalation, and adds exactly three unique CRITICAL entries:

| Entry | Exact path | Contract |
|---|---|---|
| `surface-liquid-direct-owner-tests` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner_tests.rs` | `SC-SURFACELIQUID-001` |
| `surface-liquid-direct-ingress-tests` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_tests.rs` | `SC-SURFACELIQUID-001` |
| `surface-liquid-authority-contract-test` | `tests/integration/surface_liquid_hydrology_custody_authority_contract.rs` | `SC-SURFACELIQUID-001` |

The crate-local entries bind the owning orchestrator package/test target. The
integration entry binds both the root integration-test package and the owning
orchestrator package in `affected_packages`, `test_targets` and
`covering_test_targets`. All three paths exist, entry IDs remain unique, and
the package write set explicitly authorizes atomic impact-map bindings for new
custody science surfaces.

Exact-base science admission passes with:

```text
A0_ADMITTED contracts=46 science_surfaces=16
base=af9a989063aa8751dfadb14c442e1b360653658c
head=WORKTREE
authority_sha256=79ccacc0584d487443978fcbc6bda16c9a5e8cb70c652d9358c3dad85e219b19
```

This closes the retained admission failure for the extracted test modules
without changing contract authority, enforcement posture or production code.

## Full Custody Endpoint Release Re-Audit

Source identity with the exact closure-11 review preserves every accepted
runtime correction:

- exact persistent store/restart bytes, digests, transaction lineage and
  clone-only rollback;
- immutable-snapshot D/A/F, finalized-use-only debit and signed condensation;
- one stateful chronological WB14 transition per OFE with complete cumulative
  supply/infiltration bounds;
- canonical five-field parcel ordering and local/condensation source IDs with
  separate production and receipt-free expected allocation;
- bit-frozen nonzero infiltration, retention, routed/outlet attribution,
  chronological `h_mix,b`, Q, typed recipients, ending stores and
  continuations;
- receipt-free independent persistent endpoint reconstruction before strict
  digest/state validation;
- exact E003 before E009 before E010 precedence and complete cardinality-aware
  E010 context/rollback payloads; and
- strict receiving-owner joins, unsupported snow/frost rejection and default-
  off production selection.

No arithmetic, clamp/guard precedence, dimensional conversion, state schema,
serialization, receiver, model identity, selector, publication or calibration
surface changed after the exact source review. Intentional production/
projector physical duplication remains justified by the anti-tautology
boundary; canonical constants, identity and ordering remain centralized.

## Exact-Commit Checks Run

Ran in a detached worktree at
`b42fad45c92f1ac0d7144de29fd6e68e8bddf2ff`:

```text
bash tools/release/check_science_contract_admission.sh \
  --base-ref af9a989063aa8751dfadb14c442e1b360653658c \
  --worktree
PASS: A0_ADMITTED; 46 contracts; 16 science surfaces

bash tools/release/check_authority_suite_antievasion.sh
PASS

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  --profile quick \
  --test auth11_required_suite_obligation_guards_contract
PASS: 3/3; 0 skipped

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  --profile quick \
  --test advisory_linter_authority_contract
PASS: 7/7; 0 skipped

bash tools/release/check_sc_unit_compliance.sh \
  --path docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md
PASS

markdown-doc lint \
  --path docs/work-packages/20260814-persistent-snow-free-surface-liquid-hydrology-custody-001
PASS: 56 files; 0 errors; 0 warnings

git diff --check \
  862eec744bdb2e06989bcf74f0daae3e706af6fe...\
  b42fad45c92f1ac0d7144de29fd6e68e8bddf2ff
PASS
```

A separate `jq` audit confirmed generation 26, three expected new entries,
unique entry IDs, CRITICAL risk and exact `SC-SURFACELIQUID-001` binding.

## Residual Risk And Missing Tests

No release-blocking custody risk remains. The Rust source is identical to the
fully tested closure-11 object, and the only later executable policy input has
current schema, admission and anti-evasion evidence.

The 2,998-line ingress test module is two lines below the mandatory 3,000-line
threshold. The recorded fixture/vector split must occur before any resumed
campaign adds cases. This is a binding maintenance constraint, not a current
closure defect.

Full-workspace release gates were not rerun because no source, test, Cargo,
Nextest or toolchain byte changed after the exact closure-11 source gates. This
review does not authorize production selection, publication, calibration,
deployment, completion of held LSE Child 3 or completion of the parent
campaign.

## Approval Statement

`GO`: exact commit `b42fad45c92f1ac0d7144de29fd6e68e8bddf2ff`
has no unresolved Rust correctness, science-contract, endpoint, taxonomy,
rollback, impact-map or evidence finding. The dependency-lift package may
proceed to truthful terminal disposition and resume the previously held Child
3 within its existing scope; production activation and broader campaign
closure remain separately governed.
