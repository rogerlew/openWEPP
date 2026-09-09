# Independent terminal verification B

Static: root/common/package/worker handoff, role-verification, assurance template,
testing strategy sections 7, 9–10 and 18, correctness-authority model, applicable
science-obligations, science instructions, capture instructions, changed binding
tests, source-real derivative tests, finding dispositions and final reviews.
Ran: independent retained-binary negative/assembly checks, bundle verification,
fresh A-to-J patch reconstruction, cut3/cut4 comparison and raw memory arithmetic.

Role/session: `/root/terminal_verifier_b`, assigned package_verifier B.
Configured effort: medium per role; effective runtime effort: UNOBSERVED.
Write ownership: this artifact only; no primary or experimental source changes.

## Reviewed identities

Primary base: `8fd102ee00f36f3d5fbe2fc88135ba43e608cc20`, scoped dirty cut.
Measured J source index: `7b5133c93de4c827d54945e265b94235834bffef4eb21ec62942b3c377930d57`.
Terminal cut4 index: `0c582dd73d12434faf2d0394f320c5247633f1966538b347844dbe78bb9df70a`.
Terminal bundle manifest: `17fbc3a85c9c53088734975f19999890e61d9cf52ba7cebe280e512f46f04764`.
These hashes were independently checked, not accepted from the handoff alone.
Exact retained LSE executable used below:
`/workdir/.cache/openwepp/targets/J-fd4da896059c/release/deps/openwepp_land_surface_energy-29dc6221b95c33c1`,
SHA `69b87a51a6c87ca49fe09a81ebe901cfddc38b635a467928f66c6bcad9a1a093`.
Corpus `/tmp/openwepp-hotpath-jacobian-jmvOZh/authentic-corpus-v2-cut2.json`,
SHA `000e59189d62fdc20a89fcefb80ecf62fdb060b7b3e8c8afa99a37bd92c932ec`.

## Independently executed checks

Commands ran from `/workdir/openWEPP`, with output retained in this session's
tool record. No rebuild, benchmark sample, heavy campaign or new tolerance.

1. Retained LSE executable with
   `solver_residual_corpus_capture::tests:: --test-threads=1`: exit 0,
   **10 passed**. Includes live-finish/stale-drop session protection, malformed
   decoded participation, lifecycle/count reconciliation and overflow refusal.
2. Same executable with `solver::inactive_jacobian_tests:: --include-ignored
   --test-threads=1`, environment `RUST_MIN_STACK=67108864` and
   `OPENWEPP_INACTIVE_JACOBIAN_CORPUS` set to the corpus above: exit 0,
   **7 passed, none ignored**, 0.04 seconds. Independently inspected source
   assertions cover all three participating families, actual nonunit provider
   mapping, source-real positive-area/wet-only/negative-zero FD retention,
   stale/foreign/mismatched/second-use refusal, original preprocessing errors,
   atomic invalid-scale/normalizer/tangent and malformed matrix rejection,
   complete columns and post-adjustment matrix/RHS identity. An initial command
   without `--include-ignored` passed only 3 and ignored 4; it is not substituted
   for the explicit seven-test run.
3. `.venv/bin/python -m unittest discover -s
   docs/work-packages/20260908-stage3-hotpath-jacobian-reference-and-prototype-001/artifacts/reproduction
   -p 'test_admit_identity_v34.py'`: exit 0, **3 passed**. Stale executable,
   foreign checkout and manifest substitution fail; no inherited F counter
   exception erases a changed frame counter.
4. `.venv/bin/python tools/agents/evidence_bundle.py verify
   /tmp/openwepp-hotpath-jacobian-jmvOZh/j-terminal-bundle`: exit 0.
   This validates byte custody, not scientific correctness or rebuild identity.
5. Fresh scratch `/tmp/openwepp-verifier-b-patch-BHNPQQGj`: extracted only the
   manifest's eleven existing A inputs from retained terminal A-source archive;
   applied primary `j-prototype-cut3.patch` with `patch --batch --fuzz=0 -p1`.
   All **16 reconstructed J file hashes match** the declared cut3 hashes,
   including five new files. Independently hashed current J's same 16 paths:
   all match. Scratch is retained; no original tree was overwritten.
6. Compared all **37,386 cut3/cut4 index entries** using `jq` transpose and
   inequality selection: exactly one changed member, the LSE authority binding
   test. Direct archive-member-versus-current diff shows assertion wrapping
   only; strings and assertions remain unchanged. Cut4 member hash is
   `ddce942406d6f84477fdab66cb7f958822d00118bc8e6ab03f452bc4e4865acf`.
   Full bundle verification plus fresh selected-file reconstruction supports
   documented recovery; I did not extract the entire 3.36 GB tree or rebuild it.
7. Recomputed pair-0 active peaks from raw teardown `results.jsonl` maxima:
   A 83,788 KiB, J 88,276 KiB; extra 4,488 KiB, frozen bound
   `max(4096, 0.05*83788)=4189.4 KiB`, excess **298.6 KiB**. Both rows are
   valid/exit 0, CPU0-pinned exact runner invocations with separate A/J cwd.
   Passing lifetime peaks cannot discharge this failed criterion.
8. `git status --short`, exact three primary binding-test diffs and
   `git diff --check`: scoped authority/binding/package changes only, no primary
   crate/Cargo edit; whitespace check exit 0. Primary historical v33 assertions
   retain historical authority while current version pins become 34. Terminal
   formatting is distinct from the measured runtime, not a new measured cut.

## Requirement legitimacy and accepted fixes

CAP corrections and derivative guard/scaling/exclusion fixes have independent
behavior execution above, not merely artifact-presence checks. Final narrative
refresh and closed READ-A01 record were reread after the parent's freeze;
the late SnowEnergy reading is not represented as preimplementation completion.
Canonical science review remains the assigned dual reviewers' bounded work;
this verification does not claim a new full-owner physics audit.

Raw full2 log explicitly reports 4,044 passes and 179 failures, including its
named manual SIGINT. The package correctly separates 178 ordinary failures plus
that interruption. Focused retry log reports canonical 720.022-second TIMEOUT,
not a numerical failure or replacement full-workspace pass. Five-case matched-A
attribution is limited to five; other runtime failures remain UNATTRIBUTED.
I inspected those terminal logs, not reran the expensive campaign.

Non-deferral checked: failed required broad/lint/authority gates remain FAIL
and prohibit complete/production qualification. Executed-HOLD is legitimate:
the actual J was implemented, admitted on its bounded domain and measured;
the frozen memory criterion decisively rejects this implementation. Authorization
forbids unrelated historical cleanup and says to stop tuning an uncompetitive
implementation. Conditional competitive 10/19-OFE timing is not triggered by
this rejected cut; it remains NOT QUALIFIED, not a retroactively waived gate.
No active-foliage, full soil/publication closure, persistent-memory boundedness,
or modifiable whole-run fraction claim is justified or made. The next reduced
system proposal requires separate authority; no current resizing is authorized.

## Disposition

Findings: no new blocking finding for this bounded rejected-implementation
handoff. Existing memory/gate failures are intentionally retained, not fixed or
waived. Terminal publication bookkeeping and final scoped commit checks remain
the parent's responsibility; changed substantive claims would reopen verification.

Uncertainty: external toolchains/caches/Python remain required; byte recovery
does not establish standalone or bit-reproducible rebuild. Historical collector
outer exits marked UNOBSERVED remain unobserved. Independent checks did not
rerun timing, full regression, all raw closure reconstruction, or the other
verifier's authority audit.

Verdict: **PASS for the independently checked recovery/negative-behavior evidence
and truthful REJECTED_IMPLEMENTATION / executed-HOLD disposition.** Not PASS for
full-green package completion or production activation. Production remains HOLD.
