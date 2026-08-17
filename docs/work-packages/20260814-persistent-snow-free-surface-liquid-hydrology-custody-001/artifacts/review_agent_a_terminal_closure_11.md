# Review Agent A — Terminal Closure 11 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `862eec744bdb2e06989bcf74f0daae3e706af6fe`

Verdict: `HOLD / NO-GO`.

The review used a source archive generated from the exact reviewed Git object.
The shared checkout advanced after isolation; no later Rust or documentation
bytes are assessed or used as evidence here. This review adds documentation
only.

## Finding

### Medium — The line-count inventory contains two contradictory rows for `runoff.rs`

The substantive closure-10 line-governance corrections are present. The exact
counts match the source, every affected Rust file remains below 3,000 lines,
and the previously incomplete owner and vegetation rows now include concrete
future split intent. The 2,998-line ingress test module is WARN and explicitly
requires its fixture/vector split before further cases are added.

The inventory is not exact, however. In
`docs/work-packages/20260814-persistent-snow-free-surface-liquid-hydrology-custody-001/artifacts/line-count-governance.md`,
`direct_runtime/runoff.rs` appears twice:

- line 9 retains the old non-WARN disposition, stating only that the file is
  below 3,000 lines; and
- line 19 adds the correct WARN, decomposition rationale and follow-on split.

One affected file therefore has two conflicting dispositions in a table
described as the current complete inventory. This does not satisfy the single,
truthful WARN disposition required by `crates/AGENTS.md:57-60`, and it makes the
artifact's closing compliance claim ambiguous. Remove the stale line-9 row and
retain the complete WARN row. No runtime change or gate rerun beyond Markdown
hygiene is needed for this documentation-only correction.

## Closure-10 High Finding Re-Audit

`A-TERMINAL-CLOSURE10-HIGH-001` is corrected.

`first_membership_aware_mismatch()` is now cardinality-aware at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:2342-2367`:

- shorter actual sequences report the first expected identity whose
  multiplicity is missing;
- longer actual sequences report the first actual identity whose running
  multiplicity exceeds the complete expected multiplicity; and
- equal-length replacement or reorder reports the first actual positional
  mismatch.

This supplies the required context direction for stores and continuations.
The new matrix covers every first/middle/last identity for deletions, appended
duplicates and forged replacements, plus all pairwise reorders at
`surface_liquid_ingress_tests.rs:2415-2547`. Missing cases name the expected
member, excess cases name the excess actual member, and equal-length cases
name the actual replacement/reordered row.

`validate_surface_liquid_closure_operands()` now completes every failure with
the exact independent-closure phase, operand transaction, configured owner,
beginning owner hash and recomputed attempted owner hash at
`surface_liquid_closure.rs:1096-1127`. Existing applicable row identity is
preserved because `complete_context()` fills only absent fields. The store,
continuation and aggregate assertion helpers bind:

- exact E010 code and `IndependentClosure` phase;
- exact transaction and owner;
- full OFE/tile/surface/source store identity with absent parcel;
- OFE-only continuation identity with typed absence for the inapplicable
  tile/surface/source/parcel fields;
- typed absence for all row fields on owner/configuration/digest-wide errors;
  and
- exact beginning and attempted rollback hashes.

Owner, configuration and digest-wide poisons at
`surface_liquid_ingress_tests.rs:2568-2628` prove aggregate typed absence. No
first-OFE context is fabricated. The complete-context wrapper is read-only,
preserves already contextualized code/identity, and the public candidate path
retains the same payload under its outer completion boundary.

## Canonical Order, Frozen Vector, And Endpoint Re-Audit

All earlier closure findings remain corrected:

- One typed constructor defines local and condensation source IDs, and one
  five-field key defines start, end, origin store, kind and source-ID order.
  Production, frozen identity and independent projected ordering consume those
  definitions without sharing physical allocation arithmetic.
- The mixed-kind, unequal-area, downstream-overlap fixture freezes every
  receipt's source, basis, canonical kind, disposition, complete typed
  recipient, support, mass, mixture temperature and enthalpy bits. It includes
  nonzero infiltration, retained surface water, routed runoff and outlet
  runoff and fixes final-remainder ownership.
- Frozen receipt temperature and enthalpy bind chronological `h_mix,b`, Q and
  per-source attribution. Exact ending stores and WB14 continuation supply,
  infiltration, cadence and lineage are also frozen; caller ingress reordering
  remains byte invariant.
- Expected construction retains zero access to actual receipts. The
  receipt-free replay reconstructs every partition, retained store, routed
  descendant and ending continuation before the persistent-state and digest
  joins.
- Parcel joins retain owner, source parcel, origin/current store, complete
  typed recipient, basis OFE, `UpstreamRunon` kind, support and disposition.
  Unequal-area conversion is applied exactly once at each route.
- Raw `Q = mass * specific enthalpy`, canonical mixture arithmetic, per-source
  and OFE closure, receiving soil-liquid, soil-thermal and retained-LSE joins
  remain independently checked.
- Arithmetic/domain E003 still precedes immutable producer E009, which
  precedes independent E010. Both beginning cumulative-infiltration bounds are
  checked before the zero-supply shortcut.
- Exact D/A/F custody, finalized-use-only debit, signed condensation,
  pre-ingress capacity, one stateful WB14 call per OFE, strict restart lineage,
  clone-only candidate behavior, byte-identical rollback and snow/frost
  rejection remain intact.

The reviewed Rust diff changes only closure error completion, cardinality-aware
context selection and its test matrix. It does not change arithmetic, clamp or
guard precedence, units, physical constants, serialization, state schema,
receivers, selection or publication. No duplicated physical allocation was
introduced; the intentional producer/projector duplication remains justified
by the independent anti-tautology requirement.

No new numerical, constitutive, serialization, taxonomy, rollback, receiver or
production-selection defect was found in the full custody endpoint.

## Exact Line-Count Inventory

The exact source counts are:

| File | Lines | Static disposition |
|---|---:|---|
| `direct_runtime/runoff.rs` | 2,852 | WARN; correct rationale/split exists in the duplicate second row. |
| `direct_runtime/00_core_frames.rs` | 2,783 | WARN; bounded seam and later broader split. |
| `direct_runtime/surface_liquid_owner.rs` | 2,347 | WARN; rationale and persistence/schema split. |
| `direct_runtime/surface_liquid_owner_tests.rs` | 876 | PASS. |
| `direct_runtime/surface_liquid_ingress.rs` | 2,014 | WARN; rationale and identity-boundary split. |
| `direct_runtime/surface_liquid_ingress_tests.rs` | 2,998 | WARN; rationale and mandatory fixture/vector split before growth. |
| `land_surface_energy_shadow/mod.rs` | 2,881 | WARN; rationale and receiver-DTO split. |
| `direct_runtime/surface_liquid_closure.rs` | 2,678 | WARN; rationale and projection/diagnostic split. |
| `direct_runtime/surface_liquid_wb14.rs` | 303 | PASS. |
| `vegetation_real_hydrology_shadow.rs` | 2,157 | WARN; rationale and snapshot/lane-map adapter split. |

No file reaches the 3,000-line closure blocker. The finding concerns the
duplicated contradictory evidence row, not a missing source-file count.

## Exact-Commit Validation

Ran from a source archive generated from
`862eec744bdb2e06989bcf74f0daae3e706af6fe`:

```text
CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  -p openwepp-hillslope-orchestrator --profile quick
PASS: 562/562; 3 slow retained routing-oracle tests; 0 skipped

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo clippy \
  -p openwepp-hillslope-orchestrator --all-targets --all-features -- \
  -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check \
  f375349e38c134eb6eb105d03a87d85841ba8c26...\
  862eec744bdb2e06989bcf74f0daae3e706af6fe
PASS
```

These commands prove the focused authority/consumer surface, the complete
owning-crate quick suite and strict source quality for the exact reviewed
bytes. Full-workspace, doctest, dependency-policy and release gates were not
rerun by this reviewer.

## Residual Risk And Missing Tests

No material science-state, endpoint, diagnostic-context or rollback risk
remains from the closure-10 findings. The structural matrix is comprehensive
for the three-record/three-continuation endpoint and the exact aggregate
payload. The remaining risk is evidence-only: automated Markdown lint does not
detect duplicate semantic inventory rows, so the contradictory runoff entry
must be removed manually.

The 2,998-line ingress test module has two lines of headroom before the
mandatory 3,000-line threshold. Its recorded split-before-growth requirement
must be honored by the resumed campaign.

## Approval Statement

`NO-GO`: exact commit `862eec744bdb2e06989bcf74f0daae3e706af6fe`
fully corrects the closure-10 Rust context, payload and rollback finding and
retains every historical custody, ordering, frozen-vector, endpoint and
taxonomy correction. Terminal evidence remains on HOLD solely because the
line-count inventory contains contradictory duplicate `runoff.rs` rows. Remove
the stale non-WARN row and lint the artifact before terminal approval.
