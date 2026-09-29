# BVLS-04 shifted-runtime design for independent review

**Status:** author preparation only.  This design requires independent
correctness and QA approval before a Rust body release.  It authorizes no Rust
execution, physical replay, target arm, or production activation.

## Source and evidence identity

- Canonical mutable implementation source:
  `/home/roger/openwepp-experiments/cold-canopy-m1-20260920`, as bound by the
  parent's `run_recorded.SNAPSHOT.snapshot(SOURCE)` in
  `shifted-runtime-start-custody.json`. It verified all 764 captured source
  entries against `shifted-face-capture01-source.json`; the retained identity is
  `cff19cd8d6aa20249443c826190d9ef5fe9e0bc0d9859b6e00ea3b27609a9cf8`.
- Generated targets in that directory are excluded by the snapshot. The 764-entry
  reconstruction is preserved recovery evidence only. No recovery patch is
  needed or overlaid.
- Prototype inputs are `shifted-face-observed-capture.json`
  (`946d090...ef136ecb`), `dot2-primitive.py`
  (`78260c...abc75f0`), and `shifted-refinement-protocol.json`.
  The accepted prototype and independent reconstruction agree on captured
  p0/p1/p2 and the retained `V/sigma/order`; this does not prove a true
  multiplier root.

## Reserved implementation seam

The only numerical body surface is detached
`crates/openwepp-land-surface-energy/src/m1_trust_region_stage1.rs`:

1. Follow the frozen decision table: signed zero selects unchanged BVLS-03;
   nonfinite/negative lambda never selects04 and retains unchanged02 crossing/KKT
   flow and its owning SvdNonFinite or OptimalityIndeterminate error; only failure
   to establish a feasible positive upper bracket owns LambdaBracket.
   Incomplete/rank-refused/nonfinite face construction retains its SVD owner;
   finite positive lambda on a completed
   rank-admitted face is the only BVLS-04 category; p0 box infeasibility alone
   takes unchanged BVLS-02 crossing; and active/structural/nonfinite/full-or-
   reduced-radius inconsistencies take their named terminal owner. Construct p0
   from the solved face while preserving active bits and free coordinate order.
   EFT-domain checks are not eligibility predicates: arithmetic failure after
   selection is terminal.
2. Reuse the existing factor, `factor.order`, `free_ids`, `lambda_norm`,
   `RefinementWorkLedger`, `M1TrustRegionRefinementReason`, and refusal
   carrier.  Do not factor again or introduce a solver fallback.
3. Correction 1 uses ordinary binary64 `B^T(f+A*p0)+lambda*q0`, then the
   retained shifted inverse and free-only update to private p1.
4. Correction 2 uses accepted non-FMA Dot2 for the augmented residual and
   augmented gradient-plus-shift, then the same inverse and free-only update
   to p2.  `V` rows follow free positions; raw `V[:,j]`/`sigma[j]` joins stay
   raw; traversal occurs through `factor.order`.
5. p1 must retain active bits, finite/EFT-domain admission, original box,
   full/original-radius, and reduced/remaining-radius feasibility.  It cannot
   enter phase selection, physical/hydraulic evaluation, merit, materialization,
   receipt, or owner state.  p2 repeats these checks and additionally obeys
   the unchanged positive-lambda remaining-radius closeness predicate.
6. Then, and only then, pass p2 to unchanged all-coordinate BVLS-02 KKT and
   release logic.  Do not return p0/p1, clip, normalize, re-search lambda,
   relax tolerance, or perform correction 3.

`m1_trust_region_controller.rs` may only propagate/capture the existing typed
refinement refusal. `lib.rs` may expose cfg(test) seams. `m1_coupled_tests.rs`
owns real-controller isolation and downstream-state assertions. No other Rust
surface is proposed.

## Guard and error reservation

The runtime must preserve all earlier owning errors. Eligibility work is
attempted and charged before selection; an earlier owning failure retains its
kind. Once BVLS-04 is selected, existing
`M1TrustRegionRefinementReason` and `M1TrustRegionRefinementGuardStage` values
carry the exact source/EFT, factor, update, active, scratch, final-box and
final-radius/closeness mapping stated in numerical-methods.md. Add only
`ScratchBox`, its lower/upper stages, and named p0/scratch/final full/reduced/
closeness radius stages to distinguish actual owners in the same metadata
carrier. A p1 box refusal carries correction position First and coordinate. This has precedence over
later BVLS-02 KKT and all downstream physical paths. Work-cap denial retains
its owning `WorkCap` refusal before partial refinement. The existing zero-lambda
BVLS-03 scratch policy is not changed.

Original and remaining radius are different inputs. Recompute all three full
norms by ascending coordinates 0..20 followed by final square root and all
three reduced norms in free_ids slot order followed by final square root. For
p0/p1/p2, after active/finite and box checks, check full norm
against original radius before free norm against remaining face radius; use a
control where these values differ. For p2 then require nonnegative gap and
`0 <= remaining - reduced_norm <= max(2^-40*remaining, 64*epsilon*max(1,remaining))`.
Neither old `LambdaTrace` norms nor bracket endpoints certify corrected points.

Reserve the new branch before entry from the existing monotonic scalar/guard
ledger. The two-correction topology is `12,306 + 597m + 8m^2`; the complete
reservation is `K04(m)=12306+597m+8m^2+39816+1827+129+(6m+3)+4` and
`G04(m)=3*K04(m)+324+4m`. The latter itemizes at most `64+4m` selector/
structure guards, 252 active/finite/box scans, and 8 radius/gap comparisons;
it is a conservative upper bound.
At m=21, these are 70,276 scalar/upward operations and 211,236 guards; 43
BVLS-04-only faces require 3,021,868 and 9,083,148 respectively. Mixed
BVLS-03/BVLS-04 faces remain bounded by existing BVLS-03 43-face maxima
4,118,712 and 12,367,101, since BVLS-03 per-face reservations are larger.
KKT/enclosure/norm work is separately charged and overlapping whole-solve
observations are never summed as a unique total. No cap increase is proposed.

## Review gates after this preparation

Correctness must independently reconstruct the captured face and inspect
operation order, factor joins, p1/p2 two-radius policy, error precedence, and
reservation arithmetic. QA must inspect source identity/custody, scope, test
discriminators, no-extra-work claims, and exact body/replay staging. Both must
approve the amendment and controls before the parent releases any Rust body.

After body controls and source/manifest review, the sole proposed physical
endpoint is the already named non-target domain-refusal control. It must retain
its original trial setup and only proves shifted-path proposal followed by the
intended domain refusal/shrink/restoration. A canonical numerical refusal first
is terminal evidence, not a retry trigger.

The current observation JSON has a historical
`unreached_after_Stage1_typed_refusal` literal. Its narrowly authorized recorder
correction must instead emit the actual `Result` milestone/status while retaining
the old capture bytes, setup, forcing, seed, mutation, Stage1-then-injection
order, and assertions. Time actual proposal entry-to-return with monotonic wall
and available process CPU by reusing the existing `Instant`, `/proc/self/stat`
fields 14/15, and `getconf CLK_TCK` convention. Preserve a zero CPU tick as
below resolution and do not retry. Keep setup, build and observation
serialization out of the interval where possible. Reuse the work observation to
record actual positive-refinement entries, factor applications, Dot2 residual
rows/free gradients, including refused/discarded work. This endpoint is
control-path cost, not simulation or throughput evidence.
