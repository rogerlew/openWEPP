# COLD-CANOPY-M1: correct the C6 oracle, then attempt the remaining frozen sweep

## Adoption

This is a proposal until the owner invokes it. Invocation authorizes the narrow
C6 correction, one additional C6-only replay, and the remaining original finite
measurements below. Authoring this prompt does not authorize execution.
Use `docs/work-packages/20260920-cold-canopy-m1-001/package.md` as the sole maintained
execution record; incorporate this adopted delta there. The original
`artifacts/separate-accuracy-owner-authorization.md` remains the A/B protocol.
The preceding control-repair envelope remains binding except for the explicit
C6 replay and UTC-window renewal below. Do not ask again about already authorized
relaxed accuracy or routine in-scope corrections.

## Starting state and failure disposition

Start with preservation commit `5920da7fa846e48dee993b86ef3bdd0629e883fd`, plus the
package's subsequent C6 disposition/prompt-authoring record. Verify actual state;
preserve unrelated work, historical sources, failed results and HALTED runners.
No branch change, push, main Rust adoption or production integration.

C1-C4 passed; C5 passed its sole corrected replay. C6 failed its first attempt
because its expected lower wet fraction used beginning mass 0.018 kg/m2 instead
of current trial mass 0.023 kg/m2. Independent terminal correctness establishes
expected-from-current bits `0x3fc74163587b03b8`; the incorrect assertion uses
`0x3fc3bfd8fa998876`. This is a test-oracle provenance error, not established
physics/solver failure. C6's later core/capacity/gap/certificate/hold/counter/rank
assertions have not passed. Both earlier replay slots remain consumed.

Recover source tree
`9ce31f5e75721b45af936108824435936ecae3e9dbd072075a3f4a472aae8968`
from `artifacts/near-bound-execution01-replay-source.tar.gz` and its JSON receipt.
Authenticate before recovering into a new isolated mutable directory. Preserve
fixture hash `d3616640db3e7c689162fcf5fa0e71c76ecd9b068beb070cacf3157fc97d8362`.
The old binary `e18b99e4bcc3cf3a280315f56f2eab57e02e98906460c5e4dd0e38fdb86c53e6`
is failed-source evidence, not the corrected binary.

## Narrow correction and readiness

Correct only C6 assertion expectations/operand provenance and minimal necessary
runner/custody metadata. Keep beginning mass 0.018 and current trial mass 0.023;
do not change either input to match the old assertion. Independently derive the
wetness expectation from frozen current operands and the canonical expression,
including evaluation order/rounding. Do not copy observed output as the oracle.

Before executable replay, statically audit every remaining C6 assertion, not
just the first failed line. For each expectation identify beginning versus
current state, upper versus lower occupancy, physical versus mapped matrix,
raw zero certificate versus hold eligibility, and real physical coverage versus
the separate synthetic rank-refusal seam. Independently verify phase/capacity,
wetness, gap/ratio, certificates, held/free state, work counters and typed rank
refusal. Preserve semantics and specificity. Any further demonstrable C6-only
oracle error may be corrected before this single freeze; unsupported predicates
remain blockers, not guessed literals. No executable core/Jacobian preflight.

The previous review missed this beginning/current distinction. Reviewer-owned
fix verification must inspect the operand provenance and all previously unreached
assertions explicitly. Reuse unchanged accepted review portions. Astra orchestrates
one implementer and distinct correctness and QA reviewers under repository routes,
at most two concurrent children, no nested delegation. Keep numerical bodies,
solver/derivatives, tolerances, forcing, cadence, reference strategy, fixtures and
scientific authority unchanged. If the required correction exceeds this scope,
stop with the precise prerequisite.

Run focused format and release no-run build, source-bearing inherited Clippy
comparison, source recovery and fresh build/tool custody checks. No old binary
receipt may be assigned to changed source. No full-workspace/production pass is
implied. Freeze the reviewed source, compiled fixture, binary and dispatcher;
require explicit approved receipt status and exact hashes before dispatch.

## Finite budget and replay

Propose **no additional charged seconds**. Retain cumulative ceiling
**458534.927980 seconds**. Recover the latest actual/conservative carry from
package.md, including this prompt's authoring and previous return tails; never
restart from an older smaller sample. Separate inactive owner-decision time.
At first execution work, including reading, compute remaining balance R once.
If R <= 900 seconds, preserve and return without launch. Otherwise anchor a
single UTC hard deadline at start + R and work cutoff 900 seconds earlier.
Invocation explicitly renews the former 21:06/21:21 UTC boundaries this way;
it does not add allowance. Charge all active work/waits/concurrency once.
Readiness is capped at 600 charged seconds within R. Commands remain <=180 seconds
and must fit the remaining work window. No automatic deadline extension.
This authorizes bounded execution in fixed order, not guaranteed completion of
all cells. Before replay, report time remaining after readiness and closing
reserve; require enough for the full remaining C6 cap plus custody/collection.
Then continue only while the original per-operation, aggregate and UTC caps
permit. The aggregate physical limit bounds the sum; individual maxima are not
additional allowances. If a budget ends the schedule, explicitly list every
missing cell and retain an incomplete disposition. Do not obtain a separate
renewal merely because a worst-case sum exceeds the aggregate cap.

Authorize exactly **one additional result-bearing invocation of C6**, after
reviewed repair. Its original failed 0.059973032 seconds remains charged, so
its remaining control allowance is **29.940026968 seconds**. Any executable
C6/core/Jacobian preflight consumes this slot. No further corrected replay of
any operation is granted. A new assertion/integrity failure stops execution;
valid numerical/domain negatives, timeouts and noisy timing are not retried.

Carry continuation physical elapsed **0.410551444 seconds** under the existing
1800-second aggregate cap (remaining 1799.589448556); reference subcap remains
900 seconds with zero used. Keep historical control 0.053829565 seconds separate
and preserved. New run identity must link both historical consumed replays and
this new C6-only authorization. Never reset old HALTED state/counters. Retain
failed C6 and all original charges in the new state. Reuse C1-C5 results only
with independently verified unchanged dependencies; if they are invalidated,
stop because this authorization does not permit their reruns.

## Measurements and closure

Only after C6 passes, execute the frozen order: three strict FULL/P0 references
and three predeclared stability checks, then 24 A cells and 18 additional B cells.
Use the existing balanced order, three cases, P0-P3 profiles, fixed cadences,
fresh initialization and within-day M/H continuity. References/stability each
have 150 seconds; each candidate has 60 seconds including its diagnostic and
six timing batches of four fresh complete days. Reuse matching 60-second A
baselines; B is unavailable when its frozen baseline fails. No added cases,
adaptive search, method switches, fallback or input retuning. No heavy concurrent
builds/tests during timing.

Reference negatives leave accuracy unresolved but need not block independently
safe scheduled candidates. Candidate negatives end that cell, not the sweep.
Preserve missing/failed cells and independent original-row/transfer reconstruction.
Retain CPU versus wall samples and chart/approximate-tangent attribution limits.
Targets remain 500 us CPU / 550 us wall per completed OFE-day; regime CPU ceilings
750/1500/2500 us; complete 10-OFE, 36,525-day century <=182.625 s CPU /210 s wall.
Control/proposal or column-only costs cannot establish whole-system deployment.

Report A reduction/tolerance and B cadence conclusions separately, including the
error/cost frontier or exact missing evidence, target gaps and regime coverage.
Preserve unresolved accuracy if references fail; do not adopt production error
budgets. Reconcile terminal diff, same-reviewer findings/fixes and actual charged
time. Commit scoped documentation/evidence and recoverable source locally; no
push. Stop after this finite investigation. Failed acceptance remains INCOMPLETE;
no additional method campaign or replay is implicit.
