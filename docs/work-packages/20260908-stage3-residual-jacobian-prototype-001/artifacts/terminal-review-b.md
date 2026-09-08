# Terminal review B — QA, evidence, and closure

Static: corrected v33 authority, package artifacts, primary-checkout diff, and
detached Phase-A source/custody bundle. Ran: exact active-corpus SHA/cmp checks,
the authentic transaction/oracle test, and the oracle-selector self-test.
Role/session: `terminal_review_b`; configured/requested effort inherited;
effective runtime metadata UNOBSERVED. Reviewed identity: active corpus
`a05ca09031c28f863bc8dd8f3cc8d605459a683abc2af6e0f37dd84c713068c0`,
detached oracle blobs `3ae38b0…` / `8acff12…`, custody manifest
`488eaff3…`, and the current primary-checkout substantive cut.

## Findings

### TR-B-001 — MEDIUM — the terminal line-count disposition is false

Location: `artifacts/line-count-governance.md:3-6`; detached
`crates/openwepp-land-surface-energy/src/solver_covered_evaluation.rs` and
`solver_stem_jacobian_tests.rs`.

Evidence: the package changed and retained detached Rust used to generate the
terminal evidence. The custody bundle lists twelve Rust source files, and the
reviewed detached tree has a modified 2,685-line
`solver_covered_evaluation.rs`. The artifact instead says no Rust source changed
and leaves the required >=2,000-line WARN/rationale and exact touched-file
inventory undone. This is not cured by keeping the experiment outside the
primary checkout.

Required disposition: reconcile the terminal detached Rust inventory, record
the >=2,000-line warning and why the observation-only edits do or do not warrant
a split, and state that no >=3,000-line changed file exists (if confirmed).

### TR-B-002 — MEDIUM — affected warnings-denied lint is not closed

Location: detached `solver_litter_phase.rs:245,249`;
`artifacts/gate-results.md:13-19`; `package.md:132-137`.

Evidence: both reviewer reruns passed, but rustc emitted two `unused_mut`
warnings in the package's observation hook. Gate results reports scoped
rustfmt/diff-check only and does not report an affected `clippy -D warnings`
result or explicitly classify this known package-local failure. The kickoff and
package require affected warnings-denied lint even though no J candidate was
built.

Required disposition: either run the affected warnings-denied lint on the
retained Phase-A source and fix/re-capture if it fails, or classify it plainly
as a terminal failed/not-run QA gate and remove `COMPLETE` closure language.

### TR-B-003 — MEDIUM — the oracle tests under-specify two claimed admission checks

Location: detached `solver_stem_jacobian_tests.rs:64-102,171-230,273-295`;
`artifacts/oracle-results.md:6-7,14-24`; canonical
`numerical-methods.md` v33 smooth-mask rules.

Evidence: `smooth_mask_same` checks physical branch IDs and signs, but does not
explicitly compare the shared-heat `y<1` / `y>1` floor classification required
by v33. The advertised kink self-test computes a centered difference of
`abs(-h)` and `abs(h)`, yielding the same all-zero estimate as the constant test;
it therefore does not demonstrate kink detection. The audit also retains only
the aggregate support count, not per-column failed-row/basin records. These are
test-quality gaps. They do **not** overturn the conservative 0/16 result: the
review rerun reproduced 0/16 and byte-identically regenerated the frozen corpus,
and an unsupported result cannot admit a candidate accidentally. They do limit
the claim to this exact corpus and implementation.

Required disposition: narrow the oracle-results prose to the checks actually
performed and record the missing floor/kink/per-entry diagnostics as obligations
of the proposed oracle-resolution follow-on. If the package wants to claim full
v33 oracle/mask conformance, add those checks under new prospective authority
and re-review the affected result.

### TR-B-004 — MEDIUM — terminal lifecycle artifacts disagree with one another

Location: `package.md:3,151-161`;
`artifacts/preimplementation-contract-gate.md:17-21`;
`artifacts/gate-results.md:8-19`; `docs/work-packages/README.md:7-14`;
`docs/work-packages/active.md:8`; `docs/ROADMAP.md:34-44`.

Evidence: `package.md` declares `COMPLETE` while its own terminal review and
verification checkbox remains open. The preimplementation gate still calls the
superseded zero-stem corpus `b8f6a923…` a PASS and says corrected re-review is
pending. The catalog/locator/roadmap still describe an active experiment that
will proceed through J. Gate results now records authority anti-evasion and
AUTH11 PASS, but later says “anti-evasion” is NOT RUN without distinguishing a
candidate-only check. These mismatches make the final state ambiguous even
though the current handoff is accurate.

Required disposition: keep status pending until both reviews/verifications
close, mark the zero-stem preimplementation row superseded/invalid, distinguish
the passed authority anti-evasion gates from ineligible candidate gates, and
update the catalog/locator/roadmap to the bounded terminal decision.

## Non-blocking debt / follow-ups

- The custody bundle is byte-valid: its manifest hash verifies, includes the
  active corpus and relevant Phase-A source bytes, and truthfully says
  rebuildability is not proven. Preserve that limitation; do not describe the
  bundle as a reproducible build or a candidate executable.
- The expected-red E0425 record proves only absence of the proposed symbol.
  Because J was never implemented, the later unconditional-panic seam supplied
  no behavioral assurance; retain it only as historical sequencing evidence.
- The active corpus is an authentic covered potential/fixed-final transaction,
  not a real-runner or scale corpus. The package correctly makes no downstream
  consumer, full-solve equivalence, cost, timing, memory, or multi-OFE claim.

## QA verdict

**HOLD for terminal package closure pending TR-B-001 through TR-B-004.** The
scientific architecture verdict itself is supported and should remain
`INCONCLUSIVE_WITH_BOUND`: I reproduced the exact corpus and the baseline-only
0/16 aggregate support result, found no J implementation or performance claim,
and agree that changing the frozen step/basin policy now would require new
prospective authority. Production remains HOLD. The findings require evidence
and lifecycle reconciliation; none authorizes implementing J or inferring a
performance result from the zero-support corpus.
