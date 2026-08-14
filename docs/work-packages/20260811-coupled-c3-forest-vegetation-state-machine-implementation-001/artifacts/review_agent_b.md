# Review Agent B: Independent Science And Closure Review

Status: `historical Review-B FAIL preserved / current bounded E19 QA PASS`

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
