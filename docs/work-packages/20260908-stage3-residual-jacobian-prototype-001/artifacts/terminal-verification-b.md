# Independent terminal verification B — corrected cut

Static: corrected package/handoff/catalog lifecycle records; terminal finding
disposition; corpus, oracle, gate, line-count and source/build artifacts; frozen
detached oracle source; custody bundle v2. Ran: bundle verification, direct
source/hash/membership checks, exact corpus regeneration/comparison, the corrected
authentic oracle test, exact oracle-log reduction, line-count reconciliation, and
`git diff --check`.

Role/session: `terminal_verify_b`; configured/requested effort: independent
bounded verification; effective runtime settings: UNOBSERVED.

Reviewed identity: primary scaffold commit
`827a7470e058a5e09f9feced9375b776e5959789` plus the current terminal-correction
primary diff; detached head `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`;
corrected oracle blobs `b39fe2b56be540369fa7800fcd69224ebc7f95c0` and
`7786a69d1438bc0f6dbbbaf0df0fcd336958cd7e`; custody manifest v2 SHA-256
`ea4b42170f458bb24a3e073e734758cf52f320112ab49376bc5ea7e132726f26`.
Assigned scope: corrected corpus/oracle identity and outcome, custody/source
membership, focused behavior, lifecycle/catalog consistency, and cut freshness.

## Independent checks

- Ran `.venv/bin/python tools/agents/evidence_bundle.py verify
  /tmp/openwepp-rj-phase-a-terminal-bundle-v2` from `/workdir/openWEPP`: exit 0.
  The 17-member bundle contains 12 corrected source files, the exact active
  corpus, hashed protocol, exact oracle log, result record, and explicit
  no-executable marker. Its manifest digest matches the package and sidecar.
- Independently recomputed the two oracle Git blob IDs above and SHA-256 source
  identities `e8e5bc9e...ba35f` / `64d2b3df...539b`; they match bundle v2 and
  `source-and-build-manifest.json`. The protocol member's three hashes exactly
  match the current v33 entry (`47d4590b...b02f`), numerical-method chapter
  (`b05a332f...d680`), and package protocol (`7285ffd4...a275`).
- Ran `OPENWEPP_RESIDUAL_CORPUS_PATH=/tmp/openwepp-rj-terminal-verifier-b-v2.json
  nix develop --offline -c cargo test -p openwepp-land-surface-energy
  solver::tests::covered_v8_block_matches_frozen_joint_solution -- --exact
  --nocapture` in the detached worktree: exit 0, 1 passed, no Rust warning. The
  corrected test has explicit `candidates == 16` and `supported == 0`
  assertions, emitted exactly 16 unsupported-column records with nonempty
  missing-row sets, and ended `baseline v33 dry-stem oracle support: 0/16
  columns`.
- The regenerated corpus is byte-identical to the admitted corpus: SHA-256
  `a05ca09031c28f863bc8dd8f3cc8d605459a683abc2af6e0f37dd84c713068c0`.
  Independent earlier decoding remains applicable: 509,132 bytes, 8 records
  (4 Potential/4 FixedFinal), N=2/S=2/D=25, 4,636 tagged binary64 words, and
  positive 0.72/0.4176 dry-stem areas in every record.
- Independently reduced the retained exact oracle log: 16 unsupported lines,
  16/16 with nonempty missing-row sets, one exact 0/16 aggregate, and a passing
  test footer. This binds the aggregate to per-column evidence rather than
  report-only prose.
- Static inspection confirms the corrected smooth-mask checks shared-heat floor
  region, exact floor ties and sign/kink crossings; no frozen oracle constant
  changed. No candidate derivative source or executable exists.
- Recomputed relevant detached physical line counts: 364, 327, 1,909, 309, and
  2,685. They now match `line-count-governance.md`, including the >=2,000-line
  WARN rationale and explicit future split trigger. The package records the
  affected warnings-denied Clippy PASS. `git diff --check` passed.
- The package, handoff, work-package catalog, active locator, and root roadmap
  now consistently say terminal correction / proposed
  `INCONCLUSIVE_WITH_BOUND`, stop-before-J, no candidate/performance claim, and
  production HOLD. None prematurely says COMPLETE. `authentic-corpus.md` now
  leads with the admitted positive-stem corpus and expressly excludes the older
  inactive-stem capture.

## Findings and disposition

No remaining finding in the assigned corrected-cut scope. Prior TV-B-001 is
fixed and rechecked. The v2 source/custody correction, exact 16/0 assertions and
log, mask negatives, warnings/line-count evidence, admitted-corpus record, and
lifecycle/catalog correction all match the inspected state.

The current-scope Phase-A exit rule is legitimately decisive: zero of sixteen
physically active columns has a complete frozen reference basin, so candidate J
implementation and every dependent correctness/cost/timing/memory/scale gate
are ineligible rather than deferred or passed. `INCONCLUSIVE_WITH_BOUND` does
not reject the analytic representation or claim performance. Production remains
HOLD.

Uncertainty: bundle v2 provides byte custody, not a bit-reproducible rebuild;
that limitation is explicit. Effective model/runtime settings and total reading
exposure remain UNOBSERVED. No derivative correctness, J equivalence,
performance, scale, or production-readiness claim was verified or made.

Verdict: **PASS for the fresh corrected terminal cut and bounded
`INCONCLUSIVE_WITH_BOUND` disposition.** The exact active corpus, source and
protocol identities, 16/0 outcome, stop-before-J boundary, negative claims, and
terminal-correction lifecycle are independently verified. Final COMPLETE
publication may follow only after the remaining independent corrected-cut role
records are also PASS and the final lifecycle transition is reconciled.
