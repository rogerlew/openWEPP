# Review Agent A

Status: `EXECUTED-HOLD-REVIEWED`

Evidence mode: `Static` package/contract/decision/source review plus `Ran`
instruction discovery, pinned-object source reads, scoped worktree status, and
line counts. No runtime, comparator, or Rust validation was run.

Review scope: scientific authority, pinned-baseline lineage, ADR-0036
interpretation, hold legitimacy, pre-contract/production stop correctness,
gate evidence, and line-count/no-production-edit disposition. Review Agent B's
artifact was not read.

## Findings

### High - Executed-hold disposition evidence is not yet complete

The package already declares `EXECUTED-HOLD-*` at `package.md:3`, but three
required closure artifacts remain placeholders: `artifacts/gate-results.md:3`
is `queued`, `artifacts/worker-handoff.md:3` is `queued`, and
`artifacts/disposition.md:3` is `queued`. The consumer and conservation
artifacts also remain `queued` rather than explicitly `BLOCKED` at
`artifacts/consumer-path-evidence.md:3` and
`artifacts/conservation-reconstruction.md:3`. This does not invalidate the
science hold, but it prevents final disposition under the gate-classification
rule and the package's own Phase F requirements (`package.md:366-378`).

Recommended disposition: `accepted`; closure-blocking for package disposition,
not a reason to resume production. Classify every package/root gate as `PASS`,
`FAIL`, `BLOCKED`, or `NOT RUN`; identify W11A as the blocker where applicable;
complete the worker handoff and final executed-hold disposition after review
finding disposition and verification.

### Medium - Owned-file and documentation-validation evidence is incomplete

`artifacts/owned-file-manifest.md:5-12` says documentation validation is still
to be recorded and gives categories rather than the required file-by-file
manifest. The package owns its changed artifacts, roadmap/catalog status, and
the W11A scaffold, so the executed-hold record must enumerate those paths and
record the scoped Markdown and whitespace validation required by
`package.md:374`.

Recommended disposition: `accepted`; fix before final hold disposition. This is
an evidence-governance defect, not scientific authority to proceed.

### Low - The kernel checklist does not reconcile completed lineage evidence

`artifacts/kernel-profile-compliance-checklist.md:9` leaves pinned provenance
and symbol aliases unchecked even though the pinned source commit and primary
water/sediment symbols are recorded in
`artifacts/baseline-source-map.md:5-19`. Either mark only the actually proven
portion complete and name missing aliases, or keep it blocked with an explicit
reason. The current unchecked item is ambiguous.

Recommended disposition: `accepted`; reconcile before final hold disposition.

## Scientific-Authority Assessment

ADR-0036 is not being ignored or narrowed incorrectly. D2 explicitly permits
the first-cut class reconstruction `M[h,k] = S_h * frcflw[k]` using the
event-level class fractions uniformly across hours
(`docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md:129-158`).
The package correctly labels this as a day-blend reconstruction rather than
enriched hourly composition (`package.md:34-37`; `operand-lineage.md:9-14`),
consistent with `SC-SED-001#GAP-SED-008`.

ADR-0036 D1 is specifically the hillslope Wave-1 erosion solve and its OFE-hour
state (`ADR-0036:75-127`); it does not specify channel WS18-WS26 mutation.
D3 requires watershed routing to consume and superpose paired `V_h/S_h`
(`ADR-0036:178-187`), but it does not select hour versus `dtchr`, define channel
bed/profile carry, couple routed water storage to sediment ingress, or define
per-quantum channel egress. Current canonical authority explicitly retains the
event-level single-rate channel limitation in
`SC-ROUTE-001#INV-ROUTE-005(e)`, matching
`artifacts/contract-implementation-evidence.md:7-12`.

The distinction is material: D3 is an accepted destination/consumer mandate,
not sufficient process-physics detail for implementing the channel sediment
state transition. The package correctly avoids treating D1's hillslope rule as
channel authority.

## Pinned-Baseline Lineage Assessment

Ran: `git -C /workdir/wepp-forest_260430_baseline rev-parse HEAD` returned
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. The baseline worktree is dirty in
unrelated paths, but a scoped diff over `wshchr.for`, `chrqin.for`,
`wshscs.for`, `wshpek.for`, `wshrun.for`, `wshirs.for`, `wshpas.for`,
`chnrt.for`, and `wshdrv.for` was empty. Independent `git show
dac3c950...:src/<file>` reads confirm the cited mechanism:

- `wshdrv.for:1097-1114` dispatches the channel erosion path once for the
  event after channel runoff work;
- `chnrt.for:293-347` converts event class mass to flux through scalar
  `rundur` and has no time index;
- `chnrt.for:723-739` converts routed fluxes back to event mass using the same
  scalar duration;
- `wshchr.for:231-252` superposes upstream channel `q1(it,channel)` on the
  shared water-routing grid.

This supports `artifacts/baseline-source-map.md:8-23`: water-series routing is
source-authorized for the named branches, while interval channel-sediment
sequencing is not supplied by the pinned baseline. The review found no pinned
source contradiction that would make production implementation mandatory now.

## Hold-Legitimacy Assessment

Result: `PASS` for a truthful executed hold, `BLOCKED` for W11 production and
completion.

The blocker matches the declared hold boundary at `package.md:391-405`: a
required channel branch lacks baseline/canonical process physics. The
hold-legitimacy audit identifies the exact missing decisions, records the
in-package repeated-solve route that was considered and rejected, explains why
water-only work cannot close the consumer/conservation gates, and gives the
defect-shaped first action `WSHED-W11-HOLD-001`
(`artifacts/hold-legitimacy-audit.md:7-42`). W11A is scoped to acquire and ratify
the missing temporal quantum, state-carry, class-continuity, and closure
authority before W11 resumes.

Stopping before canonical contract amendments, contract-derived tests, and
production edits is correct. Amending the contracts now would require the W11
executor to choose process semantics not established by the baseline,
ADR-0036, or an existing `SC-*` invariant. Implementing repeated independent
hour/event solves or redistributing scalar output would be surrogate physics
and would violate the package's consumer-path and no-proxy rules.

## Line Count and No-Production-Edit Assessment

Ran: scoped `git status`/`git diff` showed no changes under `crates/`, `tests/`,
or `docs/specifications/science-contracts/`. Only package/queue documentation is
in the W11 execution scope. Ran: `wc -l` reports `kernel/direct.rs` at 2,310
lines, already in the 2,000-line warning band, but no `.rs` file was edited.
Therefore `artifacts/line-count-governance-checklist.md:3-12` is correct:
line-count governance is not applicable to this hold execution, while the
future resumed implementation remains obligated to use a cohesive owner such
as `hourly.rs`.

## Recommendation

`HOLD` WSHED-W11 production on `WSHED-W11-HOLD-001`. Do not amend contracts or
land partial water-only/channel-sediment code. Complete the accepted package-
evidence findings above, disposition this review, verify the documentation
gates, and record the final executed-hold handoff to WSHED-W11A.
