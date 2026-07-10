# Review Agent B

Status: `EXECUTED-HOLD-FINDINGS`

Evidence mode: `Static` plus `Ran` repository/source checks. I did not read
`review_agent_a.md`.

## Scope

Independent review of Rust architecture claims, the consumer-path rule, gate
non-deferral, the declared hold boundary, WSHED-W11A follow-on adequacy,
artifact truthfulness, required evidence, line-count governance, and production
edit status.

## Findings

### High - Final executed-hold status precedes required closure evidence

The scientific blocker is credible, but the package is not yet eligible for a
final `EXECUTED-HOLD-*` disposition. `package.md:3` and the queue/catalog already
publish executed-hold, while `artifacts/gate-results.md:3-8`,
`artifacts/disposition.md:3-8`, and `artifacts/worker-handoff.md:3-8` remain
queued. Both verification artifacts also remain queued. This conflicts with
the required dual review, finding disposition, dual verification, worker
handoff, and final disposition before closure
(`docs/work-packages/AGENTS.md:38`, `:57-61`, `:207-216`).

Recommended disposition: `accepted`, closure-blocking for final package status.
Keep the implementation outcome on `HOLD`, but mark the package as executing
hold evidence until every exit criterion is classified `PASS`, `BLOCKED`, or
`NOT RUN`; populate the final disposition and handoff; disposition both review
reports; and complete dual verification. Publish executed-hold in the roadmap
and catalog only after that evidence is current.

### Medium - Blocked consumer and conservation artifacts are still queued

The intake assessment supplies useful current-path evidence, including the
typed dependency failure (`artifacts/intake-assessment.md:8-18`) and rejects
scalar closure substitutes (`:45-52`). The dedicated consumer-path and
conservation artifacts nevertheless remain generic queued templates
(`artifacts/consumer-path-evidence.md:3-10` and
`artifacts/conservation-reconstruction.md:3-9`). A held consumer-facing package
must still state which current consumer reads the old/scalar path, why the new
path was not produced, and which acceptance checks are `BLOCKED`; queued text is
not a gate classification. This is required by the consumer-path check and
hold-lift rule (`docs/work-packages/AGENTS.md:74-91`).

Recommended disposition: `accepted`. Convert both artifacts from `queued` to
truthful `BLOCKED` evidence, cross-reference the current typed guard, state that
no W11 producer/state/runner/publication path exists, and bind each blocked
reconstruction/consumer check to `WSHED-W11-HOLD-001`.

### Medium - Ran provenance is not reproducible from the artifact set

The baseline source map labels source searches as `Ran` and gives useful source
locations (`artifacts/baseline-source-map.md:5-19`), but it records neither the
exact commands nor their outputs. The gate-results artifact is also still
queued. I independently confirmed the baseline checkout is the declared
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, but a later verifier should not
have to recreate the executor's search procedure to establish evidence
provenance.

Recommended disposition: `accepted`. Add exact revision/source-search commands
and summarized results to the source map or gate results, including the
execution revision and date. Do not relabel static interpretation as a run.

### Medium - WSHED-W11A has the right blocker but incomplete closure scaffolding

WSHED-W11A is defect-shaped and its first executable action is adequate: it
names the temporal quantum, water coupling, profile/bed carry, class closure,
guards, tolerances, tests, and exact W11 handoff
(`docs/work-packages/20260710-wshedw11a-channel-hourly-sediment-authority-001/package.md:13-20`,
`:40-53`, `:98-128`). Its scaffold, however, has no named required-reading map,
gate-results artifact, review-finding disposition artifact, or final disposition
artifact; its current artifact inventory covers only the authority matrix,
contract disposition, reviews, verification, and W11 handoff. That leaves its
own dual-review/disposition/final-status obligations under-specified.

Recommended disposition: `accepted` before W11A execution. Add the missing
closure artifacts and require exact source/literature provenance plus a
criterion-by-criterion gate table. Retain the explicit no-surrogate and
authority-insufficient hold branch.

## Rust Architecture Assessment

Static inspection supports the package's current-state claims:

- `RoutedChannelState` is scalar and has no hourly water/sediment outputs
  (`crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs:292-308`).
- Hourly hillslope authority with dependency nodes fails closed at
  `direct.rs:970-988`; the hourly water input is reduced to peak, volume, and
  duration at `direct.rs:1028-1038`.
- Channel output assigns `sediment_yield_kg` from `qsed_kg_s`
  (`direct.rs:628-655`), while publication sums that field across selected
  channel states (`network_frame.rs:620-625`). The package correctly flags the
  unit-alias and internal-channel double-counting hazards for resumed W11.
- The proposed cohesive `kernel/hourly.rs` owner is appropriate; adding the
  solver to the existing warning-band `direct.rs` would worsen line governance.

No unsupported production-readiness or consumer-path closure claim is made.
The existing typed dependency rejection remains the production behavior.

## Gate-Legitimacy Assessment

The substantive hold boundary is legitimate. The pinned baseline routes
channel sediment once per event after water routing, while current
`SC-ROUTE-001#INV-ROUTE-005(e)` explicitly retains the single-rate scope limit.
The package identifies the missing interval sequencing, geometry/profile/bed
carry, discharge basis, and class-state closure, and explains why repeated
hourly event solves would invent process physics
(`artifacts/hold-legitimacy-audit.md:7-36`). Water-only work cannot satisfy the
package's inseparable sediment consumer and conservation gates
(`package.md:407-426`). No in-envelope authority-backed production correction
was shown to be skipped.

Gate result: `HOLD` is legitimate; final hold closure is not yet legitimate
because the closure artifacts and verification listed above are incomplete.

## Line Count and Edit Assessment

Ran: `wc -l crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`
returned `2310`, correctly placing it in the 2000+ `WARN` band and below the
3000-line mandatory-refactor threshold. No Rust file was edited for W11.

Ran: `git status --short`, `git diff --stat`, and `git diff --name-only` showed
only `docs/ROADMAP.md`, `docs/work-packages/README.md`, the W11/W11A package
trees, and unrelated pre-existing untracked artifacts. I found no production,
test, fixture, or canonical-contract edit attributable to W11. The package's
no-production-edit claim is supported.

## Recommendation

`HOLD` on WSHED-W11 implementation and `HOLD` on final package closure pending
the accepted documentation/evidence fixes, second independent review, finding
disposition, and dual verification. After those closure steps, an
`EXECUTED-HOLD-MISSING-CHANNEL-HOURLY-SEDIMENT-SEQUENCING-AUTHORITY`
disposition is supportable and WSHED-W11A is the correct next package.
