# QA review — event-aware restoration

> **Terminal status (2026-09-16):** Scoped QA evidence is PASS at terminal
> source tree `7c04f52e…` / patch `6f9f1cf0…` and frozen binary `47fd72c2…`.
> Overall acceptance remains **HOLD** for the explicit Critical strict-lint and
> campaign gates and the preserved expected native refusal. See
> [Final terminal QA disposition](#final-terminal-qa-disposition). Historical
> interim findings below are retained as evidence history, not the current
> source status.

Reviewer: `/root/event_qa`, distinct from the writer, adviser, and correctness
reviewer. Scope: event-aware artifact receipts and recorder, current A-002 test
surface, custody/resource/counter evidence, and quality-gate posture. This is an
interim record only: native composition is still being authored and requires
same-reviewer final-diff and final-evidence verification.

Evidence class: **Static and Ran (inspected supplied executions).** I did not run
Rust, a decoder, or model physics. I inspected the authorization, package and
review procedures, `run_recorded.py`, final receipts/logs named below, and the
current detached A-002 tests. “Ran” below always means an inspected receipt, not
a reviewer-run command.

## Findings

### Blocker — A-002 direct test can silently pass without its immutable corpus

`crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_publication_retention.rs:2091`
returns `None` when `OPENWEPP_B01_MEMBER_EXPORT` is absent, and the main direct
restorer test at line 2121 uses `let Some(...) else { return; }`. Therefore the
test reports success without executing the archive restorer or any poison when
the corpus environment is absent. The present `a002-expanded-green` receipt does
supply the environment, but the test fixture itself is fail-open. Require the
fixture/environment with `expect` or an assertion before accepting the test as
coverage.

**Fix verified (Static):** the current source replaces the optional return with
`expect("bound immutable one-terminal-event archive")`. This removes the silent
skip. Re-run the selector after the remaining direct-wire controls are added and
the composition source is frozen.

### Blocker — required malformed archive-wire cases do not reach the direct restorer

The direct restorer test changes only omitted, duplicated, and wrong-final-owner
wires. `a002-terminal-chain-controls` exercises the lower-level
`validate_accepted_publication_chronology_v1` helper for ordinal, tick,
foreign-parent, and reordered cases, rather than the serialized
`restart_authority_restore_archived_publication_day_v1` path. It lacks an
explicit wrong-beginning test. The adopted authorization requires omitted,
duplicated, reordered, wrong-tick, foreign-parent, wrong-beginning, and
wrong-final-owner event wires with otherwise valid structure, preserving error
precedence. Add direct-wire coverage or retain a source-bound demonstration that
the restorer reaches each guard.

### High — A-002 red source identity is missing

`a002-red-direct.*` correctly preserves a bounded targeted failure (exit 101,
`Identity("archived publication day chronology")`, 16-GiB AS and 1-GiB file
limit). Its source tree/patch receipt was not captured before the guard edit, as
the artifact explicitly acknowledges. The durable binary hash proves the binary
bytes, but does not map it to an exact pre-fix source. It is useful directional
evidence, not a fully source-bound red half of a before/after proof.

### Medium — early recorder receipts predate pinned-file stability capture

The recorder evolved after `a002-expanded-build` and during/after the expanded
green run to add per-run pinned-file stability capture. Consequently
`a002-green-recorded.json` and `a002-expanded-green.json` prove their
source/binary/member custody but do not prove runner/flake/export-summary
stability through those executions; this is a historical evidence limit, not a
claim that their then-current recorder malfunctioned. The globally preserved
initial custody still verifies the member summary and seed. Final archive/native
attempts must use the later complete receipt and make the distinction explicit.

### Medium — zero-work and publication counters remain unmatched

The A-002 receipts establish selected test behavior only. Their stdout and
resources contain no post-unwind physical-work or publication audit output.
They cannot support A-004 for the future composed archive/native success and
refusal attempts. Each final attempt needs its own counter output after success
or error unwind, bound to the actual call path.

## Non-blocking debt and follow-ups

- `a002-expanded-build` passes `--no-run`, but its stderr records existing
  warnings. It is compilation evidence, not strict `clippy -D warnings` PASS.
  The Critical change still cannot use matched diagnostics for acceptance under
  testing strategy §6.2; strict Clippy remains historical FAIL and campaign
  correctness remains BLOCKED under the run allowance.
- `a002-rustfmt-check` passes against tree
  `65467f28…`, patch `d6649059…`; it is valid formatting evidence only.
- The predecessor in-flight archive command is correctly recorded INVALID:
  it was source-raced and began decoding without AS/FSIZE/CPU bounds. It cannot
  be promoted to a red control, archive outcome, or zero-work proof.
- `a002-expanded-green` and `a002-terminal-chain-controls` are source/binary
  stable, bounded executions (tree `65467f28…`, binary `bbf48dd9…`) and are
  useful focused A-002 evidence once the two test gaps above are corrected.
- **Ran (inspected):** `dto-roundtrip-build` and `dto-roundtrip` share tree
  `456ba6c2…` / patch `a5ee1b24…`; `dto-roundtrip-binary.json` binds the
  durable binary `224b5a36…` to that build. The run preserves the 49-member
  input manifest, recorder/flake/export-summary pins, and reports zero solver,
  direct-runtime (including publication capture), and native-physics counters.
  It proves the actual DTO field capture/inverse roundtrip only, not native
  consumer construction, cross-object joins, or composed acceptance.
- `composed-second-build` is a successful build-only result for later source
  `91a5fc37…`; prior A-002 and DTO executions do not certify that changed cut.

## Composed native refusal and initial quality comparison

Evidence class: **Static and inspected Ran evidence.** I inspected
`composed-fourth.*`, its binary/source custody, the latest independent
correctness addendum, and `lint-initial-comparison.json`; I ran no command.

### A-001 — High — composed-fourth is a valid refusal, not restoration success

`composed-fourth` is a source/binary/input-stable, bounded execution (tree
`fec21c09…`, patch `545ac71e…`, frozen binary `c1ec5581…`, 16-GiB AS, 1-GiB
FSIZE, 900-second timeout). It authentically reaches the canonical native
constructor after the archive/manifest/prefix path, preserves outer position 214
versus captured 0 and the provider-GSI mismatch without installing either, and
then refuses `native soil proof accepted support or exact bootstrap`.

The post-unwind audit records one ordinary run-frame construction but zero solver
work, direct compute/state mutation/commit/publication, and native
litter/surface/WB14 physics. This is sound A-004 evidence for this *refusal*
path. It does not establish restoration success. Keep the reader position/GSI
refusal and the canonical stale-soil refusal separately attributed, as the
correctness review requires.

### Quality — initial unmatched diagnostics are not acceptance evidence

`lint-initial-comparison.json` covers intermediate tree `fd52a584…` / patch
`564063c…` and reports 59 occurrences / 48 unique candidate-unmatched
diagnostics. It predates writer quality patches and cannot certify the final cut.

The constructor-inverse and member-restoration diagnostics are introduced at that
cut: their dead code, inconsistent constructor field order, wildcard import,
overlong helper, items-after-statements, and unnecessary fallible conversion need
fixing or direct final matched-equivalence evidence. They must not be described
as inherited. Old private reader dead-code and overlong-function diagnostics may
be equivalent relocations only if the final comparison matches lint, source
content, and relevant dependency surface; current member-restoration calls now
reach several of those old helpers, but that source observation is not a final
quality disposition. Re-run the capped complete-target comparator after the
quality patch and preserve the exact unmatched classification.

## Interim QA verdict

**HOLD.** A-001, A-003, A-004, and A-005 remain open pending the composed native
cut. A-002 has useful bounded green evidence but remains open because its direct
fixture is fail-open, required direct-wire negatives are incomplete, and the
pre-fix source identity is unavailable. No production admission, scientific or
cadence closure, or quality PASS follows.

## Final static quality review

Evidence class: **Static and inspected Ran evidence.** I inspected the current
source and the final runner/orchestrator complete-target comparison records. I
did not run the in-flight focused build or any test.

The archive fixture-only helpers and the direct archive-restorer test are now
gated by `persisted-restart-v1`, including their shared `HEADER`. This matches
the restorer's feature boundary and avoids widening unrelated target surfaces.

`lint-final-runner-comparison.json` has 202 candidate messages, all 202 matched
to its 291-message baseline. Each runner lib/libtest, binary, example, and test
target record has `complete: true`, and `build_finished.success` is true.
`lint-audited-orchestrator-comparison.json` has 2 remaining candidate-unmatched
occurrences, both the already-adjudicated byte-identical
`v9_real_consumer_shadow_v10_accessors.rs:92–100`
`unnecessary_lazy_evaluations` presentation variant. Its primary source,
suggestion, target and lint are unchanged; the added warning-denial child note
is command presentation only. All audited orchestrator target records are
complete and its build-finished result is successful.

This establishes final matched-diagnostic quality evidence within the adopted
method. It remains diagnostic quality evidence only: the Critical chronology /
restart change still has no strict-Clippy or campaign-strength correctness PASS.

## Final focused controls (pre-terminal source)

Evidence class: **Static and inspected Ran evidence.** The source-stable
`final-focused` binary `21b1acc6…` at tree `677cd055…` / patch `d546254c…`
passes final DTO, phase-positive, phase-poison, provider-poison,
manifest-poison, archive-beginning-poison, and archive-control selectors. Each
receipt records unchanged source, binary, 49-member inputs, and pinned files.

The archive-control run directly exercises no/one/multiple terminal-event
endpoints and raw plus independently resealed tick, parent, beginning-owner,
reorder, omission, duplication, final-owner, schema, count, and noncanonical
serialization controls. Its stdout names the reached guard for each failure;
the run's post-unwind audit records zero solver, direct computation/mutation/
commit/publication, and native-physics work. This resolves the earlier direct
A-002 test-reachability gap for that pre-terminal source.

The phase/provider/manifest/beginning controls likewise demonstrate downstream
semantic refusal rather than parser-only rejection and retain zero-work audit
output. The default-feature check exposed 18 introduced missing-feature helper
errors in addition to inherited Q errors. Those 18 require the writer's planned
feature gating and a terminal source/build rerun; no current-source acceptance
follows from this pre-terminal control batch.

## Terminal-source compatibility finding (superseded cut)

**Evidence class: Ran (inspected receipt) and Static.**

### Blocker — default-feature test build has two new unresolved helper calls

`terminal-default-feature-check.json` is a bounded, source-stable execution at
tree `b448fa20…` / patch `fe48f2d…`, but exits 101. Its explicit comparator
against the 61-error acquisition baseline records 63 candidate errors and
attributes the two introduced messages to
`snow_stage3_v11_current_context_capture.rs:3377,4352`: both call
`snow_stage3_v11_attachment::diagnostic_restore_coupled_subslab_receipts_value_v2`,
where no such attachment function is available at the default feature set. The
similarly named helper is feature-gated in `snow_stage3_v11_restart.rs`.

This is a genuine introduced default-feature compatibility regression, not an
inherited diagnostic or an evidence-recorder defect. `terminal-focused-build`
and `terminal-format-check` pass at the same source but enable
`persisted-restart-v1,restart-authority-evidence` or inspect formatting only;
neither repairs the failed default-feature test compilation. The writer reports
that this source is superseded by gating the two persisted-restart-only callers.
Require the new source identity, a fresh default-feature comparison with no new
messages, a fresh focused binary, and rerun receipts before replacing this
finding with a terminal result.

## Exact-tree terminal quality and focused-control checkpoint

**Evidence class: Ran (inspected receipts) and Static.** The repaired terminal
tree is `7c04f52e…` / patch `6f9f1cf0…`. Its focused build succeeds and binds
`terminal-focused.frozen` SHA-256 `47fd72c2…`; `terminal-format-final-check`
passes on the changed source files. The no-default-feature test check still
exits 101, but `terminal-default-feature-final-comparison.json` shows exactly
the same 61 inherited compiler errors as the acquisition baseline, with zero
introduced and zero removed messages. Thus the two introduced compatibility
errors from the superseded `b448fa20…` source are fixed; this remains a failed
default-feature compilation and is not a passing gate.

`terminal-lint-runner-comparison.json` matches all 202 candidate diagnostics to
its 291-message baseline, with every enumerated runner target complete and a
successful build-finished record. The complete orchestrator comparison matches
2,827 of 2,829 candidate diagnostics to its 2,954-message baseline, with every
enumerated target complete and a successful build-finished record. Its sole
unmatched diagnostic has multiplicity two and is the already-reviewed,
byte-identical accessor `unnecessary_lazy_evaluations` presentation variant.
This is capped matched-diagnostic observation (`--cap-lints warn`), never a
strict Clippy PASS.

Six fresh-process terminal controls (`terminal-dto`, `terminal-phase-positive`,
`terminal-phase-poisons`, `terminal-provider-poisons`, `terminal-manifest-poisons`,
and `terminal-beginning-poison`) pass against source `7c04f52e…` and frozen
binary `47fd72c2…`. Each receipt preserves source, binary, inputs, and pinned
files. Their stdout shows actual DTO inversion; phase/prefix, provider, manifest,
and beginning semantic refusal controls; and post-test zero solver, direct
compute/mutation/commit/publication, and covered native-physics counters. The
remaining direct archive and composed native terminal selectors were still
running at this checkpoint; these six results alone do not close A-001–A-005.

## Final terminal QA disposition

**Evidence class: Static and Ran (inspected completed receipts).** No command
was run by this reviewer. The terminal executable identity is source tree
`7c04f52e6cae6327202591b37f0c4e7a5fff3ae34a103255a35c49a3dbdf37ab`,
patch `6f9f1cf01547c96baf229f5d5ac1e7cbec6124f6042053f53b39880d982158b4`,
and frozen binary SHA-256
`47fd72c2b2c9acb422021659d499be3ba0b29020d94468f084a9846eb89cb631`.

### Findings

### Hold — Critical required gates remain absent

The terminal complete-target records are intentionally capped diagnostic
observations and the default-feature test compilation retains the documented
61 inherited errors. There is no terminal strict `clippy -D warnings` PASS and
no campaign-strength full correctness execution. Under the Critical restart /
chronology scope these missing gates prevent full-quality acceptance. The
earlier unbounded/source-raced archive invocation remains **INVALID**, and the
early source-unbound red result remains a historical custody gap; neither is
promoted by these terminal records.

### Hold — the actual native checkpoint remains an expected refusal

`terminal-native-refusal` passes its *refusal assertion* (1/1, 218.56 seconds)
with complete source/binary/input/pin custody. It authenticates all four archive
days and the provider prefix, decodes the actual native member, preserves outer
position 214 versus captured 0 and the provider/captured GSI mismatch without
installation, then independently reaches the canonical soil-support refusal.
The aggregate result remains an error. Its post-unwind audit reports one
ordinary run-frame construction and zero solver, physical, direct compute,
mutation, commit, and publication work. This is sound refusal and no-work
evidence; it is not native restoration success, a cadence result, or production
admission.

### Verified — terminal archive and chronology controls are direct and bound

`terminal-archive-controls` passes 1/1 in 124.29 seconds at the terminal
identity, reaches the direct archive-restorer controls for raw and independently
resealed tick, parent, and beginning-owner substitutions, ordering, omission,
duplication, terminal owner, schema, count, and noncanonical serialization,
and emits the reached guard messages. Its audit counters are all zero. The
three fresh terminal chronology selectors also pass 1/1 each with the same
identity and custody. Together with the first six focused controls, this
resolves the earlier direct-test fail-open/reachability QA finding for the
terminal source.

## Non-blocking debt and follow-ups

- Private no-physics inverse/reseal representation remains coupled to canonical
  persisted-restart and land-surface-energy encodings; retain the documented
  parity controls and synchronization responsibility.
- The unchanged accessor lint presentation variant remains attributed as
  inherited in `lint-equivalence-review.md`; do not make unrelated accessor
  changes merely to change diagnostic rendering.

## QA verdict

**Scoped QA evidence PASS; overall acceptance HOLD.** The final source has no
remaining introduced default-feature compiler message, introduced capped-lint
diagnostic, formatter failure, fixture skip, or focused-control custody defect
identified by this review. Full acceptance remains blocked by the explicit
Critical strict-lint and campaign gates and by the preserved native refusal;
the historical FAIL/INVALID records and A-001–A-005 scientific/cadence
disposition remain intact for correctness authority.
