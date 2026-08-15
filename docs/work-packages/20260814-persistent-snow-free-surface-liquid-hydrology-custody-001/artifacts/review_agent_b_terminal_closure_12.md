# Review Agent B Terminal Closure 12 — Hydrology, Science, And Ownership

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `862eec744bdb2e06989bcf74f0daae3e706af6fe`

Verdict: `HOLD / runtime and science PASS; one contradictory line-governance record remains`.

This fresh review preserves every prior finding and failed review as immutable
history. It re-audits the exact clean commit above from immutable beginning
state through D/A/F, signed condensation, chronological WB14 ingress, routing,
receiving-owner joins, persistent ending state, restart and rollback.

## Material finding

### B-TERMINAL-CLOSURE12-MEDIUM-001 — the complete affected-file inventory contains two conflicting `runoff.rs` dispositions

The runtime portion of `A-TERMINAL-CLOSURE10-MEDIUM-002` is not at issue. No
affected Rust file reaches the mandatory 3,000-line threshold, and every other
file at or above 2,000 lines has a WARN rationale plus follow-on split intent.

The final evidence artifact is internally contradictory, however.
`artifacts/line-count-governance.md` lists the same 2,852-line
`direct_runtime/runoff.rs` twice:

- line 9 says only that it is below the mandatory threshold after a completed
  extraction and does not mark WARN or provide a future split; and
- line 19 marks it WARN and supplies the required future separation of WB14
  mechanics from unrelated runoff kernels.

The table therefore is not one exact, unambiguous affected-file inventory, and
its statement that all WARN files satisfy governance depends on choosing the
second row over the first. This is the same evidence surface named by the
accepted Rust review finding, so the finding cannot truthfully be marked fully
closed while both dispositions remain.

Required correction: remove the obsolete first `runoff.rs` row, retain the WARN
row with its rationale and split intent, rerun package Markdown and diff checks,
and obtain fresh exact-byte review. No source, test, authority, physics or model
identity change is required.

## Context-direction correction

`A-TERMINAL-CLOSURE10-HIGH-001` is completely corrected.

The membership-aware endpoint helper now distinguishes sequence direction by
cardinality:

- fewer actual rows report the exact expected missing identity;
- more actual rows report the first actual excess identity, including a
  duplicated row at the point its multiplicity becomes excessive; and
- equal-length replacements and reorders report the actual first mismatching
  row.

The complete store and continuation matrix covers first, middle and last
deletions, additions/duplicates and replacements, plus every pairwise reorder.
It asserts `E010`, independent-closure phase, transaction, owner, exact
available row identity, typed absence for inapplicable fields, and beginning
and attempted rollback hashes. Owner, configuration and digest-wide failures
retain aggregate context with no fabricated OFE or tile.

`validate_surface_liquid_closure_operands()` now completes every independent
failure with canonical rollback hashes. The added context machinery is
read-only and changes no candidate construction or accepted value.

## No physics or fixture regression

The closure11 corrections remain intact:

- one typed canonical order key governs production, frozen and projected DTOs;
- one canonical constructor governs local and condensation source IDs;
- production allocation and receipt-free expected allocation remain separate;
- the bit-frozen mixed-kind, unequal-temperature, unequal-area and
  downstream-overlap vector still fixes every receipt source, OFE, kind,
  disposition, typed recipient, support, mass, temperature and Q bit; and
- the same vector fixes ending stores, continuation values, cadence and
  transaction lineage and remains invariant to caller ingress order.

The exact source diff changes endpoint failure completion, mismatch-direction
selection and poison coverage. It does not alter WB14 equations, source
amounts, partition arithmetic, mixture arithmetic, retention, routing,
authorization, model identity or state schema.

## Complete custody and historical-finding re-audit

No runtime, scientific or ownership defect was found:

- Strict per-OFE/tile/surface/source persistent state, canonical restart bytes,
  digests and predecessor lineage remain enforced.
- One immutable beginning snapshot supplies typed requests and proportional
  maximum authorizations. Exact `0 <= F <= A <= D` is independently
  reconstructed, finalized use alone debits storage, and unused authorization
  remains.
- Signed condensation credits the exact store before ingress and routes
  capacity overflow with exact mass, temperature and enthalpy identity.
- Open rain and covered canopy releases remain mutually exclusive. Each OFE
  executes one admitted stateful chronological WB14 continuation per interval.
- Expected infiltration, retention, routed runoff and outlet runoff have zero
  access to actual receipts. Complete owner, source, origin/current store,
  recipient, basis OFE, kind, support and disposition identity enters the join.
- Routed descendants become canonical `UpstreamRunon`, preserve source/origin
  lineage, use destination OFE/store identity and apply unequal-area mass and
  energy conversion once.
- Raw `Q = mass * specific enthalpy`, canonical chronological `h_mix,b`,
  per-source and OFE aggregate closure, soil-liquid, soil-thermal and
  retained-LSE receipts remain independently reconstructed with checked
  arithmetic.
- Numeric/domain `E003` precedes structural producer `E009`, which precedes
  independent `E010`; both cumulative infiltration bounds remain enforced
  before zero-supply handling.
- Receipt-free final stores and continuations join directly to the persistent
  ending state before digest and strict-state validation.
- Candidate construction remains clone-only, and failure paths preserve
  byte-identical beginning owners with canonical rollback hashes.
- Snow, terminal snow, frozen and thawing branches remain typed unsupported.

## Production and campaign boundaries

No runner selector, production scheduler reachability, default dispatch,
output publication, runtime activation, calibration value or consumer cutover
was added. Production execution remains unchanged and the bridge remains
explicitly default-off. This review concerns the custody dependency lift only;
it does not claim completion of held LSE Child 3 or the parent campaign.

Current code counts are 2,998 lines for
`surface_liquid_ingress_tests.rs` and 2,678 lines for
`surface_liquid_closure.rs`; both remain below the mandatory split threshold
and have explicit WARN split plans. The finding is solely the duplicate,
conflicting `runoff.rs` evidence row.

## Commands run at the exact reviewed commit

```text
git rev-parse HEAD
PASS: 862eec744bdb2e06989bcf74f0daae3e706af6fe

git status --short --branch
PASS: clean main; 77 commits ahead of origin/main

cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 55/55 selected; 507 skipped by the focused filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

## Approval statement

`NO-GO`: exact commit `862eec744bdb2e06989bcf74f0daae3e706af6fe`
corrects final context direction and retains the complete custody endpoint with
no physics regression. Terminal disposition remains blocked only because the
same affected `runoff.rs` file has two conflicting line-governance rows. Remove
the obsolete row and repeat exact-byte review; no further runtime correction is
indicated.
