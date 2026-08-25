# Terminal rejected-trial diagnostic-authority mini-gate

Status: `IN REVIEW / EVIDENCE-ONLY / NO IMPLEMENTATION AUTHORITY`

Objective: retain the already computed, immutable component receipts from
admitted rejected terminal trials so the 1.875-second blocker and estimator
effectivity matrix can be evaluated. This gate cannot change temporal
acceptance, state, physics, output, the `600 ms` floor or production APIs.

## Allowed evidence seam

After an admitted carrier/hydrology trial has returned and before its immutable
local result is discarded, a `cfg(test)`-only observer may receive a deep clone
of existing typed trial evidence. The observer:

- has input-only `&RejectedTerminalTrialEvidenceV1` semantics;
- returns `()` and cannot return failure/control information;
- is compiled out of non-test builds;
- cannot be installed by production configuration or public API;
- cannot mutate a candidate, joint, owner, clock, cache, provider, receipt,
  parcel, WB14 cursor, output or `last_*` field;
- runs only after the normal trial result/failure decision is fixed;
- is not called for pre-provider identity/domain rejection;
- cannot suppress, replace or translate `BelowCarrierDomain`.

If private access cannot be achieved without editing the production temporal
control path, this mini-gate remains `HOLD`; duplicating the solver in a test
harness is not equivalent evidence.

## Evidence record

The read-only record contains exact support, role/attempt/coupling ordinals,
beginning owner/joint/forcing/topology digests, provider-call ordinal and LSE
admission; ordered prescribed amount, rate/component, generated amount,
snow--soil, hydrology and ending-joint receipts; typed beginning/ending ice,
liquid, cold content and all component/complete/unallocated energies; physical
active-set tag; accepted/rejected decision and scaled error. Canonical digest
uses closed tag `terminal-rejected-trial-evidence-v1`, schema `u32(1)`, then
the fields in this sentence order with the standard framed primitive encoding.

The observer stores evidence only in test-local memory. It publishes no model
output or durable/restart state. Tests may serialize an artifact under this
package only after independently reconstructing its digest and redacting no
physical field.

## Required noninterference gates

1. The original focused command still returns exactly
   `Stage3(TerminalNumerics(BelowCarrierDomain))`.
2. Observer absent versus present produces byte-identical caller owners,
   attachment state, clock, provider cursor/call sequence, errors and outputs.
3. Observer panic/failure injection is caught by the test harness outside the
   physical transaction and cannot be converted into a model result.
4. Source guard proves the seam is `cfg(test)`, private, input-only and absent
   from non-test symbols/API.
5. Captured calls prove all positive supports are `>=600000000 ns` and zero
   zero calls occur below the floor.
6. The 1.875-second full and two 0.9375-second trials retain every component
   receipt/value and independently reproduce the known
   `27.2131278332233 J m^-2` difference.
7. No acceptance/controller/high-state code is added in this mini-gate.

## Write set and stop condition

Prospective bounded write set after two `GO-to-evidence` reviews: one new
test-only diagnostic module, the smallest `cfg(test)` private observer seam
needed to expose the existing immutable result, one source-guard integration
test and this package's evidence artifacts. Production temporal/Batch/event/
receiver/restart behavior and public APIs are excluded.

Two independent reviews are required before any seam implementation:

1. numerical/evidence review: evidence completeness and non-perturbation;
2. Rust/custody review: test-only reachability, mutation/control-flow/API
   exclusion and rollback.

Either HOLD stops before diagnostic implementation. Two GO-to-evidence results
authorize only the bounded evidence seam and capture, not v21 implementation.
