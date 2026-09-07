# Independent terminal verification A

Static: frozen acceptance, primary original Git objects, archived candidate, clause map, changed consumers, exercise answers, retained actual logs and exact terminal diff. Ran: independent pytest, real-candidate reconstruction/checker negatives, unit-consumer corruption, source/hash/mapping and range arithmetic described below. No verifier B artifact read.

Role/session: `/root/verify_a`, package_verifier; configured/requested effort medium; effective runtime settings UNOBSERVED.
Reviewed identity: substantive commit `3ec62445368a4c72bd242c6c98fb1d3a77cf6dd7`, original baseline `b932db101cce07d0860b45b5ecaa8ddb7f455b58`, plus the explicitly disclosed final format-status-only working delta. Assigned write: this artifact only. External temporary fixtures used; no source/configuration edits or push.

## Authority, freshness and preservation

Read root/local instructions, role-verification, package/kickoff/handoff, assurance template, testing strategy sections 7, 9–10 and 18, science instructions/obligations, correctness authority and directory-format specification. Acceptance is the frozen kickoff, not the parent report or current test outcome. The package declares no deferred gate. Phase 5 and format adoption gates require all affected checks and independent assurance before publication/reading-route activation.

Ran independent `.venv/bin/python - <<'PY'` audits from `/workdir/openWEPP`, exit 0 after the explicitly recorded diagnostic correction below. These read the original with `git show BASE:PATH`, inspect actual archived UTF-8 files, and independently compare mapped source/destination content; they do not execute `build_candidate.py` or accept its verdict:

- Original LSE is exactly 264863 bytes, SHA-256 `7a002bcac2ad640716b52f4bd59326f241d5fd3084d26e56f6a703eb496ed61d`; current canonical file equals that original Git object byte-for-byte.
- All ten original source-manifest objects match their pinned original lengths/hashes. Historical identities are verified against Git, not incorrectly required to equal an intentionally updated current status document.
- Archived entry plus 14 chapters total 410021 bytes. Independently computed SHA-256 of compact sorted-key JSON mapping repository-relative paths to file SHA-256, `separators=(',', ':')`, is `99ca67fb97351a7eaf495e89fc54720c314795acfbe7bcdc9c3ccab47a8596a8`, matching the reviewed candidate set.
- Source lines 26–3346 are mapped exactly once, without holes/overlap. All direct target lines match after the disclosed pipe-entity encoding; only original invariant header becomes a nondefining reference header and Change Log becomes a subordinate heading. History maps exactly original lines 1894–1918, not later still-binding amendments.
- All 100 actual marked definition Statements match their original statements. For prose obligations, the comparison removes the original ID/list prefix and joins source continuation lines; it preserves the remaining words and qualifiers. All 74 invariant Evidence cells retain explicit DIRECT/INFERENCE and Static/Ran classification. All 16 old BEI rows retain every cell except their intentionally pinned original-source locator.
- All 36 original fenced bodies remain byte-exact in the archived set. This establishes preservation of unindexed equations as well as ID rows. The mapping and actual target checks cover both original-to-current destinations and each mapped current canonical definition's original source. Added routing/guard/applicability metadata is separately covered by the accepted focused reviews and inspected correction locations; structural coverage alone is not scientific sufficiency.
- All seven live Rust consumers equal their original Git objects; live candidate chapter directory and shared Rust bridge are absent. Exact base-to-working diff outside this package is limited to checker/shared parser, unit/A0 consumers, Python tests, catalogs, and the final format-status paragraph. No production, other contract, historical package, Cargo, fixture, tolerance or frozen runtime identity change was found.

Static focused semantic checks agree with preserved clauses and corrected answers: V1 finite/equilibrium-zero CN distinction, immutable water and opposite first-node transfer; zero-duration versus pre-Newton 60-second admission; exact-bound stencil/no-update witness and V10-only exceptions; ingress-before-native-resident error precedence; V14 phase-before-liquid-only WB14; exact accepted physical operands before dyadic carry; historical v31 versus EXP-R/PC1/SG1 and retained production HOLD/owner pause. Independent original-to-target comparisons support reuse of the unchanged full-source preservation review; this verification does not claim a second human-style full 264863-byte semantic reread or an executed physical reconstruction.

The final `docs/specifications/science-contract-directory-format.md` change replaces the obsolete assertion that checker support is absent with implemented structural support, pilot HOLD and original whole-contract reading. Its observed SHA-256 is `4b7aa41200faaac647c8d8624597779a13c50bee444c0ca738ff7fd29653df1c`. Inspected exact diff: no grammar, adoption criterion or scientific authority change. ACCEPTED; source-manifest remains a historical snapshot. Later changes to substantive claims require reconciliation.

## Independently executed behavior

Commands below ran from `/workdir/openWEPP` unless the reconstructed tree is explicitly the working directory. `P=/workdir/openWEPP/.venv/bin/python`; `A` denotes this package's absolute `artifacts` path; `E=docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`. Variables here abbreviate exact paths, not alternate environments.

| Actual command/check | Exit/result |
| --- | --- |
| `.venv/bin/python -m pytest -q tests/python/test_check_sc_binding_exposure.py tests/python/test_sc_contract_directory.py` | 0; 83 passed in 3.50 s |
| `$P $A/restore_candidate.py /tmp/lse-verifier-a-93fjnja0/candidate` | 0; restored 40 verified files |
| Same recovery command with same existing output | 2; refused existing destination |
| `$P $A/restore_candidate.py /workdir/openWEPP/verifier-a-forbidden-recovery` | 2; refused checkout destination, no directory created |
| `$P /tmp/lse-verifier-a-93fjnja0/candidate/tools/check_sc_binding_exposure.py --strict /tmp/lse-verifier-a-93fjnja0/candidate/$E` from reconstructed root | 0; 17 BEI rows, 100 actual definitions, structural only |
| `$P /tmp/lse-verifier-a-93fjnja0/candidate/tools/release/check_sc_unit_compliance.py --path /tmp/lse-verifier-a-93fjnja0/candidate/$E` from reconstructed root | 0; no unit findings |
| Same strict command after deleting the actual `INV-LANDSURFACEENERGY-001` marked row in reconstructed interface.md, retaining entry and binding-index mentions | 1; missing explicit interface definition anchor, not mention-count PASS |
| Unit checker with `--path /tmp/lse-verifier-a-unit-0lqh3rdz/SC-X-001.md --registry-source /tmp/lse-verifier-a-unit-0lqh3rdz/registry.rs`, before/after removing soil alias row | 0 then 1; SCUNIT-E-011 for missing registered `soil_kelvin`/`soil_temp` mapping |
| `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | 0; restored LEGACY 16 BEI rows, not directory adoption |
| `.venv/bin/python tools/release/check_sc_unit_compliance.py --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | 0; restored legacy no unit findings |
| `.venv/bin/python -m pytest -q tests/python/test_sc_contract_directory.py -k 'qualifier_loss or admission or missing_entire_definition or unit_consumer'` | 0; 4 passed, 76 deselected; focused recheck of authority-identity and corruption cases |

The unit negative was independently invoked through the actual fixture builder and test function using importlib, `valid.__wrapped__(new_external_tmp)`, and `test_unit_consumer_reads_all_chapters_and_rejects_removed_mapping(entry)`, with subprocess argv/outputs observed. It proves the real consumer reads distributed chapter alias content. The full pytest run additionally executes public malformed-format/path/provenance/deferred/legacy negatives and the qualifier-loss semantic counterexample. A structurally legal qualifier deletion may PASS lint; that remains a preservation failure, not scientific approval.

Recovery safety inspection confirms confined manifest paths, hash verification before destination creation, rejection of existing/symlink/repository destinations, and an explicitly minimal structural/unit tree. Independently checked lengths/SHA-256 for all 29 overlay and 11 baseline-dependency files, and rechecked all 40 hashes after restoring temporary corruptions. This is not a complete Cargo checkout, executable reproduction, or new actual A0/Rust run from the recovered fixture.

Diagnostic failures retained in this record: an initial exploratory deletion removed the real candidate's unregistered `T_s | none | ... | gap` alias row; unit lint remained exit 0, so the surrounding diagnostic assertion exited 1. That was an unsuitable registered-alias poison, not evidence of a failed registered-alias guard. The separate real registered alias deletion above gives the required exit 1. An initial Statement audit assumed every original definition was a table row and raised IndexError on prose obligations; the corrected audit handles original prose explicitly and passes all 100. Neither failed diagnostic is reported as PASS or hidden. Temporary candidate content was finally restored and rehashed.

## Gate evidence and non-deferral

Static evidence reuse: read actual `logs/rust-baseline-comparison.log` and `logs/rust-candidate-reviewed.log`; independently parsed panic blocks keyed by test name, removing only process/location identity and the standard backtrace advice. Both contain exactly 18 panic identities and identical reason payloads. Actual summaries are 129 run, 111 passed, 18 failed, no skipped; no new observed candidate failure. The logs preserve the missing v31 production replay seam, unrelated retained authority failures and frozen executable-library identity failure. Raw selected nextest status remains FAIL, exit 100. I did not rerun that heavy Rust command.

Static: `logs/clippy-candidate.log` ends in 1136 production dependency errors. Those source files are unchanged by the terminal diff. Clippy remains FAIL, exit 101, not a passing lint or a silently suppressed warning set. Formatting and original/candidate A0 execution are retained evidence; this verifier independently reran the focused real-Git A0 fixture via pytest, not the complete historical Cargo/A0 workflow. No new full correctness campaign is selected solely for this document/parser work, and no external suite bindings/cohort fixtures were altered. Test-only line-count warning/disposition is proportionate; no live Rust edit remains.

Both primary acceptance and the non-deferral rule prohibit replacing required PASS with equal failure sets. Fixing absent production code, frozen runtime identity or unrelated scientific expectations is outside this package's protected scope. The explicitly authorized recovery is therefore legitimate: executed-HOLD, original v31 canonical, whole-contract reading retained, candidate archived. Missing current requirements are not moved to a later boundary or counted complete. This does not resume the owner-paused experiment or block unrelated authorized architecture.

## Exercise legitimacy and measurements

Independently read the frozen four-task rubric and recorded exercise answers/corrections, checked relevant primary clauses above, and recomputed selected bytes directly from Git-original and final archived files/ranges using context-selection.json as the declared selection, not trusting report totals. Baseline figures are equivalent required-source inventories, not observed baseline-agent sessions.

| Task | Equivalent baseline bytes | Candidate selected bytes | LSE bytes | LSE change from 264863 |
| --- | ---: | ---: | ---: | ---: |
| Surface/soil | 2438218 | 2494374 | 321019 | +21.2019% |
| Solver/replay | 1170590 | 1200346 | 294619 | +11.2345% |
| Executable identity | 333482 | 168600 | 99981 | -62.2518% |
| Physical-closure requirements | 1184037 | 1217616 | 298442 | +12.6779% |

Independent inclusive UTF-8 range arithmetic additionally reproduced task01 new terminal 26276 and nonlinear repeat 7704 bytes; task02 added custody headers 3393 and second range reread 29320 bytes; task04 listed recovery 202918 = 191997 duplicate-request + 10921 newly requested WATBAL361–390; final liquid-only supplement 1314 bytes. This verifies named source/range accounting, not exact delivered repetition after truncation. Task04's separately reported 880 search-exposed source bytes are excluded consistently from the inventory table.

Final bounded answers identify required rules, tests, operands, boundaries and expansion triggers. Task01's support admission, task02's V10/reuse/norm/witness/error-order details, and task04's zero/phase/liquid-only/chronology/exact-wire qualifiers required corrections. Those initially failed or incomplete attempts remain in exercise/review records. Subsequent same-agent corrections are carried-context answers, not new fresh first-pass successes. Initial four nonforked/read-only provenance is recorded, but I have no independent session telemetry proving every originally displayed byte or absence of hidden context. No stronger freshness/runtime claim is certified.

Task03 is a sufficient smaller identity-requirement route; it does not establish actual executable custody, physical closure, replay activity, performance or promotion. Task04 selects reconstruction requirements and explicitly lacks full retained p61 primitive inputs; it does not claim to have executed that reconstruction or independently validated an unspecified temperature-to-sublimation-enthalpy family. Actual implementation/scientific verification would trigger additional authority and tests. I found no applicable frozen requirement omitted by the final bounded selections inspected. Their three larger physical/solver selections cannot support a blanket context-saving claim or compensate for the failed adoption gates. Full runtime context, token and quota savings remain UNOBSERVED.

## Findings, disposition and verdict

No new open defect found within this terminal scope. Accepted reviewer corrections are supported by exact candidate identity, direct preservation checks, actual negative execution and corrected requirement selections. Final status-document correction is accepted on its explicit hash/diff. Packaging may update pending-terminal-assurance wording after both roles finish; it may not convert these bounded results or inherited failures to adoption PASS.

Verdict: **PASS for the corrected cut as a truthful executed-HOLD delivery with restored original authority and retained checker/candidate evidence. Adoption remains HOLD; required Rust and Clippy gates remain FAIL.** This artifact fulfills verifier A only and does not certify another verifier's work, a production scientific result, or unobserved workflow savings.
