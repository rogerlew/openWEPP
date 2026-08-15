# Review Agent B — Terminal Hydrology And Science Release Review

Evidence class: `Static exact-commit + Ran exact-commit + exact-tree evidence reuse`

Reviewed commit: `b42fad45c92f1ac0d7144de29fd6e68e8bddf2ff`

Verdict: `PASS / persistent snow-free surface-liquid hydrology custody release approved / no unresolved material hydrology, science, ownership, or evidence finding`.

This release review preserves all earlier findings, HOLDs and failed attempts as
historical evidence. It reviews the exact clean commit above and does not infer
source equivalence from commit messages.

## Exact-byte reconciliation and evidence reuse

The range from the last fully reviewed runtime commit
`862eec744bdb2e06989bcf74f0daae3e706af6fe` to the reviewed release commit
changes only:

- the authority impact map;
- package gate, line-count and finding-disposition evidence; and
- the retained closure11 Rust and closure12 hydrology review artifacts.

There is no crate or integration-test source diff. Git object identity proves:

```text
crates tree at 862eec744:  c6a583921bb8527ccfa83e2081b4a23041e7372f
crates tree at b42fad45:   c6a583921bb8527ccfa83e2081b4a23041e7372f

tests tree at 862eec744:   f438299654fdbbd60129a0dc6586eec883ab48fb
tests tree at b42fad45:    f438299654fdbbd60129a0dc6586eec883ab48fb

SC-SURFACELIQUID-001 blob at both commits:
c18b13938a19c86a7d90991ef5c676fb8b8065e6
```

Accordingly, this review reuses the exact source-level 55/55 focused
orchestrator and strict all-feature Clippy evidence run at `862eec744`, where
this reviewer inspected and tested the identical Git trees. It does not reuse
that evidence for the changed authority map or documentation. Those changed
surfaces were inspected and gated again at `b42fad45` as recorded below. The
28-test authority/consumer suite was also rerun at the release commit.

## Closure12 evidence correction

`B-TERMINAL-CLOSURE12-MEDIUM-001` is completely corrected.

`line-count-governance.md` now contains exactly one `runoff.rs` row. The one
authoritative row records 2,852 lines, WARN status, why the file remains
cohesive for this package and the future split of WB14 mechanics from unrelated
runoff kernels. Every other affected Rust file at or above 2,000 lines likewise
has one WARN rationale and explicit follow-on split intent. No affected file
reaches the mandatory 3,000-line threshold; the 2,998-line ingress-test module
explicitly requires splitting before further cases are added.

The review-finding disposition records the duplicate-row finding as accepted
and remediated. Every preceding accepted authority, implementation, receiver,
arithmetic, taxonomy, context, endpoint, ordering and evidence finding has a
recorded remediation. Historical prose stating that a then-future review was
pending is retained as chronology, not an undispositioned finding.

## Generation-26 authority bindings

The generation-26 impact map is valid JSON, has no duplicate entry ID and adds
exact CRITICAL bindings for:

```text
surface-liquid-direct-owner-tests
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/
  surface_liquid_owner_tests.rs

surface-liquid-direct-ingress-tests
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/
  surface_liquid_ingress_tests.rs

surface-liquid-authority-contract-test
  tests/integration/surface_liquid_hydrology_custody_authority_contract.rs
```

Each entry names the water-balance process owner, the persistent snow-free
surface-liquid custody semantic surface, affected and covering targets,
`SC-SURFACELIQUID-001`, the hard WAT5 runtime invariant gate and the canonical
contract documentation path. The current-base science-admission gate recognizes
all three bindings and reports 46 admitted contracts and 16 science surfaces.

## Full hydrology, science, and ownership re-audit

No regression or unresolved material defect was found in the unchanged runtime
bytes:

- Strict per-OFE/tile/surface/source persistent state, canonical restart bytes,
  digests and predecessor transaction lineage remain enforced.
- One immutable beginning snapshot supplies typed requests and proportional
  maximum authorizations. Exact `0 <= F <= A <= D` is independently
  reconstructed, finalized use alone debits storage, and unused authorization
  remains.
- Signed condensation credits the exact store before ingress and routes
  capacity overflow with exact mass, temperature and enthalpy identity.
- Open precipitation and covered canopy releases remain mutually exclusive.
  Each OFE executes one admitted stateful chronological WB14 continuation per
  interval.
- Expected infiltration, retention, routed runoff and outlet runoff have zero
  access to actual receipts. Complete owner, source, origin/current store,
  recipient, basis OFE, kind, support and disposition identity enters the join.
- Routed descendants become canonical `UpstreamRunon`, preserve source/origin
  lineage, use destination OFE/store identity and apply unequal-area mass and
  energy conversion exactly once.
- Raw `Q = mass * specific enthalpy`, canonical chronological `h_mix,b`,
  per-source and OFE aggregate closure, soil-liquid, soil-thermal and
  retained-LSE receipts remain independently reconstructed with checked
  arithmetic.
- One typed order key and one source-ID constructor govern production, frozen
  and projected DTOs while production and expected allocation remain separate.
  The bit-frozen mixed-kind, unequal-temperature, unequal-area and
  downstream-overlap fixture retains exact receipt and endpoint bits.
- Numeric/domain `E003` precedes producer `E009`, which precedes independent
  `E010`; both cumulative infiltration bounds remain checked before zero-supply
  handling.
- Receipt-free ending stores and WB14 continuations join directly to persistent
  state before digest and strict-state validation. Cardinality-aware contexts
  report missing expected or actual excess/replacement/reordered identity in
  the correct direction.
- Candidate construction remains clone-only. Complete failure payloads retain
  beginning and attempted hashes and byte-identical rollback.
- Snow, terminal snow, frozen and thawing branches remain typed unsupported in
  the declared snow-free bridge.

## Production and campaign boundaries

No reviewed commit adds a runner selector, production scheduler reachability,
default dispatch, output publication, runtime activation, calibration value or
consumer cutover. Production execution remains unchanged and the custody bridge
remains explicitly default-off.

This PASS releases the surface-liquid hydrology custody dependency. It does not
itself claim completion of the held LSE runtime child or the parent integration
campaign; those packages must resume against their own write sets, gates,
reviews and terminal dispositions.

## Commands run at the exact release commit

```text
git rev-parse HEAD
PASS: b42fad45c92f1ac0d7144de29fd6e68e8bddf2ff

git status --short --branch
PASS: clean main; 81 commits ahead of origin/main

git diff --quiet 862eec744...b42fad45 -- crates
PASS: no crate source difference

git rev-parse 862eec744:crates HEAD:crates
PASS: exact tree identity shown above

git rev-parse 862eec744:tests HEAD:tests
PASS: exact tree identity shown above

.venv/bin/python -m json.tool \
  tools/release/authority-policy/impact-map.json
PASS

jq generation, exact generation-26 entries and duplicate entry IDs
PASS: generation 26; three exact custody bindings; no duplicate ID

bash tools/release/check_science_contract_admission.sh \
  --base-ref af9a989063aa8751dfadb14c442e1b360653658c \
  --worktree
PASS: A0_ADMITTED contracts=46 science_surfaces=16

bash tools/release/check_authority_suite_antievasion.sh
PASS

cargo nextest run \
  --test auth11_required_suite_obligation_guards_contract
PASS: 3/3; 0 skipped

bash tools/release/check_sc_unit_compliance.sh \
  --path docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md
PASS

cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

markdown-doc lint --path \
  docs/work-packages/20260814-persistent-snow-free-surface-liquid-hydrology-custody-001 \
  --format json
PASS: 56 files; 0 errors; 0 warnings

cargo fmt --all -- --check
PASS

git diff --check
PASS before this release-review artifact was added
```

Reused exact-tree source evidence:

```text
cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS at identical 862eec744 source tree: 55/55 selected

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS at identical 862eec744 source tree
```

## Approval statement

`GO`: exact commit `b42fad45c92f1ac0d7144de29fd6e68e8bddf2ff`
has one truthful line-count inventory, complete generation-26 science-authority
bindings, dispositioned findings and source bytes identical to the fully
reviewed custody endpoint. No unresolved material hydrology, science,
ownership, rollback, production-exclusion or release-evidence finding remains.
The dependency-lift package may proceed to its required terminal verifiers and
truthful final disposition, after which the held LSE runtime child may resume.
