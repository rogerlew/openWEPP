# Review Agent B Remediation Re-review: Science And Closure

Status: `FAIL — material science and closure findings remain`

Evidence mode: `Static + Ran`

Reviewed the current exact worktree on 2026-08-11 against
`SC-VEGETATION-001@5`, `SC-BIOGEOCHEM-001@1`, the digest-bound equation,
numerical-solver, parameter, state-ownership/transaction, and test-vector
authority artifacts, the historical `review_agent_b.md`, and the current
finding disposition. This review traced the actual public
`execute_candidate_with_failure` path and the default-off diagnostic; it did
not accept artifact claims or the existence of helpers as implementation
evidence.

Ran:

- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick` — PASS, 12/12.
- `cargo nextest run --test c3_vegetation_implementation_contract --profile quick` — PASS, 10/10.
- `cargo nextest run -p openwepp-vegetation --profile quick` — PASS, 2/2.
- `cargo nextest run -p openwepp-biogeochemistry --profile quick` — PASS, 3/3.
- `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick` — interrupted after 95 seconds because three unrelated long-running Iwagaki tests remained; 487 tests had passed. This is not a package gate result and is not treated as a product failure.

The A0 authority suite is restored, the former VPD/PAR proxy transaction and
literal residual array are absent, typed layer/species resource keys are in
production, the radiation primitive now uses a matrix exponential rather than
fixed RK4, and the peaked temperature-response expression is corrected. Those
are real improvements. They do not close the following defects.

## Material Findings

### RB-CRITICAL-001 — Plant area is consumed as photosynthetic leaf area and dry-stem shortwave is discarded

The public radiation path calls `two_stream` with
`plant_area = leaf_area + stem_area` at
`crates/openwepp-vegetation/src/transaction.rs:722-724`. Consequently the
returned `sunlit_lai + shaded_lai` equals total plant area. The energy input
then uses those values directly as leaf-class LAI at `:1095-1119`, while also
constructing a separate dry-stem area at `:1120-1122`. It assigns all dry
absorbed shortwave to the two leaf classes and sets
`dry_stem_shortwave_w_m2: 0.0` at `:1123-1124`. Finally, interval GPP multiplies
gross leaf assimilation by the same plant-area sun/shade values at `:944-959`.

This double-counts stem area in the dry-leaf heat/vapor nodes, permits stem
area to produce photosynthetic carbon, and leaves the explicit dry-stem energy
owner without its area-proportional shortwave operand. It violates E02, E05,
E13 and E16 in the equation authority ledger and the contract requirement that
wet leaf, wet stem, dry leaf, and dry stem operands reconstruct the
unpartitioned stratum energy exactly. The one-stratum transaction test checks
only successful commit and transaction IDs, so it cannot detect this poison.

Required correction: retain total plant area for optics while deriving leaf
sun/shade areas and separate stem absorption/area operands before FvCB and the
four energy surfaces; prove no stem carbon gain and exact wet/dry leaf/stem
shortwave closure with all-distinct areas.

### RB-CRITICAL-002 — The admitted coupled energy/hydraulic system is not implemented exactly

There are several independent constitutive deviations:

- Negative wet-surface vapor flux is accepted without applying or checking
  `condensation_capacity_kg_m2`. The field is validated but never consumed;
  `energy.rs:223-232` returns every negative potential flux unchanged. E05/E13
  require typed rejection when the store cannot receive the condensation and
  an active capacity re-solve otherwise.
- The leaf-to-stem hydraulic paths use the maximum conductances directly at
  `hydraulics.rs:141-149`. The canonical hydraulic specification requires
  vulnerability conductances on every leaf/stem/root path; `q1a`/`q1b` do not
  apply a vulnerability factor to `k1a_max`/`k1b_max`.
- The outer gas/hydraulic equality accepts an absolute mismatch of `1e-10` at
  `transaction.rs:1242-1245` and rechecks the same threshold at `:871-876`.
  The numerical contract requires equality at the hydraulic
  `1e-12 mm s^-1 + 1e-9*scale` tolerance and explicitly forbids a separate
  mismatch allowance.

These defects leave B-CRITICAL-003 unresolved and invalidate the claim that
E11--E15 are the exact admitted common residual system. Required correction:
implement the finite condensation-cap branch, all prescribed vulnerability
conductances, and common normalized convergence/equality acceptance, then add
capacity, path-vulnerability, tolerance sensitivity, alternate-start,
singular-Jacobian and iteration-limit vectors.

### RB-CRITICAL-003 — E17/E20 phenology and persistent C/N branches differ from the frozen equations

The maintenance primitive silently clamps the exact Atkin leaf expression to
zero at `carbon_nitrogen.rs:130-132`. The admitted E17 expression has no such
floor; an invalid negative result must follow the typed failure posture rather
than become substitute zero respiration.

Deciduous threshold equality is also wrong. The contract says equality retains
phase, but `advance_phenology` enters onset when `gsi >= on_threshold` and
offset when `gsi <= off_threshold` at `carbon_nitrogen.rs:579-585`. In the
offset calculation, `Nlit` is additionally changed to
`min(Cfall/CNleaf_litter, donor_N)` and retranslocation is computed from the
mutable donor N at `:616-623`; the frozen E19/E20 identities are
`Nlit=Cfall/CNleaf_litter` and
`Nret=Cfall/CNleaf-Nlit`, with a typed failure if the accepted donor cannot
support them. The persistent `previous_leaf_offset_flux` and
`previous_root_offset_flux` state fields are never advanced or consumed.

These are public-path differences, not missing poison tests alone, and leave
B-CRITICAL-004 unresolved. Required correction: remove the constitutive clamp,
implement strict crossing/equality retention and the exact retranslocation
identity, and bind dormant/onset/active/offset plus invalid-donor vectors to
the independent oracle.

### RB-CRITICAL-004 — The five-ledger/owner transaction remains producer-derived and has no energy owner

`DiagnosticOwnedState` contains vegetation, water and biogeochemistry only
(`vegetation_diagnostic.rs:21-26`); there is no land-surface-energy owned state,
candidate, validation, or atomic commit. The five operand structures are
constructed inside the vegetation transaction by `build_ledgers`
(`transaction.rs:1382-1522`). Soil-water and mineral ending stores are derived
as `beginning - finalized` there rather than being checked against the actual
water and BGC owner candidates. The diagnostic constructs those candidates
later at `vegetation_diagnostic.rs:195-215`, but never reconciles their ending
states or BGC receiver increments against the five ledger operands before the
three sequential assignments at `:227-231`.

Material receipt validation is likewise not dual. The orchestrator converts a
producer transfer into a new receipt mechanically (`:178-194`), while BGC
accepts arbitrary string receiver names and creates missing pools with
`entry(...).or_default()` (`openwepp-biogeochemistry/src/lib.rs:178-208`). It
does not compare an immutable typed proposal to one exact receipt, and two
receipts for the same donor/receiver are accepted when their caller-supplied
`proposal_index` differs. Therefore missing receiver, duplicate receiver,
wrong receiver and donor/receiver mismatch are not structurally closed.

The rollback test at
`tests/integration/c3_vegetation_implementation_contract.rs:410-508` uses
`PartialEq` on the in-memory three-owner aggregate; it does not serialize every
owner and compare exact bytes, and it cannot test the absent energy owner.

This leaves B-HIGH-006 unresolved and violates the state-ownership ledger,
`INV-VEGETATION-066/068/070`, `INV-BIOGEOCHEM-003/004`, and the explicit
five-owner-candidate acceptance rule. Required correction: add an independent
energy owner/candidate, have water/BGC/energy owners reconstruct from immutable
typed proposals, validate actual candidate endings and exact receiver credits
against ledger operands, reject missing/duplicate/unsupported receivers, and
prove serialized byte equality for all owners at every injected phase.

### RB-HIGH-005 — The selected nonlinear algorithms and convergence test are not faithfully transcribed

The Brent rotation at `numerics.rs:73-79` uses sequential assignments:
after `lower = upper; upper = c`, it sets `c = lower`, and similarly overwrites
`fa` before setting `fc = fa`. This is not the required three-value rotation
and can collapse distinct bracket points/function values. Passing the current
FvCB examples does not prove Brent-Dekker identity across the required
brackets.

The damped-Newton routine accepts whenever the residual norm is at most one at
`numerics.rs:171-184`, regardless of whether the most recent temperature or
potential step meets the selected step tolerance. The numerical contract says
every normalized residual and every applicable step tolerance must be met.
Further, `normalized_norm` scales a residual by that same residual magnitude
rather than a separately supplied physical residual scale, so it does not
implement `atol + rtol*physical_scale` as frozen.

This leaves the numerical portion of B-HIGH-007 unresolved. Required
correction: repair the safeguarded Brent state rotation, supply explicit
physical scales, require both residual and step convergence, preserve typed
solver-specific failures/diagnostics, and add independent bracket, singular,
iteration-limit and half/twice-tolerance vectors.

### RB-HIGH-006 — Exact proportional arbitration and receiving-class guards remain incomplete

Typed `(layer,species)` nitrogen identity is now preserved, so the central
species-borrowing defect in B-CRITICAL-005 is corrected. However,
`authorize_proportionally` accumulates request totals in caller order with
plain floating-point addition at
`openwepp-biogeochemistry/src/lib.rs:87-120`; the BGC contract requires
deterministic stratum-ID order and compensated sums. The diagnostic water
arbiter uses the same plain caller-order aggregation at
`vegetation_diagnostic.rs:95-125`. BGC material receivers are untyped strings
and unsupported classes are not rejected as noted above.

The current tests cover one proportional N example, layer/species separation,
unused authorization, and an identical duplicate receipt. They do not cover
wrong-species authorization, wrong-layer authorization, full competing-owner
debit bypass, alternate request order, a duplicate receiver with a distinct
proposal index, missing receiver, or unsupported receiver. These omissions
matter because several of those alternatives are accepted by the current
public receiving API.

Required correction: deterministic compensated aggregation, typed exhaustive
receiver identity, exact proposal/receipt matching, and all mandatory poison
vectors.

## Historical Finding Reassessment

| Historical finding | Current assessment |
|---|---|
| `B-CRITICAL-001` public E01--E22 path | `PARTIAL / FAIL`: the proxy was removed and modules are called, but RB-CRITICAL-001 through RB-CRITICAL-003 mean the public path is not the exact indivisible model. |
| `B-CRITICAL-002` E01--E03 radiation | `PARTIAL / FAIL`: matrix-exponential optics and directional fields exist, but the public consumer aliases plant/stem area into photosynthetic leaf area and does not preserve the required surface-energy partition. |
| `B-CRITICAL-003` E11--E15 coupled solve | `FAIL`: RB-CRITICAL-002 and RB-HIGH-005 remain. |
| `B-CRITICAL-004` E16--E22 persistent C/N | `FAIL`: RB-CRITICAL-003 and receiver-closure defects remain. |
| `B-CRITICAL-005` mineral-N identity | `CORE CORRECTED / acceptance incomplete`: typed layer/species request/auth/use/debit exists; deterministic arbitration and mandatory poisons remain under RB-HIGH-006. |
| `B-HIGH-006` independent closure/rollback | `FAIL`: RB-CRITICAL-004 remains. |
| `B-HIGH-007` guards/numerics | `PARTIAL / FAIL`: peaked response and broad finite guards are corrected; selected solver identity/convergence is not. |

## Review Disposition

`FAIL`. The package must remain `executing`; heavy gates, terminal verifiers,
prompt archival, and `science_implementation_status=IMPLEMENTED` are not yet
legitimate. All findings above are within the existing package authority and
write set. None is an authority contradiction or a legitimate HOLD boundary.
