# Independent terminal verification B

Static: package, handoff, gate/result/custody artifacts, frozen oracle source,
current package/catalog status, and testing/work-package closure rules. Ran:
independent corpus decoding and bit-count inspection; SHA-256/size checks; evidence
bundle verification and member-to-live-source comparison; both frozen focused
oracle tests.

Role/session: `terminal_verify_b`; configured/requested effort: not recorded;
effective runtime settings: UNOBSERVED.

Reviewed identity: primary package cut at scaffold commit
`827a7470e058a5e09f9feced9375b776e5959789` plus its current uncommitted package
and v33 authority diff; detached Phase-A head
`e89befa4678eadec039b3e7f7fe0a176af8e9dc5`; custody manifest SHA-256
`488eaff372f48802efc1ccf59a7deb8d5fdd27aacf25d06900f5896e0c8293ad`.
Assigned scope: corpus shape/activity/identity, 0/16 oracle result,
source/custody membership, and package claim/status legitimacy.

## Independent checks

- Ran a separate JSON walk over
  `/tmp/openwepp-rj-active-stem-corpus-v1.json`: SHA-256
  `a05ca09031c28f863bc8dd8f3cc8d605459a683abc2af6e0f37dd84c713068c0`,
  509,132 bytes, schema `openwepp.covered-residual-corpus.v1`, 8 records split
  4 first/4 last and 4 Potential/4 FixedFinal, and 4,636 tagged binary64 words.
  Every record independently decoded as N=2, S=2, D=25 with 25 residuals;
  both stem areas were positive and exactly 0.72/0.4176.
- Ran `.venv/bin/python tools/agents/evidence_bundle.py verify
  /tmp/openwepp-rj-phase-a-terminal-bundle` from `/workdir/openWEPP`: exit 0.
  A second direct member audit found 16 entries (12 source, one fixture, one
  protocol, one result, one no-executable marker), no blob hash/size mismatch,
  12/12 source members byte-identical to the detached worktree, and the bundled
  fixture byte-identical to the external corpus. The manifest hash matches its
  sidecar. The two declared oracle Git blob identities independently recomputed
  as `3ae38b0830472d404b4378c7fea82a08172182f4` and
  `8acff12d8e571914c6ced1f69973c0d8e99d3ad6`.
- Ran `nix develop --offline -c cargo test -p openwepp-land-surface-energy
  solver_stem_jacobian_tests::v33_oracle_selector_self_tests_precede_candidate_code
  -- --exact --nocapture` in the detached worktree: exit 0, 1 passed.
- Ran `OPENWEPP_RESIDUAL_CORPUS_PATH=/tmp/openwepp-rj-active-stem-corpus-v1.json
  nix develop --offline -c cargo test -p openwepp-land-surface-energy
  solver::tests::covered_v8_block_matches_frozen_joint_solution -- --exact
  --nocapture`: exit 0, 1 passed, independently emitted `baseline v33 dry-stem
  oracle support: 0/16 columns`. The corpus hash remained exact afterward.
  Static inspection confirms each positive-stem column is admitted only when
  every prospectively declared affected row has a frozen nine-scale basin; no
  candidate derivative participates in this audit.
- Static negative check found no `solver_stem_jacobian.rs` candidate source and
  no candidate capability symbol outside the baseline-only tests. The custody
  result and no-executable marker therefore support the bounded claim that J and
  downstream cost/timing/memory/scale work were not executed.

## Findings

**TVB-001 — HIGH — terminal status is prematurely closed and conflicts with
the package's authoritative locators.** `package.md:3` says `Status: COMPLETE`,
while its final progress item remains unchecked, `worker-handoff.md` says
terminal review is pending, no terminal-review/verification artifacts existed
at this reviewed cut, and `active.md`, `README.md`, and `docs/ROADMAP.md` still
classify the package as active. Work-package governance expressly makes dual
review and dual verification a prerequisite to completion. Correction: keep the
package executing/pending until all four independent records and dispositions
exist, then reconcile package, handoff, active locator, catalog, and roadmap on
one corrected stable cut and reverify freshness.

## Disposition

The scientific/evidentiary result is independently supported: the authentic
positive-stem corpus and frozen baseline oracle establish **0/16 admission**, so
stopping before candidate J and reporting downstream gates as NOT RUN is a
legitimate bounded negative result, not a waiver or PASS. TVB-001 remains open
on this cut; no current-scope terminal-role requirement may be deferred while
claiming `COMPLETE`.

Uncertainty: this verification does not establish bit-reproducible rebuilding,
which the custody manifest explicitly disclaims. The rerun reported two
`unused_mut` warnings in detached observation code; no warnings-denied claim was
accepted here. Workflow-total reading and runtime metadata remain UNOBSERVED.

Verdict: **FAIL for terminal package closure on the reviewed cut** because
TVB-001 makes the completion/status claim untruthful. **PASS only for the bounded
Phase-A evidence claim**: exact active corpus, 0/16 frozen oracle admission, no J
candidate, and downstream measurements NOT RUN.
