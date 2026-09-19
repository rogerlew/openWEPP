# Grid40 baseline comparator correctness review

Reviewer: `/root/grid40_correctness` (Sol/high), independent of the method,
comparator and detached implementation authors.

Reviewed comparator SHA-256:
`beaf885258b495437a138cfc59695cee128e65799214deeced7d61ed15e7a655`.
Reviewed input-bit manifest SHA-256:
`67ae40339f8088bcc288905aabf5a6d375c7174f48c637803a8dd3c2a21b3314`.

Static: inspected the complete comparator, frozen trace structure, input-bit
manifest, current package scope and detached solver call sites needed to assess
the comparison. The comparator hashes the frozen reference, removes only the
three historical `diagnostic_directional_probe` records from that reference,
forbids that kind in the new baseline, requires the exact top-level key set and
schema, and recursively compares the complete remaining record list. Object
keys, list lengths/order, scalar types and values are exact; binary64 values use
packed-bit equality, including signed zero. The actual record list is neither
filtered nor extended by an allowance mechanism. New baseline metadata must
therefore remain in a separately checked sidecar.

Ran: a read-only independent traversal of the frozen trace's `solver_input`
and `initial_trial` found exactly 311 binary64 paths. The manifest has exactly
the same 311 paths and bits, with zero missing, extra or mismatched entries and
an exact redundant 29-coordinate `initial_trial_bits` vector. Neither structure
contains integer-valued JSON numbers. This command decoded evidence only; it did
not invoke a Rust test, model, solver or evaluator.

Inspected retained Ran evidence: `comparison-self-check.json` records ten full
CLI synthetic-copy cases. Normal and optimized (`python -O`) modes each accept
the exact ordinary-record copy and reject a changed float, added field, added
record and changed schema. Raw stdout/stderr and synthetic copies remain in
`/home/roger/openwepp-experiments/b01-wb14-grid40-rootchecks-20260919`.

## Findings and fix verification

- **High, `G40-CMP-1`, resolved:** the original comparator used Python
  `assert` for every gate, so optimized execution could disable validation and
  still emit PASS. The reviewed cut uses explicit `require`/`ValueError` checks
  throughout. Allowing optimized execution is safe because no admission check
  depends on `assert`; the normal/optimized negative CLI evidence confirms the
  corrected behavior.
- **Medium, `G40-CMP-2`, resolved:** the original `--allow-added-path` checked
  only path names and `--allow-added-kind` discarded whole records, permitting
  unchecked parallel arithmetic, policy, identity or cost claims. Both options
  and all actual-record filtering are removed. Recursive exact comparison now
  rejects every added field or record. Experimental metadata is assigned to a
  sidecar and cannot affect baseline equality.
- **Medium, `G40-CMP-3`, resolved:** the original comparator ignored the actual
  top-level schema and key set. The reviewed cut requires equality with the
  frozen document's top-level keys and compares the schema exactly; retained
  normal and optimized schema-mutation cases reject.

No unresolved comparator correctness finding remains. **Comparator verdict:
GO** for later use on the single authorized baseline trace. This verdict does
not approve the detached Rust implementation, its confinement/cost recorder,
either result-bearing arm, or any scientific outcome. A real baseline must
still run on the frozen source/input/protocol and this exact comparator cut;
only its resulting receipt can establish correspondence.

## Independent post-pair reconstruction scope

For both arms I will independently count bases, assemblies, signed probes,
witness and strict-search evaluations, `covered_trial_is_valid` entries,
installed updates and terminal records from raw events rather than accepting a
producer summary. Input and initial-coordinate bits will be checked against the
manifest and the baseline ordinary records against this comparator.

For every installed treatment update I will reconstruct
`x_trial = x_base + 2^-b * delta`, check coordinate bits at the next base, apply
the declared 29-coordinate bounds, recompute the normalized infinity norms and
verify that the installed factor is the first domain-valid, completely evaluated
strict decrease. The first treatment-only factor and terminal iteration receive
the decisive linear check: exact `rhs = -r`, independent matrix infinity norm
and pivot posture, plus a high-precision backward residual for
`J * delta = rhs`.

If treatment refuses, I will reconstruct each terminal b0..40 factor from the
base and direction, independently classify its coordinate domain, recompute the
norm for every completed evaluation, prove no admissible strict decrease was
skipped, and reconcile the typed obstruction and failure-count contribution.
An Error or resource stop receives the analogous first-denied/first-error and
counter reconstruction and remains incomplete where the contract says so.

If treatment returns Accepted, I will identify the actual acceptance path. For
ordinary acceptance I will verify all finite normalized residuals are at most
one, the last installed governed steps pass, and the candidate solution and
evaluation are the current base. For no-update acceptance I will additionally
verify a passing current residual vector, the exact full-trial refusal, the
first domain-valid b1..20 witness, all governed prospective norms and absence of
trial installation. In either case I will reconstruct raw/tolerance/normalized
relations and decisive physical rows from primitive fields: represented-snow
ground/soil identity rows, shared heat and vapor from canopy/lower-boundary/
reference terms, and any dominant occupancy energy or water row from its signed
component operands. Candidate-construction water/domain guards will be checked
from the retained endpoint rather than a producer PASS flag.

Inspection failures in this comparator review: **0**.
