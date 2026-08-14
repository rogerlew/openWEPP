# Terminal Verification B — Rust And Package Integrity

Status: `PASS / GO`

Evidence class: `Static + Ran`

Verifier role: independent terminal Rust/API/evidence verifier

Reviewed identity: local `main` at
`3f1cf8ee32855a501d7d5b07ac3459d8a3fc8cc3` plus the current uncommitted
Child-2 worktree. The campaign base resolves to
`0db1960129ad4f8fc4e292b20574dfe7229d5fe1`; the branch remains two local
commits ahead of `origin/main` and no Child-2 commit or push exists.

## Verdict

`PASS / GO` for the declared bounded endpoint:

```text
COMPLETE / V7 vegetation-real-hydrology arbitration shadow implemented /
production unchanged
```

No unresolved material Rust, arithmetic, candidate-isolation, impact-map,
selector-exclusion, review-disposition, evidence, or prompt-archive finding
remains within the default-off single-OFE Child-2 boundary.

## Exact Implementation Identity

The independently inspected load-bearing file hashes are:

| File | SHA-256 |
|---|---|
| `vegetation_real_hydrology_shadow.rs` | `ddf743c469d5fe8e59cde153921dc2996a9cbb52e6ec00f08c2ccce6b7977646` |
| `direct_runtime/real_water_owner.rs` | `79ee609ca8a15add0ab024ea898084bef67edfff2c2e77bbcd01922160b3f9ad` |
| `resource_transaction.rs` | `9bfc69630584c603f20317dd12f4072724fce9157b2df50a29b91a69f3d1c3c1` |
| vegetation `water_phase.rs` | `486f262cab76a3c5d653d1f552bb47df778675b1dd2b88b0bb062aa21d4706de` |
| public integration contract | `6a4cb9b02785af627b057fe449b013703f7bff3b262a607aca634084671aca6f` |
| authority impact map | `8a050e73e1186fd2879cbbe84e2610e6e90f82b288c13b5785cd944fc1e6509d` |

## Independent Static Verification

### API and candidate isolation

- `execute_v7_real_hydrology_water_shadow()` is an explicit public shadow API;
  neither `openwepp-runner` nor direct-runtime production dispatch calls it.
- The bridge rejects more than one OFE/lane and any selected lane other than
  index zero. Low-level multi-lane tests prove source-key isolation without
  being misrepresented as routed consumer evidence.
- The adapter borrows the production `DirectRunFrame` immutably, calls the
  production `seed_day_frame()` state constructor, retains a complete cloned
  beginning frame, and constructs the ending owner state in another clone.
- Authorization exposes no commit operation. Candidate construction performs
  all fallible identity, protocol, aggregate and debit checks before returning
  the owned candidate. The supplied production frame is unchanged on success
  and failure.
- A single arbiter instance rejects a second authorization and a second
  candidate construction. The vegetation phase therefore performs one
  potential request, one authorization and one fixed-cap finalization.

### Typed identity and arithmetic seams

- The owner envelope preserves transaction, interval, requester occupancy,
  OFE/lane, layer, stand-ground amount basis and the immutable beginning
  snapshot through request, maximum authorization, finalized use and debit.
- The public join checks the immediately following transaction, exact interval,
  configured layer cardinality/order, beginning liquid and frozen facts before
  the V7 solve. Root access comes from the joined vegetation forcing rather
  than a second caller-provided fact.
- The production-owned authorization endpoint reads actual seeded day-frame
  layers. It delegates equal-status allocation to the shared typed kernel
  primitive; no second proportional algorithm exists in the adapter.
- The shared allocator validates finite nonnegative operands, rejects derived
  overflow, canonically orders each source group, bounds every authorization
  by its request and remaining supply, and applies a bounded closing correction
  for binary64 overdraw. Returned records retain caller order and exact keys.
- Receiving validation and debit reconstruction use canonical identity maps
  and compensated sums at the source joins. Exact `0 <= F <= A <= D` is
  validated before any cloned layer mutation; authorization is never treated
  as finalized use.
- The production native root-uptake path and shadow candidate share
  `apply_direct_finalized_layer_liquid_debit()`. ET, subsurface and the shadow
  candidate also share the one aggregate-soil-water implementation.
- Reason precedence is consistent at the producer and vegetation receiver:
  zero demand, rooting exclusion, frozen exclusion, zero/nonzero storage limit,
  eligible competition and full supply are distinguished. Positive and
  negative zero use the same exact numerical zero class.

### Rollback, serialization and production behavior

- Canonical snapshot bytes truthfully cover the bounded arbitration projection:
  scheduler/owner/transaction/interval identity, OFE topology and area,
  water/transfer state, exact layer order, and all twelve layer fields.
- The package does not claim those bytes serialize every production runtime
  field. Unprojected production state is retained in the complete clone and is
  protected by whole-frame structural equality.
- Error cases cover stale and foreign identities, wrong basis/layer/OFE,
  duplicate keys, partial frost, over-finalization, zero supply, signed zero,
  aggregate rounding and exact full depletion. No rejected path exposes a
  candidate or mutates the borrowed production value.
- Production native ET remains unchanged and reachable only through its
  existing executor. The shadow does not invoke native ET, subtract it later,
  donate denied canopy demand, create ground demand, alter defaults, or publish
  an output.

### Impact map and package governance

- `impact-map.json` is valid JSON at generation 24. Exact entries cover the new
  production-owner endpoint, shared debit, centralized subsurface aggregate,
  shadow transaction and shared resource protocol. Changed transaction
  surfaces bind to admitted `SC-VEGETATIONTRANSACTION-001`; the package does not
  falsely bind the draft/in-review `SC-WATBAL-001` as admitted authority.
- Admission reports 45 admitted contracts and nine changed science surfaces.
- Both independent reviews are `GO`. Every recorded hydrology and Rust finding
  is accepted and corrected or, for routed multi-OFE, bounded explicitly to the
  later real-consumer child. No finding is silently rejected or left
  undispositioned.
- The two initially stale ledger status lines were corrected during terminal
  verification to record the completed hydrology and Rust reviews. Current
  package Markdown lint passes after that correction.
- `vegetation_real_hydrology_shadow.rs` is exactly 2,118 lines. The required
  `WARN` is recorded with a bounded rationale and a mandatory decomposition
  before Child 4 extends the module. It remains below the 3,000-line closure
  block and is not presented as a waiver for future growth.

## Comparator And Exact-Byte Evidence

The final comparator bundle at
`/tmp/openwepp-child2-comparator-20260814-20260814-113603` passed without
retries. It includes all four affected checks and strict Clippy gates, 17/17
vegetation implementation-contract tests, 26/26 vegetation authority tests,
3/3 AUTH11 tests, 3/3 public Child-2 tests, 507/507 orchestrator quick tests,
anti-evasion, admission, formatting, diff hygiene and package Markdown lint.

Filesystem timestamps and the terminal diff show that changes after the
comparator completed are confined to comparator/review/gate/terminal evidence
Markdown. No Rust, test, Cargo or impact-map byte changed afterward. This
verifier reran the focused executable, arithmetic, admission and hygiene gates
on the current worktree and reran package Markdown lint after the final status
corrections.

## Ran Evidence

Executed independently against the current worktree:

- `cargo nextest run --test vegetation_real_hydrology_shadow_contract
  --profile quick` — PASS, 3/3.
- `cargo nextest run -p openwepp-kernel-contract -E 'test(proportional_) |
  test(projected_supply_) | test(canonical_resource_)' --profile quick` —
  PASS, 7/7 selected, 48 skipped by the explicit filter.
- `cargo nextest run -p openwepp-vegetation -E 'test(water_phase)'
  --profile quick` — PASS, 6/6 selected, 221 skipped by the explicit filter.
- `bash tools/release/check_science_contract_admission.sh --base-ref
  0db196012 --worktree` — PASS; 45 contracts, nine changed science surfaces,
  authority SHA-256
  `ac829c7b73c92022e269823a2f88c3329efcc4785e4c8cd10caef6dfb455e5af`.
- `cargo fmt --all -- --check` — PASS.
- `git diff --check` — PASS before writing this report.
- `jq empty tools/release/authority-policy/impact-map.json` — PASS.
- Recursive runner/direct-runtime selector scan — PASS; no production call to
  the shadow API.
- `markdown-doc lint --path
  docs/work-packages/20260814-vegetation-real-hydrology-arbitration-shadow-001`
  — PASS, 25 files and zero findings before writing this report.

## Archive Readiness And Retained Boundaries

The active prompt SHA-256 is
`1c96379481b12ad1ae587b915391024d89aa84d996f9249d22eed66e6c82f940`.
It remains active as required while verification is in progress and is ready
for byte-for-byte archival now that both independent terminal verifiers report
PASS. Archival must preserve that digest and must not be represented as a push,
activation, selector change or production cutover.

The following remain explicit later-child obligations and are not claimed by
Child 2: routed multi-OFE scheduling, partial-frost execution, a ground-water
requester, land-surface-energy runtime, real scheduler consumption, exhaustive
whole-consumer phase injection, production publication and activation.

## Final Verification Disposition

`PASS / GO`. The exact implementation is API-isolated, arithmetically bounded,
identity-preserving, fail-closed, rollback-safe and unreachable from production
selection. Review and comparator evidence supports the stated bounded claim,
package artifacts are truthful after the terminal status cleanup, no Child-2
push or activation occurred, and the prompt is ready for byte-identical
archival.
