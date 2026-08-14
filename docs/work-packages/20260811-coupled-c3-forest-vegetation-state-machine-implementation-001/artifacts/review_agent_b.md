# Review Agent B: Independent Science And Closure Review

Status: `historical Review-B FAIL preserved / Milestone 6 science implementation PASS / terminal closure HOLD`

Evidence mode: `Static + Ran`

Reviewed the current worktree against `SC-VEGETATION-001@5`,
`SC-BIOGEOCHEM-001@1`, the predecessor equation-authority, numerical-solver,
test-vector, and state-ownership/transaction ledgers, and the package's stated
E01--E22 and independent-closure acceptance criteria.

Ran on 2026-08-11:

- `cargo test -p openwepp-vegetation -p openwepp-biogeochemistry --tests` — PASS (two tests total).
- `cargo test --test vegetation_boundary_authority_contract` — PASS (seven tests).

Those passing tests do not establish the package claim: they exercise isolated
helpers, not the complete public candidate transaction or independent water,
energy, C, N, and dry-material closure.

## Findings

### B-CRITICAL-001 — The public candidate path does not execute E01--E22

`crates/openwepp-vegetation/src/transaction.rs:210` constructs water demand as
`VPD * root_fraction * dt * 1e-6` and `:224` constructs nitrogen demand from
direct PAR and a `1e-12` factor. Neither expression is an admitted equation.
The function never invokes radiation, FvCB/Medlyn, energy, four-potential
hydraulics, respiration/allocation, phenology, turnover, or material-transfer
logic. At `:242-243` it finalizes every resource as `min(request,
authorization)` without the required cap-active coupled re-solve; at `:244-275`
it changes only transaction IDs, LAI, T10, and liquid interception; and at
`:288-294` it reports zero solver work and five literal zero residuals.

This is prohibited proxy physics and tautological closure, not an exact
E01--E22 implementation. It violates `INV-VEGETATION-064` through
`INV-VEGETATION-072` and makes `science_implementation_status=IMPLEMENTED`
illegitimate.

Required disposition: `accepted`; replace the proxy transaction with the
contract-admitted full coupled candidate solve and independently reconstructed
ledgers before closure.

### B-CRITICAL-002 — E01 uses a different numerical algorithm and E02/E03 use an unauthorized shortcut

`crates/openwepp-vegetation/src/radiation.rs:29` explicitly claims a fixed-step
RK4 implementation; `:133-166` integrates the boundary-value ODE with exactly
4000 RK4 steps. The contract requires a real 2x2 matrix exponential plus the
analytic exponential particular solution, including exact resonance handling.
Separately, `sunlit_shaded` at `:255-267` substitutes a hard-coded diffuse
extinction coefficient `kd=0.8` and Beer-law absorption rather than consuming
the admitted two-stream band/direction solution. `traverse_column` therefore
propagates the unauthorized shortcut.

The single test at
`tests/integration/vegetation_boundary_authority_contract.rs:92-99` compares
one RK4-produced scalar and defines closure algebraically from the same
operands; it does not prove the required algorithm, sun/shade integral,
VIS/NIR-direction poison vectors, resonance, or independent energy closure.

Required disposition: `accepted`; implement the exact admitted solver and bind
independent digest-generated vectors for all E01--E03 branches.

### B-CRITICAL-003 — The coupled numerical system required by E11--E15 is absent

There is no Brent `ci` solve, damped Newton canopy energy solve, semismooth
hydraulic complementarity solve, or common/nested convergence check in the
vegetation crate. `crates/openwepp-vegetation/src/hydraulics.rs:20-46` computes
only a reduced root-layer flux from a supplied `psi_root`; it omits leaf/stem
paths, gravity/path lengths, vulnerability conductances, four continuity
equations, and authorization-cap complementarity. `:48-64` again returns
`min(request, authorization)`. `crates/openwepp-vegetation/src/energy.rs:68-96`
is only a scalar ledger helper and implements no wet/dry leaf/stem or canopy-air
nodes. Consequently no gas/energy/hydraulic transpiration equality,
nonconvergence diagnostics, alternate-initial-guess equivalence, or solver
rollback vector exists.

Required disposition: `accepted`; this blocks Milestones 3, 5, and 6.

### B-CRITICAL-004 — E16--E22 persistent state, ownership, and donor/receiver closure are not implemented

`crates/openwepp-vegetation/src/carbon_nitrogen.rs:46-180` contains disconnected
helpers only. Its allocation result collapses live/dead wood into aggregate
stem/root values and has no maintenance-reserve priority, six-tissue
display/storage split, retranslocation ordering, potential/final external-N
demand, phenology state transition, mortality ordering, donor debit, or
receiver proposal assembly. None of these helpers is called by
`execute_candidate`. The diagnostic passes an empty transfer list and an
execution-only constant carbon fraction at
`crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs:136-143`.
Thus the code cannot independently close carbon, nitrogen, or dry material and
does not implement `INV-VEGETATION-069/070` or `INV-BIOGEOCHEM-003`.

Required disposition: `accepted`; implement exact persistent pool transitions
and same-operand donor/receiver candidate receipts with independent C/N/DM
reconstruction.

### B-CRITICAL-005 — Mineral-N species identity and transaction accounting are wrong

`SC-BIOGEOCHEM-001` requires independent `(layer, NH4|NO3)` requests,
authorization, finalized use, and inventory. The boundary types alias the key
to a single `String` (`crates/openwepp-biogeochemistry/src/lib.rs:39-41`),
authorization aggregates only by that key (`:47-72`), and application debits
combined availability by consuming NH4 first and then NO3 (`:102-112`). The
vegetation path emits only one request per layer and never uses
`nh4_request_fraction` (`crates/openwepp-vegetation/src/transaction.rs:224-232`).
This violates species separation, can authorize against the wrong inventory,
and defeats the mandatory NH4/NO3 and wrong-layer poison vectors.

Required disposition: `accepted`; use a typed layer/species key and preserve
species through all three resource stages and candidate debit.

### B-HIGH-006 — The claimed independent closure and rollback evidence is absent

`CoupledCandidate` exposes no water/energy/C/N/dry-material ledger operands or
material proposals (`crates/openwepp-vegetation/src/transaction.rs:161-172`).
Commit accepts five producer-supplied residual scalars (`:345-360`), which the
producer currently fills with zeros, so the check is tautological. The
diagnostic has no hydrology owned state or candidate debit, and commits only
vegetation and BGC (`crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs:148-162`).
The injected failure occurs before either assignment, but there are no failure
injections at every computational phase and no byte-level serialization
comparison for all owners. Passing isolated tests therefore cannot satisfy the
package's anti-tautology, real closure, or rollback gates.

Required disposition: `accepted`; expose authoritative operands, reconstruct
each ledger outside the producer, include the water owner in the atomic commit
set, and add phase-by-phase byte-identical rollback and all-distinct poison
vectors.

### B-HIGH-007 — Guarding and temperature-response evaluation are incomplete

`crates/openwepp-vegetation/src/photosynthesis.rs:42-46` evaluates the peaked
response exponent as `T*S-Hd` without the required gas-constant/temperature
denominator of the admitted CLM response and without the selected stable
log-domain algorithm. More broadly, public primitive inputs often omit finite
checks (for example FvCB at `:83-100` and Medlyn at `:102-115`), allowing NaN
comparisons to bypass domain guards. The complete forcing object is also not
validated before use. These paths do not satisfy the contract's finite,
scale-aware, fail-closed numerical posture.

Required disposition: `accepted`; transcribe the exact digest-bound responses
and add exhaustive finite/domain/error-precedence vectors.

## Claim And Gate Legitimacy

The current package artifacts remain mostly queued, and no current artifact
provides E01--E22 oracle reproduction, independent closure, poison-vector,
rollback, or exact terminal-gate evidence. The narrow passing tests are useful
scaffold evidence only. They cannot support Milestone 2--6 completion,
`science_implementation_status=IMPLEMENTED`, implementation readiness, or a
complete-package disposition.

Review disposition: `FAIL / HOLD`. All seven findings are material and
undispositioned at review time; B-CRITICAL-001 through B-CRITICAL-005 are
closure-blocking scientific correctness defects.

## 2026-08-13 Fresh E19 QA Review

Evidence mode: `Static + Ran`

The historical Review-B failure above remains immutable. This fresh review is
limited to the bounded E19 ordering-remediation increment.

Initial disposition: code correction PASS, lifecycle evidence HOLD. QA
accepted the correctness review's aggregate-`Nused` finding and additionally
found stale test-count/line-count evidence plus a regression that proved only
`Nused>demand`, not the exact one-ULP adjacency. All findings were accepted.

Corrections:

- removed the aggregate `Nused>final_total_demand` overconstraint;
- asserted exact `rounded_nused.to_bits()==demand.to_bits()+1`;
- documented why canonical eta owns the bounded aggregate ratio;
- updated the gate history to 215/215 and preserved the initial review HOLD;
- refreshed formatted line-count governance to 2,214 lines.

Final disposition: `PASS`. No material QA finding remains. Neither canonical
contract orders final physiological demand below potential demand; immutable
potential requests, one authorization, `F<=A<=D`, receipt-bound eta/NSC, and
beginning-state immutability are covered. Fresh vegetation 215/215, formatting,
and diff hygiene passed. BGC debit, energy-owner completion, atomic commit,
heavy gates, runtime activation, and calibration remain explicitly unclaimed.

## 2026-08-13 V7 Increment 4A QA Review

Evidence mode: `Static + Ran`

### HIGH — Signed `XS_C` can reject a contract-valid vegetation candidate

`CoupledOwnedState::validate_for_model` treats `xs_c` as finite signed state and
does not impose a nonnegative bound
(`crates/openwepp-vegetation/src/transaction.rs:420-449`), consistent with the
canonical signed-state rule in
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:1605`.
The Increment 4A ledger builder includes that signed value in total vegetation
carbon (`crates/openwepp-vegetation/src/vegetation_candidate.rs:395-404`), but
the validator then requires both aggregate beginning and ending vegetation
carbon to be nonnegative
(`crates/openwepp-vegetation/src/vegetation_ledger.rs:74-89`). A finite,
otherwise valid state whose negative `XS_C` debt exceeds its positive pools is
therefore admitted by state/configuration validation and rejected only at the
new owner-ledger boundary. The current real fixture has negative `xs_c` but a
positive aggregate, so it does not cover this valid edge.

Required disposition: `accepted`. Preserve finite signed `XS_C` accounting
without imposing a new aggregate nonnegative domain, and add a focused valid
signed-debt candidate regression that proves closure and beginning-state byte
identity.

### HIGH — Ledger validation does not bind a unique stratum set or one whole-state identity

`validate_vegetation_ledgers` checks only equal vector cardinality and equality
within each zipped carbon/N/DM row
(`crates/openwepp-vegetation/src/vegetation_ledger.rs:63-73`). It does not reject
duplicate `stratum_id` values, prove the configured stratum set, or require one
transaction/beginning digest/ending digest across all rows. In addition,
`validate_sealed` does not bind ledger identities back to the candidate's
transaction, beginning digest, and ending-state digest
(`crates/openwepp-vegetation/src/vegetation_candidate.rs:82-100`). Consequently,
matching duplicate triplets can replace a missing configured stratum, and
individually matching rows from different transactions or whole-state digests
can pass the purported independent validator. The current builder happens to
emit configuration order from one state pair, but the validation boundary does
not independently prove that claim, and no duplicate/missing/mixed-identity
poison exercises it.

Required disposition: `accepted`. Bind validation to the expected configured
stratum set and candidate-wide transaction/beginning/ending identities, with
duplicate-stratum, missing-stratum, mixed-transaction, and mixed-digest poison
tests.

### MEDIUM — Real-candidate C/N/DM provenance is not independently exercised

The standalone ledger tests use a hand-authored internally closing fixture
(`crates/openwepp-vegetation/src/vegetation_ledger.rs:197-230`). The real
candidate test checks only ledger vector counts
(`crates/openwepp-vegetation/src/transaction.rs:1942-1944`); it does not
independently reconstruct each real ledger from beginning state, ending state,
finalized use, final carbon operands, and material proposals, nor verify exact
proposal C/N/DM copying from source amounts. The carbon-as-dry-matter and forged
aggregate poisons do exercise the arithmetic validator, but they do not prove
the production operand-lineage mapping asserted in the package evidence.

Required disposition: `accepted`. Add an outside-producer reconstruction over
the real candidate with all-distinct C, N, and dry-material operands and poison
the production mapping rather than only a synthetic aggregate fixture.

## Non-blocking Debt / Follow-ups

- Missing, potential-only, and duplicate occupancy results are directly covered
  at `crates/openwepp-vegetation/src/vegetation_candidate.rs:420-516`; the real
  fixture also covers canonical digest validation, exact shared/occupancy
  lineage, derived-area refresh, and serialized beginning-state identity at
  `crates/openwepp-vegetation/src/transaction.rs:1897-1967`.
- Unresolved beginning/candidate transfers have production guards at
  `crates/openwepp-vegetation/src/transaction.rs:817-824` and
  `crates/openwepp-vegetation/src/vegetation_candidate.rs:191-196`, but Increment
  4A adds only a positive empty-state assertion. Add a targeted rejection poison.
- Proposal IDs and sort order are asserted, but a second construction is not
  compared byte-for-byte and exact owner/C/N/DM/source-sequence preservation is
  not asserted. Add a repeated-construction and all-distinct source-order test.
- `artifacts/implementation-and-test-evidence.md:3` still labels E19 composition
  as the active status although the package and final-disposition headers have
  advanced to Increment 4A.
- Line-count evidence matches the reviewed bytes: `vegetation_candidate.rs`
  518, `vegetation_ledger.rs` 258, `persistent_phase.rs` 498,
  `transaction.rs` 2,049 (WARN with split follow-up), and
  `carbon_nitrogen.rs` 2,214 (retained WARN); no reviewed file reaches 3,000.

Ran on unchanged reviewed source bytes: vegetation quick 219/219;
implementation contract 13/13; vegetation authority contract 25/25; AUTH11
3/3; strict all-target vegetation Clippy; formatting; and diff hygiene all
passed. One attempted test-list command used an invalid output-format value;
the corrected list command succeeded and did not affect a gate result.

Public candidate publication, BGC receipt, energy ownership, atomic commit,
Milestone 4/5 completion, heavy gates, terminal verification, activation, and
calibration remain explicitly withheld. QA disposition: `HOLD` pending the
three accepted findings above.

## 2026-08-13 V7 Increment 4A Final QA Rereview

Evidence mode: `Static + Ran`

### Findings

No remaining material QA finding. The initial Increment 4A `HOLD` above remains
immutable historical evidence; all three accepted findings are corrected on
the current exact worktree bytes.

- Signed `XS_C` is now a separate finite signed beginning/ending operand while
  physical vegetation C remains nonnegative
  (`crates/openwepp-vegetation/src/vegetation_ledger.rs:21-32,105-134`). The
  closure consumes the directly retained final-maintenance operand rather than
  deriving respiration from ending `XS_C`
  (`crates/openwepp-vegetation/src/persistent_phase.rs:28-36,81-89,216-226,271-286`;
  `crates/openwepp-vegetation/src/vegetation_candidate.rs:374-386`). A negative
  physical-plus-reserve aggregate is accepted and ending-reserve corruption is
  rejected (`vegetation_ledger.rs:365-380`); the real two-ULP candidate also
  rejects ending-`XS_C` corruption (`transaction.rs:1993-2002`).
- Ledger validation now requires the exact configured stratum set, candidate
  transaction, beginning digest, ending digest, unique strata, row-consistent
  C/N/DM identities, and globally unique positive proposal IDs
  (`crates/openwepp-vegetation/src/vegetation_ledger.rs:61-159,179-223`).
  Duplicate/missing strata and mixed whole-state identity reject at
  `vegetation_ledger.rs:377-406`; the real candidate coherently forges all three
  ending-digest copies and still rejects against candidate identity at
  `transaction.rs:2003-2025`.
- The production two-ULP path now validates the actual candidate operands, not
  only a synthetic ledger. It poisons signed reserve, whole-state digest, and
  carbon-as-dry-matter while preserving the independent C/N/DM closure path
  (`crates/openwepp-vegetation/src/transaction.rs:1974-2036`). Direct final GPP,
  final maintenance, finalized N use, beginning/ending physical state, and
  proposal C/N/DM provenance are visibly bound in
  `vegetation_candidate.rs:336-415`.
- Candidate construction requires structural equality with the exact water
  phase retained by the nitrogen phase
  (`crates/openwepp-vegetation/src/persistent_phase.rs:38-48,70-74,289-302`;
  `crates/openwepp-vegetation/src/vegetation_candidate.rs:113-133`). Ending
  occupancy uses only capped results, requires the exact configured set, and
  advances lineage once (`vegetation_candidate.rs:244-287`).
- Material proposals retain deterministic typed stratum/donor/receiver/source
  ordering and consecutive positive IDs (`vegetation_candidate.rs:290-328`).
  The real fixture constructs the entire sealed candidate twice and proves
  exact structural equality, then asserts proposal order/IDs
  (`transaction.rs:1897-1973`). Serialized beginning state remains byte-
  identical after both constructions and real-candidate poisons
  (`transaction.rs:2037-2040`).

### Non-blocking Debt / Follow-ups

- Add one isolated cross-stratum duplicate-proposal-ID poison. The production
  validator already enforces global uniqueness and candidate construction
  generates one consecutive global sequence, but the present duplicate-stratum
  fixture reaches an earlier identity guard.
- The valid negative aggregate and real-candidate corruption tests jointly
  cover signed reserve semantics; a future multi-stratum full-candidate fixture
  with physical-plus-`XS_C` below zero would make that integration coverage more
  direct.
- `UncommittedNitrogenPhase` retains a boxed clone of the full source water
  phase, and the candidate also stores its own water-phase clone. This is exact
  and safe for the bounded private candidate, but a compact immutable phase
  identity should be considered before a public/performance-sensitive API.
- Line-count governance matches current bytes: `vegetation_candidate.rs` 540,
  `vegetation_ledger.rs` 408, `persistent_phase.rs` 508, `transaction.rs` 2,136,
  `carbon_nitrogen.rs` 2,214, and `migration.rs` 2,873. The three 2,000-line
  files remain WARN-level decomposition debt with recorded follow-up; none
  reaches the 3,000-line blocker.

Reviewed source SHA-256: `vegetation_candidate.rs`
`3a75f82039818ae01073cb8b770e3d8853936ab39ce2cbd944b8d6de4fd3751c`;
`vegetation_ledger.rs`
`45b5d66ca46feab1a587e05e75d8d82d3495eaafeaa2e49c3256814ac6816afe`;
`persistent_phase.rs`
`b9be5e28c81a971cd937b0657b2b3c2520534152d7d33b136bb1327afbcd16c1`;
`transaction.rs`
`5a8556ac0d53cef8ca252611e78c1e48499bc8e2266145ae9179e9a28249a6ef`;
and `lib.rs`
`1cdbf8de57972764cc13983b3dd8a4ecb57f382db8e1139d4d3207861189f436`.

Ran on those source bytes: vegetation quick 221/221; implementation contract
13/13; vegetation authority contract 25/25; AUTH11 3/3; strict all-target
vegetation Clippy; formatting; and diff hygiene all passed. Current retained
package evidence records Markdown lint over 55 files with 0 errors and 0
warnings. Counts, active Increment 4A lifecycle status, historical HOLDs, and
WARN-level line-count dispositions are truthful.

Public candidate publication, BGC receipt, independent energy ownership,
atomic commit, Milestone 4/5 completion, heavy gates, terminal verification,
activation, and calibration remain explicitly out of scope and fail-closed.
Final bounded Increment 4A QA disposition: `PASS`.

## 2026-08-13 V7 Increment 4A Superseding Final QA Addendum

Evidence mode: `Static + Ran`

### Findings

No material finding. This addendum supersedes the immediately preceding final
rereview for the exact final bytes; the original `HOLD` remains immutable
historical evidence.

- The new typed failures emit only the contract-authorized `VEG-E-093`,
  `VEG-E-097`, and `VEG-E-100` codes
  (`crates/openwepp-vegetation/src/error.rs:47-54`; `SC-VEGETATION-001.md:1574-1577,2186-2188`).
  Exact-variant assertions cover capped rollback, V7 allocation/closure
  rejection, and byte-preserving V7 transaction rollback.
- The real two-ULP path now rejects pairing its nitrogen phase with a distinct
  complete source-water phase
  (`crates/openwepp-vegetation/src/transaction.rs:1913-1933`). Independent
  ledger poisons reject a residual admitted only by the former loose envelope
  and a globally duplicated proposal ID across distinct strata
  (`crates/openwepp-vegetation/src/vegetation_ledger.rs:365-376,422-442`).
- Package evidence reports vegetation `223/223` and matching governed counts:
  `vegetation_candidate.rs` 535, `vegetation_ledger.rs` 443,
  `persistent_phase.rs` 508, `transaction.rs` 2,157,
  `carbon_nitrogen.rs` 2,214, and `migration.rs` 2,873. No file reaches the
  3,000-line blocker.

### Non-blocking Debt / Follow-ups

- The boxed full source-water-phase clone remains a performance/API refinement
  before any public path. The three 2,000-line modules retain their recorded
  WARN-level decomposition debt.
- A future multi-stratum full-candidate fixture with a negative
  physical-plus-signed-`XS_C` aggregate would make the existing unit-level
  semantic coverage more direct.

Reviewed source SHA-256: `vegetation_candidate.rs`
`daa882ccddfe071368279fd78cb83b1c913d806860dd4e0921b05f082f951a41`;
`vegetation_ledger.rs`
`336fcd53bdf726fedc4b5f12271d2e7cf5c87cca27af222f344a00b34098cd2e`;
`persistent_phase.rs`
`b9be5e28c81a971cd937b0657b2b3c2520534152d7d33b136bb1327afbcd16c1`;
`transaction.rs`
`f6c5035796057af12f5a1e303fc1d87de6a127055c910ae0c0271f9eefa0a24f`;
`error.rs`
`773f6d4c40f2b375efcc299d9a9c4e61c65d129a38b62e7c7f62ecefca0c5a51`;
`migration.rs`
`ab62ef22fe438547173462c863ded3d0b88cfda3810bbcbe525aa6d15ad23d45`;
and `lib.rs`
`1cdbf8de57972764cc13983b3dd8a4ecb57f382db8e1139d4d3207861189f436`.

Ran on those bytes: vegetation quick 223/223; implementation, vegetation
authority, and AUTH11 focused contracts 41/41; strict all-target vegetation
Clippy; formatting; and diff hygiene all passed. Package Markdown lint validated
55 files with 0 errors and 0 warnings.

Public candidate publication, BGC receipt, independent energy ownership,
atomic commit, Milestone 4/5 completion, heavy gates, terminal verification,
activation, and calibration remain out of scope and fail-closed. Superseding
final bounded Increment 4A QA disposition: `PASS`.
## Increment 4B / Milestone 5 Final QA Rereview

Evidence: `Static + Ran`

The first QA review returned HOLD for missing BGC admission, unvalidated prior
energy operands, missing behavioral cross-owner poisons, and stale lifecycle
evidence. Every finding was accepted and remediated.

Final exact-byte disposition: **PASS**, with no material finding. Final QA ran
722/722 affected quick tests, 15/15 implementation-contract tests, strict
affected-crate Clippy, formatting, diff hygiene, anti-evasion, 18-surface A0
admission, and package Markdown lint (55 files). Milestone 6 and campaign/
terminal closure remain pending.

## Milestone 6 Fresh Independent Science / Closure Review

Evidence mode: `Static + Ran`

### Findings

#### HIGH — Required benchmark and terminal closure evidence is not executed

The current production science bytes have no material defect in this review,
but the exact worktree cannot yet receive a Milestone 6 or package-closure
PASS. The package requires benchmarks, exact A1/A3 closure, Critical gates,
and two terminal verifiers
(`package.md:260-269,398-401`). The performance record still says
`benchmark execution pending post-review` and contains no command, hardware,
sample distribution, first-clean median, or result for any of its six required
surfaces (`artifacts/performance-budget.md:3-5`). Both terminal-verifier
artifacts remain `queued` and `not-run`
(`artifacts/verification_agent_a.md:3-7` and
`artifacts/verification_agent_b.md:3-7`). The recorded 2,664/2,664 full
workspace run is valid campaign evidence for the stable production bytes, but
it does not supply the missing benchmark results or the required two exact-byte
terminal dispositions.

Required disposition: `accepted`. Do not alter production for this finding.
Have the authorized comparator runner execute and record the required benchmark
and terminal A1/Critical commands, complete both independent exact-byte
verifications, reconcile the terminal diff, and then request a fresh closure
rereview.

### Original Review-B Findings Reassessed

- `B-CRITICAL-001` is closed. The public diagnostic calls the real sealed
  `execute_candidate_with_failure` path, which executes the potential and
  fixed-authorization capped column passes, persistent nitrogen phase, sealed
  vegetation candidate, BGC receiver, and independent energy owner. The old
  VPD/PAR proxy equations, `min(request, authorization)` finalization, and
  literal five-zero residual surface are absent and guarded by the public
  implementation contract.
- `B-CRITICAL-002` is closed. E01 uses the admitted real 2x2 matrix exponential
  and analytic particular solution; E02/E03 consume direction- and band-owned
  mixed-column absorption. Released radiation vectors and the Beer-Lambert,
  direction, VIS/NIR, rank, and boundary poisons execute through production
  Rust.
- `B-CRITICAL-003` is closed. The public path reaches Brent `ci`, coupled
  canopy-air/leaf/wet/stem energy, four-potential hydraulics, the uncapped
  nested solve, and V5 fixed-cap complementarity/generalized-Jacobian solve.
  Convergence diagnostics, alternate warm starts, complete typed rejected
  payloads, and rollback vectors are bound to the V3/V5/V6 fixtures.
- `B-CRITICAL-004` is closed. E16--E22 execute accepted gross/respiration
  aggregation, T10/maintenance/NSC allocation, exact V7 storage preparation,
  all-six-tissue onset, evergreen/deciduous phenology, ordered turnover and
  mortality, retranslocation, material proposals, and receiving-owner
  receipts without same-interval growth recycling.
- `B-CRITICAL-005` is closed. Mineral N retains typed
  `(SoilLayerId, Ammonium|Nitrate)` identity through request, proportional
  authorization, finalized use, inventory debit, and BGC validation. Separate
  layer/species and wrong-species poisons pass.
- `B-HIGH-006` is closed for production science. Water, component/stand energy,
  vegetation C, vegetation N, and dry material are reconstructed from exposed
  operands without accepting producer residuals. BGC separately reconstructs
  exact layer/species mineral debits and C/N/dry-material receiver credits.
  Twenty-seven injected or malformed-owner failures, including six real
  cross-owner identity/receipt poisons, preserve the serialized beginning
  vegetation, water, BGC, and energy state byte-for-byte.
- `B-HIGH-007` is closed. The admitted stable log-domain Arrhenius/peaked
  responses, cancellation-safe quadratic, finite/domain guards, and typed
  nonconvergence/error precedence replace the original response and NaN-prone
  scaffold.

### Current Science / Consumer Evidence

Static tracing confirms that `run_default_off_diagnostic_at_phase` constructs
all four owner candidates before one non-fallible whole-state assignment. The
real integration test invokes that public orchestrator API; no vegetation-only
commit method exists. The production selector negative test scans the runner
and hillslope direct-runtime source trees, and the exact diff from frozen base
`cd51fef9583f77973a2f4898864b9fe12b42545a` contains no runner,
`direct_runtime`, or runtime-input production change. The legacy PMET and
GSI/final-canopy routes therefore remain unchanged and cannot select V7.

Fresh exact-byte runs:

- science admission: `A0_ADMITTED contracts=45 science_surfaces=18`, authority
  SHA-256 `a73a905dbb85929561d1c55e442350429518e17705ddcd1ea95d65a71e9f6f0a`;
- authority anti-evasion: PASS; AUTH11: 3/3 PASS;
- vegetation authority / applicable A3 vectors: 25/25 PASS;
- public implementation, selector, poison, and rollback contract: 16/16 PASS;
- affected vegetation/BGC/hillslope quick suite: 722/722 PASS, including three
  known slow routing-oracle tests; and
- `git diff --check`: PASS.

Reviewed stable source SHA-256 includes diagnostic
`25a217cacc368e75d3bdd9d32d7af16b33d2fe6bf55b251ac3ec6fb418d404a7`,
energy owner
`d2ed144b5b648b6ce3d56c3735709bcefdaa34fcd5388fc2618891f3afe3d04f`,
BGC owner
`2fc0f2b2a2edf864155ebdf503cfb64f1495769e02539866669115e93598598a`,
sealed vegetation candidate
`19637e8f650334c31b2f2c6a550420f1f05fef8150c2a879037a0b6d8d29dfd8`,
and public integration contract
`ff7edb6886612d16fcb0c67c8b97f28b02ec6932e8fe2abd3344709634d84a78`.

### Non-blocking Debt / Follow-ups

- Reconcile historical/pending prose in the equation map, final disposition,
  milestone matrix, and gate narrative before terminal verification. These
  artifacts currently mix superseded checkpoint language with the active
  four-owner path.
- `transaction.rs`, `carbon_nitrogen.rs`, and `migration.rs` remain WARN-level
  decomposition debt. The boxed full source-water-phase clone is also an API
  and performance refinement before any production-facing consumer.
- Retain explicit exclusions: calibration remains
  `NOT_CALIBRATION_READY`; canopy snow, nonneutral/calm aerodynamics, soil
  transformations, real production-consumer integration, hydrology/LSE
  cutover, and runtime activation are not claimed by this package.

Science and default-off diagnostic implementation QA: **PASS**. Exact
Milestone 6/package closure disposition: **HOLD** solely for the missing
benchmark, terminal A1/Critical, and dual-verifier evidence above.

## Milestone 6 Central Arbitration and Taxonomy Focused QA Rereview

Evidence mode: `Static + Ran`

### Findings

No material science, closure, maintainability, or test-quality finding remains
in the reviewed remediation bytes.

- `V7-M6-A-001` is closed. The diagnostic water owner now calls kernel
  `authorize_proportionally_by` with `SoilLayerId` as the supply identity while
  the returned authorizations retain each request's complete occupancy/layer
  key, owner, transaction, and basis. The shared helper performs request-batch
  validation, deterministic ordering, compensated same-supply summation,
  full-supply branching, and proportional authorization. Its shared-layer
  vector proves exact `2:6` competition against supply `4` produces `1:3`
  while preserving the two distinct occupancy keys. Static search confirms the
  former diagnostic-local `supply / total` allocation formula is absent.
  Paths: `crates/openwepp-kernel-contract/src/lib_mod/resource_transaction.rs`
  and
  `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs`.
- `V7-M6-A-002` is closed. `VegetationError` now carries the canonical
  `VEGTXN-E-001`, `VEGTXN-E-002`, and `VEGTXN-E-003` categories. Water maps
  identity/correspondence, operand/basis/domain, and authorization/final-use
  bounds into those categories at both its typed boundary and shared-protocol
  validation seams. Nitrogen maps its typed protocol errors identically, and
  the production persistent phase preserves those mappings through request
  construction, authorization validation, and finalization. Focused tests bind
  all three rendered codes for both water and nitrogen; the water wrong-debit
  poison also binds the exact `ResourceBound` variant. Paths:
  `crates/openwepp-vegetation/src/error.rs`,
  `crates/openwepp-vegetation/src/occupancy_solver/resources.rs`,
  `crates/openwepp-vegetation/src/nitrogen_protocol.rs`,
  `crates/openwepp-vegetation/src/persistent_phase.rs`, and
  `crates/openwepp-vegetation/src/water_phase.rs`.
- No science or closure regression was found. The remediation changes the
  resource transaction seam and failure taxonomy, not the admitted E01--E22
  process equations, candidate isolation, four-owner validation order, or the
  final non-fallible whole-state replacement. The unchanged public integration
  contract continues to pass its real-consumer, selector, poison, and exact
  rollback vectors.

Fresh focused execution on the reviewed bytes:

- `cargo nextest run -p openwepp-kernel-contract --profile quick` -- 49/49
  passed;
- `cargo nextest run -p openwepp-vegetation --profile quick` -- 225/225
  passed;
- `cargo nextest run --test c3_vegetation_implementation_contract --profile
  quick` -- 16/16 passed;
- strict all-target Clippy for kernel-contract, vegetation, BGC, and hillslope
  orchestrator -- passed; and
- `cargo fmt --all -- --check` plus `git diff --check` -- passed.

### Non-blocking Debt / Follow-ups

- Benchmark execution, terminal A1/Critical gates, and dual exact-byte terminal
  verification remain separate pending closure evidence. This rereview neither
  executes nor substitutes for them.
- Retain the previously recorded WARN-level module decomposition and boxed
  water-phase clone follow-ups; neither is changed by this focused remediation.

Science/default-off diagnostic implementation QA: **PASS**. Exact Milestone 6
and package closure-evidence disposition: **HOLD** solely pending the separately
running benchmark and terminal verification workflow.

## Milestone 6 Final Taxonomy Architecture Exact-Byte QA Rereview

Evidence mode: `Static + Ran`

### Findings

No material science, closure, maintainability, or test-quality finding remains
in the final taxonomy bytes.

- The taxonomy authority is cohesive and contract-correct. Kernel
  `ResourceProtocolCategory` exhaustively classifies identity, operand, and
  bound violations once. The outer vegetation transaction converts those
  categories to `VEGTXN-E-001`, `VEGTXN-E-002`, and `VEGTXN-E-003`, matching
  `SC-VEGETATIONTRANSACTION-001`. The BGC owner deliberately retains its own
  public contract surface: invalid or mismatched protocol inputs render
  `BGC-E-001`, while authorization/use bounds, inventory overdraw, and material
  closure render `BGC-E-010`, matching `SC-BIOGEOCHEM-001`.
- Every reviewed public BGC `ResourceProtocolViolation` seam uses the same
  category adapter: proportional authorization, request-batch validation,
  request/authorization/final-use validation during candidate construction,
  and protocol revalidation on the sealed BGC candidate. Direct tests cover
  wrong basis, authorization greater than request, finalized use greater than
  authorization by one binary64 ULP, inventory overdraw, and material closure.
- No science or closure regression was found. The final delta changes error
  classification and test disposition only; same-snapshot arbitration,
  compensated proportional arithmetic, mineral inventory debit, material
  receiver accounting, candidate construction, and atomic owner replacement
  are unchanged.

Reviewed exact SHA-256 values are kernel resource transaction
`766a331f18aa83756ed42d2b960cc3324825da0abff41f6c096a152158db54cb`, BGC
`308c9f20c978deb009a1b160e0816afa81fc6e1a0012a9c33bfda6367e2ef6cc`,
vegetation error mapping
`00c611aa3a56d11165680ab2a715fc4f1c14904696204288b7f2110b2d666979`, and
public implementation contract
`ff7edb6886612d16fcb0c67c8b97f28b02ec6932e8fe2abd3344709634d84a78`.

Fresh focused execution on those bytes:

- kernel, vegetation, and BGC quick suites -- 281/281 passed (50, 225, and 6
  respectively);
- public implementation contract -- 16/16 passed;
- strict all-target Clippy for kernel-contract, vegetation, BGC, and hillslope
  orchestrator -- passed; and
- formatting and diff hygiene -- passed.

### Non-blocking Debt / Follow-ups

- Benchmark results, terminal A1/Critical gates, and dual exact-byte terminal
  verification remain separate external closure evidence. They are neither
  executed nor substituted by this taxonomy rereview.
- Previously recorded WARN-level decomposition and API/performance follow-ups
  are unchanged and do not block this implementation disposition.

Final science/default-off diagnostic implementation QA: **PASS**. Exact
Milestone 6 and package closure-evidence disposition: **HOLD** solely pending
the separately authorized benchmark and terminal verification workflow.

## 2026-08-14 Final Science / Closure QA Rereview

Evidence mode: `Static + Ran`

Disposition: `HOLD` for two closure-evidence defects. The exact implementation
science itself passes this review.

### Findings

#### HIGH — The benchmark matrix does not execute the required active water/N competition surface

The frozen package requires a benchmark for active water/N competition. The
retained matrix assigns that surface to
`v7_default_off_diagnostic_commits_all_owners_once_and_rolls_back_every_phase`,
but its exact `identity_rebound_v7_fixture` has one stratum, one tile, and one
root layer. It can produce only one water request for the layer and one mineral
request for each distinct `(layer,species)` supply key. Same-supply water or N
competition is therefore structurally impossible. The test also does not
assert positive, constrained, or competing water/N use.

The separate two-rank timing does not close this gap. It executes
`upper_cap_changes_final_release_received_by_descendant` through a
`ControlledCappedEvaluator`; it proves overlapping-column cap rerouting, but
does not invoke resource arbitration or a real mixed-stand constitutive solve.
The other four retained timing surfaces and their five corrected samples are
valid, and the two initial zero-filter attempts remain disclosed, but they do
not substitute for the missing competition surface.

Paths:
`docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/package.md`,
`docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/performance-budget.md`,
`tests/integration/c3_vegetation_implementation_contract.rs`, and
`crates/openwepp-vegetation/src/occupancy_solver/capped_pass.rs`.

Required correction: retain release-mode timing evidence for a fixture that
provably activates shared-supply water and mineral-N competition through the
real arbitration boundaries. The timed test must assert the competing owner/
key set and nontrivial authorization outcome, record the exact accepted argv
and raw warm/sample logs, and satisfy the prospectively frozen 2x budget.
Update the benchmark and gate artifacts without erasing the current valid or
failed attempts.

#### MEDIUM — The canonical finding-disposition artifact still says every original finding awaits rereview

`review-finding-disposition.md` retains a current header stating that the
public candidate is pending. Its current-status cells label all seven original
`B-CRITICAL-001..005` / `B-HIGH-006..007` findings as `re-review pending` and
all remediation-review findings as `repeat review pending`. Later appendices
and both review files demonstrate final GO/PASS, but the canonical disposition
table never reconciles those terminal results. This contradicts the package
exit criterion that no material finding remain undispositioned and makes the
terminal verifier input internally inconsistent.

Path:
`docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/review-finding-disposition.md`.

Required correction: append or otherwise add one unambiguous terminal
disposition that maps every original and remediation finding to its accepted
correction, final review result, and any genuinely non-blocking follow-up.
Preserve the historical review text and failed checkpoints.

### Passing Science And Closure Checks

- `B-CRITICAL-001` is closed. The public candidate executes the real potential
  and capped water phases, persistent phenology/C/N phase, and sealed
  vegetation candidate; the diagnostic then constructs water, BGC, and energy
  owner candidates before one envelope validation and one non-fallible whole-
  state replacement.
- `B-CRITICAL-002` is closed. E01 uses the real 2x2 matrix exponential and
  analytic particular/resonance solution; E02/E03 consume the band- and
  direction-owned mixed-column result. The RK4, fixed-`kd`, Beer proxy, VPD
  demand, PAR demand, and five-literal-zero scaffold patterns are absent.
- `B-CRITICAL-003` is closed. The production path reaches Brent `ci`, coupled
  leaf/wet/stem/canopy-air energy, four-potential hydraulics, potential and
  fixed-cap outer solves, cap complementarity, generalized derivatives, and
  complete typed nonconvergence diagnostics.
- `B-CRITICAL-004` is closed. E16--E22 retain accepted gross and respiration
  operands, T10 and reserve priority, final N-limited six-tissue allocation,
  evergreen/deciduous phenology, onset deployment, turnover/mortality,
  retranslocation, and transaction-scoped litter/CWD proposals and receipts.
- `B-CRITICAL-005` is closed. Typed transaction, owner, occupancy, layer,
  species, and basis identities persist through water and mineral-N request,
  authorization, finalized use, debit, receipt, and cross-owner validation.
- `B-HIGH-006` is closed for implementation science. Water, component/stand
  energy, physical plus signed-reserve carbon, vegetation and mineral
  nitrogen, and dry material are independently reconstructed from typed
  operands. No producer residual is accepted. The real 27-point phase/owner/
  malformed-envelope matrix compares the complete serialized four-owner
  beginning state byte-for-byte after rejection.
- `B-HIGH-007` is closed. Stable log-domain temperature responses,
  cancellation-safe roots, finite/domain guards, explicit unsupported
  aerodynamic branches, error precedence, and solver failure payloads replace
  the original incomplete guards.
- The E01--E22 equation map is connected to the same public diagnostic call
  exercised by the 16-test implementation contract. The 25-test authority
  suite binds the admitted V2/V3/V5/V6/V7 vectors, identity transitions,
  poisons, exclusions, and transaction authority.
- The default-off diagnostic is real but intentionally nonselectable. Static
  diff and recursive source scans find no diagnostic entry point, V7 selector,
  or vegetation candidate execution in runner or hillslope direct-runtime
  Rust. No runner, `direct_runtime`, `runtime_inputs.rs`, model-definition, or
  canonical science-contract path changed; legacy PMET/GSI-final-canopy
  behavior is unchanged.
- Exclusions remain truthful: no activation, production-consumer cutover,
  calibration/validation/transferability, canopy snow, calm/nonneutral
  fallback, or soil transformation is claimed. The empty stand follows the
  real zero-demand owner path, while required BGC transformations fail typed.

### Gate And Governance Legitimacy

The delegated exact-worktree heavy campaign is legitimate for the source
bytes recorded in terminal reconciliation: workspace strict Clippy passed;
the full profile passed 2,670/2,670 with 33 declared skips; workspace doctest
invocation passed; `cargo deny check` exited zero with only the retained
unmatched-MIT-0 warning; formatting and diff hygiene passed. Environment,
commands, exit codes, timings, and raw outputs are retained, and final source
hashes match the terminal reconciliation table.

Fresh review execution passed science admission with 45 contracts and 21
surfaces, authority anti-evasion, the two affected SC unit checks, and the
combined vegetation-authority/AUTH11/implementation targets at 44/44. One
initial unit-lint invocation used unsupported positional arguments, and one
overbroad correction exposed 148 unrelated catalog findings; the correctly
scoped `--path` runs for `SC-VEGETATION-001` and `SC-BIOGEOCHEM-001` each
passed. Previously run exact-source kernel/vegetation/BGC suites remain
50/225/6 and strict affected Clippy remains passing.

Exact line counts match the terminal artifact: `migration.rs` 2,873,
`occupancy_solver/constitutive.rs` 2,790, `carbon_nitrogen.rs` 2,214,
`transaction.rs` 2,082, energy owner 1,232, implementation contract 1,106,
BGC 827, and diagnostic 484. No Rust file reaches the mandatory 3,000-line
split threshold. The four 2,000-line WARN modules remain visible, accepted
decomposition debt.

### Non-blocking Debt / Follow-ups

- Dual terminal verifier artifacts are intentionally queued after final review
  and are not counted as an implementation defect in this disposition. They
  must not approve or archive the kickoff prompt until the two blockers above
  are corrected on their exact input bytes.
- Calibration remains `NOT_CALIBRATION_READY` and identifiability remains
  `NOT_ASSESSED`. The queued calibration, real-consumer, security, and worker-
  handoff placeholders should be reconciled with the package's explicit
  exclusions and terminal lifecycle wording, but they do not change the two
  concrete blockers or the passing implementation-science result above.

Final science/default-off diagnostic implementation QA: **PASS**. Readiness to
proceed to dual terminal verification and exact package closure: **HOLD** until
the required active-competition benchmark and canonical finding-disposition
reconciliation are present on the reviewed bytes.

## 2026-08-14 Superseding Final Science / Closure QA Rereview

Evidence mode: `Static + Ran`

Disposition: `PASS`. The two closure-evidence defects in the immediately
preceding review are corrected on the exact bytes below. No material science,
closure, maintainability, test-quality, benchmark, or line-governance finding
remains.

### Findings

No blocking or material finding.

- The former HIGH benchmark defect is closed. The authoritative matrix in
  `artifacts/m6-benchmark-final-20260814-20260814004247/` executes five exact
  release-mode commands. Each has one warm run, five retained samples, one
  identified test per run, an accurate command record, and a passing zero-test
  guard. Strict configuration parsing/canonical hashing, complete-state
  parsing and identity, two-rank radiation, the sealed public candidate with
  its independent energy owner, and the real scarce mixed-stand diagnostic all
  execute. The largest maximum-to-first-sample ratio is 1.02174 against the
  prospectively frozen 2x budget.
- The corrected scarce fixture contains two configured strata and occupancies
  on the same tile and root layer. Through
  `run_default_off_diagnostic_at_phase` it produces exactly two positive
  partial water authorizations and four positive partial NH4/NO3
  authorizations, positive bounded finalized use, owner transaction-lineage
  advancement, and byte-identical rollback at `BeforeCommit`. This is the
  real production constitutive candidate and four-owner diagnostic boundary,
  not the former controlled capped evaluator or abundant single-occupancy
  proxy. The rejected first matrix and both zero-filter mistakes remain
  historical evidence rather than being relabeled.
- The former MEDIUM disposition defect is closed.
  `artifacts/review-finding-disposition.md` now identifies the artifact as a
  terminal candidate, maps every original B and remediation-review finding to
  its accepted correction and final GO/PASS, and explicitly labels the old
  benchmark-pending statement as a superseded checkpoint. Historical HOLD and
  pending rows remain preserved as chronology; they are not current
  undispositioned findings.
- Every original Review-B finding remains closed on unchanged science-bearing
  bytes. `B-CRITICAL-001` reaches E01--E22 through the real public candidate;
  `B-CRITICAL-002` retains exact matrix-exponential/analytic radiation;
  `B-CRITICAL-003` retains Brent, coupled multi-node energy, four-potential
  hydraulics, cap complementarity, and typed failures; `B-CRITICAL-004`
  retains six-tissue C/N allocation, phenology, turnover, mortality, and
  material transfers; `B-CRITICAL-005` retains typed transaction, owner,
  occupancy, layer, species, and basis identity through debit and receipt;
  `B-HIGH-006` retains independent water, energy, carbon, nitrogen, and dry-
  material reconstruction plus all-owner rollback; and `B-HIGH-007` retains
  stable numerical evaluation, finite/domain guards, tolerances, and error
  precedence.
- The public closure path remains one potential water pass, one centralized
  compensated proportional water arbitration, one authorization-capped pass,
  one typed mineral-N arbitration, persistent E16--E22 finalization, sealed
  vegetation/water/BGC/energy owner candidates, complete cross-owner
  validation, and one non-fallible whole-state replacement. The unchanged
  27-point injected and malformed-owner matrix compares the entire serialized
  four-owner beginning state after every rejection. No producer residual is
  accepted as any of the five independent ledgers.
- Water and nitrogen identity remain exact. Water keys retain occupancy and
  layer; mineral-N keys retain layer and NH4/NO3 species; requests,
  authorizations, finalized uses, owner candidates, debits, and receipts bind
  transaction, owner, key, and amount basis. Deterministic owner/key ordering,
  compensated shared-supply totals, full key retention, authorization/use
  bounds, inventory closure, duplicate rejection, and species-preserving
  receipts remain covered by behavioral poisons.
- The diagnostic remains a real default-off consumer but is not selectable in
  production. Exact diff and recursive source checks contain no runner,
  hillslope `direct_runtime`, `runtime_inputs.rs`, diagnostic selector, V7
  selector, or legacy PMET/GSI-final-canopy route change. Empty stands use the
  real zero-demand path; required BGC transformations fail typed. Activation,
  real-consumer cutover, calibration, empirical validation, transferability,
  canopy snow, nonneutral/calm fallback, and soil transformations remain
  explicitly excluded.
- The authoritative exact-head Critical campaign is
  `artifacts/m6-heavy-short-final-20260814005156/`. Workspace strict Clippy
  passed; full nextest passed 2,671/2,671 with 34 slow and 33 skipped (run ID
  `471dafdc-4948-436f-8201-63fd4ad7326f`); workspace doctest invocation,
  `cargo deny check`, formatting, and diff hygiene passed. The nonfatal
  unmatched `MIT-0` allowance warning is retained. Commands, environment,
  exit codes, raw stdout/stderr, timings, and the corrected JSON summary agree.
  The earlier 2,670-test PASS predates the new test, and the first current-byte
  attempt failed because its TMPDIR exceeded Unix socket `SUN_LEN`; both are
  truthfully retained and neither substitutes for the clean 17-character
  `/tmp/owm6f-lQkG1z` run.
- Current exact implementation identities include kernel resource transaction
  `766a331f...54cb`, vegetation transaction `ebc08804...fa4b1`, BGC owner
  `308c9f20...f6cc`, energy owner `d2ed144b...d04f`, diagnostic
  `b73b5283...bc51`, and public contract `80cb4a3e...2bc3`. Science admission
  remains 45 contracts and 21 surfaces; anti-evasion, affected SC unit checks,
  restored authority, implementation, and AUTH11 evidence remain passing.
- Exact line counts remain below the 3,000-line mandatory split threshold:
  migration 2,873, constitutive 2,790, carbon/nitrogen 2,214, transaction
  2,082, energy owner 1,232, implementation contract 1,216, BGC 827, and
  diagnostic 503. The four 2,000-line WARN modules remain visible accepted
  decomposition debt; no numerical ordering, serialization, or API was
  distorted to reduce their counts.

### Non-blocking Debt / Follow-ups

- Future heavy runners should keep `TMPDIR` short enough for Unix-domain
  socket fixtures and parse Cargo stderr when constructing summaries. The
  retained failure and corrected post-processing make this developer-
  ergonomics issue non-blocking for the present campaign.
- The four WARN-level modules should be decomposed along existing production/
  test or kernel boundaries when a later authorized package changes them.
- Calibration remains `NOT_CALIBRATION_READY`; identifiability and all explicit
  scientific exclusions remain follow-up work rather than claims of this
  package.
- Dual terminal verifiers and verifier-authorized prompt archival remain the
  next lifecycle step. Their intentionally queued absence is not an
  implementation or QA defect and this review does not pre-approve them.

Final science/default-off diagnostic implementation QA: **PASS**. Readiness to
proceed to dual terminal verification on these exact bytes: **PASS**. No exact
blocker remains in this review scope.
