# Review Agent B: Independent Science And Closure Review

Status: `FAIL — closure-blocking findings; package must remain HOLD`

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
