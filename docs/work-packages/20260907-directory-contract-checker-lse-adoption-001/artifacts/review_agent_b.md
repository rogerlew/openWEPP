# Reviewer B — initial parser and consumer inventory

Static: complete directory-format revision 1; existing binding checker and its three Python tests; unit checker; cited live consumer dataflows; parent metadata/invariant/BEI and provenance requirements.
Ran: instruction discovery, source searches/reads and HEAD identity only; no behavioral validation.
Role/session: independent Reviewer B (/root/review_b); requested QA effort medium; effective runtime settings UNOBSERVED. Reviewer A's findings were not read.
Reviewed identity: initial source inventory at HEAD 62f0d07e86a6844c883b6df782d505eec8a020da (package scaffold after baseline); no parser or migrated candidate reviewed.
Assigned write set: this artifact only.

## Initial findings and implementation pitfalls

These are prospective acceptance risks, not claims that unimplemented functionality is defective. Owner for all: package implementer; disposition: OPEN pending implementation and focused review.

| ID / severity | Location and evidence | Required resolution |
| --- | --- | --- |
| B-P01 / high | tools/check_sc_binding_exposure.py:43-57 decodes with replacement and treats every pre-BEI ID mention as existence. Directory specification requires strict UTF-8 and canonical rows. | Keep meaningful legacy outcomes but dispatch before the legacy mention scan. Unknown, duplicate, malformed format metadata must hard-fail. Test retained registry/BEI mentions after deleting the actual definition. |
| B-P02 / high | Directory specification Layout and membership permits normalized dependency parent traversal but prohibits membership traversal, symlinks and undeclared files. | Check components before reading or resolving through symlinks, including entry/ancestor and nested-directory symlinks. Enumerate for exact membership without treating discoveries as authority. Reject FIFO/device/socket and unreadable/non-UTF-8 files without hangs or traceback. Test external sentinel targets and dependency escape separately from member escape. |
| B-P03 / high | Directory specification binding grammar distinguishes true marked rows, ordinary mentions, aliases and fenced examples. | Preserve fence delimiter type/length, up-to-three-space indentation, info strings and matching closing rules. Wrong first-cell anchor/ID, duplicate anchors, standalone/heading declarations, wrong table header, extra/missing cells and definition padding must fail. Fenced registry/index/table headings must not alter parsing state. |
| B-P04 / high | science-contract-spec.md:231-282; sidecar provenance Required Fields/Lifecycle/Lint. Existing checker validates neither review-gate vocabulary nor sidecar fields. | Validate provenance sidecars, source entry correspondence and supersession pointers; allowed review gates include none/flagged-binding-addition/science-review-follow-on. Deferred residue must remain normative with owner/next gate. No optional Notes heuristic substitutes for an owned queue/source binding. |
| B-P05 / high | tools/release/check_sc_unit_compliance.py:376-497 reads one file and only first Variables and Units / Symbol Alias Map section and first table. | Route to all applicable normative table owners with original source locations. Naive concatenation can silently omit later tables. Keep registry canonical-symbol, alias and expected-unit checks. Corrupt a chapter mapping and demonstrate real unit-lint failure. |
| B-P06 / high | Rust readers below use substring/row/heading searches on the old monolith. | Preserve scientific assertions on canonical content. Entry aliases or repeated test sentences cannot satisfy them; no directory scan or historical aggregate is authoritative. Corrupt a protected chapter obligation and demonstrate corresponding consumer failure. |
| B-P07 / medium | Directory specification Reading Routes/Adoption Gates; kickoff phases 4-5. | Freeze original rubric before candidate exercises; require fresh non-forked primary-module readers and inspect dependency expansion. Byte accounting includes required instructions, destination entries, repeated reads and recursive dependency union. A smaller entry alone is insufficient. |

## Live consumer inventory and dataflow

Ten Rust files directly read current LSE authority and need reconciliation (paths relative to tests/integration/):

- land_surface_energy_balance_authority_contract.rs: read(CONTRACT) at lines 5/86 and throughout, row at 526, exact section-boundary and whitespace expectations; lines 739-749 assume legacy invariant row prefixes. Relocated BEI, gaps, equations, status and successive amendment bodies feed substantive assertions. Definition markup changes require row parsing changes without weakening contents.
- solver_architecture_authority_contract.rs:133-161: text(path).contains(INV...) in adjacent-contract loop. An alias satisfies this existing ID-only check while the definition is missing; canonical loader binding is necessary.
- surface_liquid_hydrology_custody_authority_contract.rs:235,278,457,511: read(LSE) ID assertions on custody/exact enthalpy authorities.
- snow_stage3_terminal_receiver_authority_contract.rs:5,116-117,230: LSE constant enters contract reads and multi-contract authority checks for terminal receiver rules.
- snow_stage3_shared_carrier_authority_contract.rs:118,230-235: LSE joins shared-carrier corpus and direct receiver invariant checks.
- snow_stage3_terminal_chronology_v19_contract.rs:34,80-82,150: current LSE read plus exact Binding Exposure Index to Gap Register slice; inventory-order aggregation alone may break or enlarge that slice.
- snow_stage3_terminal_batch_temporal_v20_contract.rs:30,65-68,129: shared temporal corpus and separate LSE reads.
- snow_stage3_terminal_batch_temporal_v21_contract.rs:31,74-75: analogous temporal invariant/corpus checks.
- vegetation_boundary_authority_contract.rs:534,560,865,946: generic adjacent-contract reads, energy/exact-one checks, later authority loops. Preserve historical JSON/model-stack reads separately.
- stage3_native_vegetation_laned_throughput_recovery.rs:1255-1271,1350: direct fs::read_to_string(repository.join(path)) bypasses a generic read helper; requires LSE version 31, IDs and replay-fallback prose. Revision expectation must track approved migration revision while its substantive clauses survive.

Additional affected or preservation-only consumers:

- tools/release/check_sc_unit_compliance.py and wrapper .sh; real Rust harness tests/integration/hphys0279_sc_unit_compliance_lint_contract.rs invokes Python and tests diagnostics/registry coverage. Preserve its legacy fixtures and add directory corruption coverage.
- tools/release/check_science_contract_admission.sh:91-113 enumerates retained entry files and reads entry front matter. It need not consume chapter science, but full target metadata and sole entry lifecycle must keep it working.
- v10_nighttime_authority_contract.rs:44 checks a frozen JSON identity (SC-LANDSURFACEENERGY-001@4), not current entry content; do not rewrite it as a new version.
- Live neighboring authority references occur in SC-SNOWENERGY, SC-VEGETATION, SC-SURFACELIQUID, SC-COUPLEDTIME and SC-VEGETATIONTRANSACTION. They include logical ID anchors and expressly versioned references; aliases can preserve current logical IDs, while pinned/versioned scientific identities must remain intact. Many references are code spans rather than Markdown links, so Markdown-only discovery misses them.
- science-contracts/index.md owns lifecycle locator. Relevant whole-contract instruction/template routes require atomic final adoption reconciliation; no route change is qualified by this inventory.

## Bounded grammar details needing explicit decisions

The format fixes metadata fields, inventory/definition/coverage columns and path confinement, but not every serialization edge. Record narrow implementation interpretations before relying on them: front-matter scalar/list syntax and duplicate-key handling; repository-root selection for isolated external fixtures; source identity syntax for original Git spans versus local sidecars; how provenance template headings provide entry_id/title/summary; unique section anchors versus HTML anchors; structural syntax for owned deferral queues and concrete applicability. Do not replace these with keyword-based scientific adequacy tests.

Coverage targets must identify real normative sections, not arbitrary anchors in prose or a registry alias; aliases resolve one hop to actual sections/definitions. N/A is conditional under the original schema, not a global escape from any of the 32 keys. The entry can own definitions but cannot appear in its own inventory; chapters cannot create competing versions.

Adversarial cases include malformed front matter still containing contract_format, duplicate section/table headings, fences closed by too-short or wrong delimiter, indented fenced fake tables, literal pipes versus HTML entities, aliases to aliases/history, dependency escapes, undeclared hidden files, nonregular filesystem entries, missing chapter content with retained IDs, invalid sidecar fields/status, false supersession, deferred narrative removed, default/strict verdicts and usage exit 2. Qualifier removal with all structural IDs intact remains a semantic counterexample rather than prompting a keyword checker.

## Scope and non-deferral

Verdict: NOT RUN for parser conformance, migration preservation, fresh-agent usability and terminal adoption; this is an initial source-backed inventory only. No package acceptance is waived or deferred. Full starting LSE preservation reading and stable-cut parser/consumer review remain required at the later checkpoint; this preliminary bounded task does not certify those duties. No Rust edits in this review. Accepted implementation fixes and final source identity require focused follow-up review.

## Interim checker adversarial review

Static: initial implementation of tools/sc_contract_directory.py, checker dispatch and full tests/python/test_sc_contract_directory.py. Ran: from /workdir/openWEPP, .venv/bin/python -m pytest -q tests/python/test_check_sc_binding_exposure.py tests/python/test_sc_contract_directory.py: exit 0, 45 passed in 1.55 s. Independently ran isolated public CLI mutations via .venv/bin/python heredocs importing the test module through importlib and invoking valid.__wrapped__(temporary Path), mutate(), and run(entry, True). No source/test edits. Parent edited the checker concurrently after initial findings; these are reproducible defect reports against observed interim cuts, not a stable-cut qualification. Assigned artifact is the only retained reviewer write.

Every case below unexpectedly returned strict PASS (exit 0) unless stated. The positive fixture passed before mutations. Disposition: OPEN, parent notified; fixes require focused rerun.

| Finding / severity | Reproducer mutation and source location | Required result |
| --- | --- | --- |
| B-C01 / high | ContractSet.validate provenance block: add declared history.md with Source [original](history.md#old), anchor then heading and six required bullet fields; change sidecar status to potato and canonical_binding_ids to INV-X-999. | Hard FAIL: validate sidecar own vocabulary and IDs, not just BEI row. |
| B-C02 / high | Same valid history fixture, sidecar status superseded, no superseded_by, while BEI status remains active. | Hard FAIL; pointer requirement must follow sidecar status and agreement, not only BEI status. |
| B-C03 / high | Append an unindexed anchored second sidecar heading with missing fields. | Hard FAIL; required sidecar-to-BEI reverse coverage is absent. |
| B-C04 / high | Replace Source Markdown link with raw history.md#old and remove all sidecar fields. | Hard FAIL or unsupported source syntax; raw spelling must not bypass provenance/source resolution. |
| B-C05 / medium | Historical sidecar title and summary removed while six checked bullets remain. | Hard FAIL: required title/summary/entry identity cannot be silently omitted. |
| B-C06 / high | BEI mapped IDs INV-X-001-BOGUS, OBL-X-P-001-BOGUS. ID_RE.findall extracts valid substrings. | Hard FAIL; parse complete comma-delimited IDs. |
| B-C07 / high | Prefix actual invariant table with an HTML comment opener. | Definition cannot count from commented-out Markdown; reject unsupported comments around structural declarations or exclude comment content. |
| B-C08 / medium | Empty Reading routes row; move none beyond entry out of Dependencies into unrelated section; table separator reduced from six to two columns; coverage heading replaced with #NotAHeading. | Hard FAIL for each structural violation. |
| B-C09 / medium | Unmatched opening quote in contract_format accepted by strip; invalid lifecycle status potato accepted. | Reject malformed metadata and validate applicable lifecycle vocabulary. |
| B-C10 / medium | Alias targets arbitrary prose anchor; alias Target has valid first Markdown link and broken second link. Registry also selects targets(...)[0]. | Fail nonsection/nondefinition targets and nonsingular alias/registry targets. |
| B-C11 / high | Dependency points to ../SC-Y-001/history.md#old, whose owning SC-Y-001.md inventory explicitly declares historical. resolve creates kind external and uses path-name heuristics. | Fail historical-only authority across contract boundary too; inspect bounded owning membership rather than trusting filename absence of provenance. |
| B-C12 / medium | Deferred Notes owner: ; next evidence gate: ; retained: surface.md#surface. | Hard FAIL for empty owner/gate. Observed default PASS-DEFERRED / strict exit 1, which incorrectly admits invalid deferral as binding-safe. |

The provenance fixture was built by adding the history row immediately after the existing bindings inventory row (without a blank line), then replacing the existing Source link. History baseline was anchor old, heading source Historical context, status historical, source_package original, effective_date 2026-09-07, verdict historical, canonical_binding_ids none, provenance_anchors original, and Summary retained. All six reported provenance mutations passed independently. An earlier probe inserted an inventory row after a blank line and correctly failed malformed-table parsing; those malformed fixtures are excluded from findings.

Bounded interpretation assessed: unique conventional Markdown heading fragments for existing neighboring dependencies are acceptable as a documented compatibility clarification, since explicit HTML syntax is mandated specifically for definitions. Fix the slug algorithm, reject duplicate/colliding explicit and heading IDs, never infer fragments from link labels, and never admit a heading as a binding definition. Newly migrated sections should retain explicit HTML anchors. This avoids unnecessary neighboring scientific/version edits.

Verdict: FAIL for interim checker conformance due to the false passes above. Existing 45-test PASS does not establish the full required matrix: its missing_provenance mutation is also historical-only obligation/dependency corruption and can fail before reaching provenance code. Add independently valid sidecar fixtures and isolated mutations. Migration, consumer reconciliation and reading usability remain NOT RUN and are not certified by this review.

## Focused re-review at 64833dd7d

Static: complete corrected shared parser and bounded interpretations in checker-consumer-conformance.md. Ran: .venv/bin/python -m pytest -q tests/python/test_check_sc_binding_exposure.py tests/python/test_sc_contract_directory.py, /workdir/openWEPP, exit 0, 71 passed in 2.56 s. Source HEAD 64833dd7d5dad025bb2c98def6b524b9817643e9; worktree changes were assigned assurance reports plus preserved unrelated untracked directories.

Independent public CLI reproductions rejected all 15 original mutations: empty routes, displaced dependency marker, malformed mapped IDs, commented definitions, separator width, unmatched metadata quote, fake section heading, invalid lifecycle, historical status/ID/supersession/unindexed entry/title/summary, and raw provenance target. Separately re-executed first-level neighboring history, multiple alias targets, empty deferral ownership and prose alias cases; all rejected. Fixtures were isolated TemporaryDirectory trees, created from valid.__wrapped__ and with_history.__wrapped__, positive history fixture checked before each mutation.

Disposition: B-C02 through B-C10 and B-C12 original reported defects are FIXED / independently rechecked. B-C01/B-C04 provenance and B-C11 history resolution fixes address the original cases but remain incomplete for cross-contract/nested cases below. No claim of migration or consumer readiness.

| New finding / severity | Independent counterexample at this exact cut | Required correction / disposition |
| --- | --- | --- |
| B-C13 / high | Dependency ../SC-Y-001/archive/history.md#old; SC-Y-001.md inventory explicitly declares SC-Y-001/archive/history.md historical. Returned strict PASS. resolve infers owner only from immediate parent archive.md, missing owning ancestor SC-Y-001.md. | Resolve the owning declared contract entry for permitted nested members and preserve its kind; OPEN, parent notified. |
| B-C14 / high | BEI Source [original](../SC-Y-001/history.md#old) points to neighboring declared historical chapter with heading/anchor but no provenance fields. Returned strict PASS. resolve reads owner membership with normative=False, then constructs target kind external, so validate_provenance ignores it. | Carry recognized historical kind through source resolution and validate sidecar fields/reverse coverage; OPEN, parent notified. |
| B-C15 / medium | Existing explicit anchor surface and its adjacent Surface heading remain; append another distant Surface heading. Returned strict PASS. Document suppresses every heading slug whenever any explicit anchor with that slug exists, contrary to the documented adjacent-same-section exception. Separately, an intervening prose line between coverage anchor and heading also passes the two-line proximity heuristic. | Recognize actual same-section adjacency (blank lines allowed, intervening prose not); reject competing distant heading/explicit targets. OPEN, parent notified. |
| B-C16 / medium | Valid historical effective_date replaced with 2026-99-99. Returned strict PASS. | Require a real calendar date, not only digit widths; OPEN, parent notified. |

The newer provenance fixture has a separate historical BEI row with historical status and none mapping, agreeing with sidecar fields, while the original active BEI row remains. This avoids the former fixture's newly prohibited status disagreement. Cross-contract negative fixtures use a minimal readable neighbor inventory, which the bounded resolver already treats as sufficient for ownership classification; their missing full neighbor metadata is not the cause under test.

Verdict: FAIL pending B-C13 through B-C16. No gate is waived or deferred. The 71-test run and verified original fixes remain valid evidence for their bounded claims; added cases and resulting fixes require focused re-review, not automatic reuse as terminal conformance.

## Candidate parser and consumer re-review

Static: corrected parser, unit checker diff, seven Rust consumer diffs and shared Rust bridge; baseline/candidate logs and restoration record; v19/v20/v21 read dataflows. Ran: current full Python selection produced 76 passed / 1 failed in 2.94 s; the new whole-set corruption assertion incorrectly requires diagnostic text "actual definition", while the correctly rejected removed obligation reports "missing explicit anchor: soil.md#OBL-X-P-001". Parent notified to fix this diagnostic assertion without weakening rejection.

Ran independently: strict real LSE checker PASS (17 BEI rows, 100 definitions); real LSE unit checker PASS. Individually repeated B-C13 nested historical dependency, B-C14 external sidecar, B-C15 distant collision, B-C16 invalid date, and distributed unit-alias deletion public-CLI tests: all correct. B-C13..16 disposition FIXED / independently rechecked. Removing blanket work-packages-directory rejection is a legitimate bounded correction: frozen kickoff requirements can impose execution/capture duties. This does not make package evidence science authority; declared historical classification/provenance rejection remains, and dependency rows must state the execution boundary and source identity.

Reviewed source hashes:
- tools/sc_contract_directory.py: 0f5af549473187fb0687b074850eb7ea511f7272b34b93962ec9f422d45e2633
- tools/release/check_sc_unit_compliance.py: 2999db488e3723f6aa8c4dfe31469bcf9692f46e1b8127eaa980435e69f42fbe
- tests/integration/support/sc_contract_text.rs: 1fb802070812f5fc198a5749b0ac9877618903f83e3ad41397384843ee07285b

Consumer review: unit loader selects all declared normative Variables and Units / Symbol Alias Map tables, merges only relevant columns and keeps original row origins. Its two-chapter regression passed independently: deletion of soil_temp -> soil_kelvin alias fails SCUNIT-E-011. Rust tests use a deterministic parser-produced normative aggregate; source text assertions retain scientific strings, with whitespace/HTML-pipe normalization only. Revision expectations advance to 32, while invariant 164 definition and guard-map multiplicity checks now separately enforce one canonical marked definition and one original guard mapping. No fixture/tolerance/physics assertion removal observed.

Ran an independent actual Rust bridge proof, not merely Python-helper proof: temporary standalone harness includes the unchanged shared bridge by absolute source path; CARGO_MANIFEST_DIR=/workdir/openWEPP; compiled with nix develop -c rustc --edition=2021 <temporary bridge.rs> -o <temporary executable>, exit 0. Valid isolated fixture returned BRIDGE PASS / exit 0 and included "Receive transfer once". Deleting the canonical obligation row from soil.md while entry/registry mentions remain made the compiled bridge exit 1 with the missing canonical anchor diagnostic. Temporary fixture/harness were removed automatically.

New finding B-C17 / medium / OPEN: support/sc_contract_text.rs uses its own line.starts_with("contract_format:") dispatch. A malformed declaration "contract_format : directory-v1" returns raw entry text instead of invoking canonical parsing, allowing ID/alias-only consumer checks to bypass malformed directory rejection. Route recognized SC entry files through the shared parser regardless of a shallow metadata prefix; retain raw reads for ordinary source/package files. Parent notified.

Inventory correction: the earlier broad search overcounted three live readers. snow_stage3_terminal_chronology_v19_contract reads Git object 83fb00514e8932561bee5aff26ccdf7c130d470f; v20 uses that same held revision; v21 reads e3b9e20eebbf5ecd319c372c3d31b1a05a2479d7. Their LSE inputs are frozen objects; leaving these test files untouched correctly preserves history. Seven live Rust consumers need reconciliation, not ten.

Baseline analysis: independently parsed unique failing test identities from rust-baseline-comparison.log and rust-second.log: baseline has 18; candidate second has 20, includes all baseline failures, and adds only version_thirty_binds_parent_static_and_same_map_validation_once_to_existing_invariant and adjacent_contracts_retain_current_owners_until_real_consumer_cutover. Those markup changes are now corrected statically but require the parent's pending actual rerun. V9 runtime libcrypto/libz diagnostic is directly visible in baseline; baseline record names exactly eleven temporarily restored paths and byte-exact candidate restoration. This is a scoped baseline replay, not an independent full historical-checkout execution.

Acceptance judgment: exact inherited failure equivalence may prove the narrow claim that migration preserves existing consumer behavior. It cannot truthfully be called a green Rust run or automatically waive a current required PASS gate. Per common non-deferral requirements, required FAIL prevents complete disposition. Record each failed criterion and whether its actual acceptance is execution/preservation evidence or a required passing gate; do not retrospectively rewrite a required PASS into comparison-only acceptance. Existing expected-red scientific/HOLD postures must remain unchanged.

Verdict: prior parser findings rechecked fixed; consumer structure has direct positive/negative evidence. Overall adoption remains NOT CERTIFIED: B-C17, the one Python diagnostic assertion failure, final Rust rerun/disposition, preservation/usability and independent terminal gates remain outstanding.

## Corrected bridge and legacy-compatibility check

Ran: independently compiled the corrected actual Rust bridge in a temporary harness using nix develop -c rustc --edition=2021, then invoked it on public fixture paths. Spaced, double-quoted, single-quoted and Markdown-fence-hidden format declarations all reject with exit 1. A legacy YAML continuation fixture round-trips byte-for-byte; all 48 existing non-LSE SC entry files also round-trip byte-for-byte through the compiled bridge. This directly verifies consumer compatibility rather than only internal Python parsing.

The former bridge prefix bypass B-C17 is FIXED / independently rechecked. Follow-up quoted-key bypass discovered during the metadata early-return fix was reported and immediately corrected: discovery examines raw front matter and recognizes quoted format-like keys before applying the bounded grammar. No malformed directory fixture above falls back to raw legacy text.

Ran: full Python selection now 82 passed in 3.45 s (same two test module paths, repository cwd). The incorrect missing-definition diagnostic assertion is corrected while retaining exit-1 rejection.

Corrected observed source hashes:
- tools/sc_contract_directory.py: 758ae0fffdee4146d12d36fc91f6970e5d1fae867b78beb1ee02345153760d6b
- tests/integration/support/sc_contract_text.rs: c2b03e16eeba168e0d3fbaf4e2ade4aaf5c68df5b0fba5be8e6c3fe09874b65b

Independent final-log comparison at this observation: rust-candidate-final.log and rust-candidate-corrected.log both report 110 passed / 19 failed, containing all 18 baseline failing identities plus adjacent_contracts_retain_current_owners_until_real_consumer_cutover. The migration introduced a second whole-file section assumption in that same test (INV-043 after the INV-042 loop), reported for correction. A later rerun must demonstrate its resolution before claiming zero added consumer failures; retained failed logs cannot support that claim.

Bounded verdict: PASS for corrected parser/bridge progress and the directly executed Python/legacy/corruption claims above. Adoption remains HOLD, and candidate no-added-Rust-failure claim remains unproven at this observation. Parent has explicitly selected original canonical v31 restoration after exercises, retaining candidate evidence with runnable reconstruction. That is consistent with the frozen recovery requirement; it does not convert required inherited failures to PASS. Final restoration/source identities, preservation/usability and terminal independent verification are separate remaining duties.

## Reviewed consumer cut and exercise 03 assessment

Static: final vegetation consumer correction; full frozen task/rubric, exercise_03 answer/ledger, qualification chapter, and relevant primary EXP-R kickoff/protocol/reproduction/handoff requirements.
Ran: independent log parsing and source-byte/hash reconstruction using repository .venv Python. No actual experiment build/capture or fresh-agent session was executed by this reviewer.

Final Rust evidence now supports the narrow preservation claim: rust-candidate-reviewed.log reports 129 run, 111 passed, 18 failed. Independent parsing found exactly the same 18 failing test identities AND identical normalized panic-reason payloads as rust-baseline-comparison.log. No added/removed/changed failure reason. This verifies more than the generated failure-name JSON. Both INV-042 and INV-043 consumer slices now select canonical invariant sections for directory input; their original guard-map checks and legacy slices remain. The formerly additional vegetation regression is FIXED / independently rechecked against actual logged execution.

Bindings:
- tests/integration/vegetation_boundary_authority_contract.rs SHA256 5ff969756b5e0ddf8500961db525ecddaed6be3a52116fbf75e85419a3792710
- logs/rust-candidate-reviewed.log SHA256 9768b75f0cc76ed4bf11c5d5fded836cb6b2d256231762d8e5691a2de877e55e
- exercise_03.md SHA256 1cfe4550b57e46e36600f9694cd37084d9d0cc52cc4a4b6ad42ca54a5e5ea7c1

Exercise 03 independent answer assessment: PASS for the frozen requirements-only executable-identity task. It identifies a fresh A and R=A+one reviewed delta, never F+replay; exact per-path additions/deletions, base/patch/build inputs, Cargo/Nix/compiler/flags, executable discovery/hash, workload/commands/environment and unchanged repeated series. It separates historical v31 manifest/ceilings from prospective protocol and current canonical reconstruction recipes, retains paused execution and production HOLD, and explicitly refuses physics/closure/replay/performance/promotion conclusions from identity. Qualification -> frozen kickoff/handoff -> protocol/reproduction expansion is justified by actual mandatory capture dependencies. Scientific replay/solver detail is conditionally expanded for scientific claims, not assumed discharged. No missing requirement in rubric task 3 was identified from the recorded answer and primary sources.

Measurement assessment: independently confirmed every current file byte count in the ten-file ledger and all three final candidate hashes. Initial listed bytes sum to 168622; final source inventory sums to 168600. Final LSE subset is 33598 + 47478 + 18905 = 99981 bytes, compared with 264863 bytes read directly from baseline Git object: 62.2518% fewer LSE source bytes. The remaining 68619 instruction/EXP-R bytes are counted. This is sufficient bounded source selection, not evidence of a proportional total runtime-context reduction. Full interface retry requested another 47500 bytes; actual duplicate display volume, search repetition and missing initial hashes prevent a precise observed total exposure claim. The exercise explicitly discloses those limits; initial/final byte equality is not misrepresented as content identity. Freshness is recorded in the exercise artifact; this reviewer does not claim independent runtime/session telemetry.

Verdict: ACCEPTED corrected checker/candidate consumer progress and bounded exercise-03 requirements selection, with prior directly executed conformance/negative evidence retained. The Rust comparison proves no new observed consumer failure, not a green Rust gate. Overall adoption remains HOLD; other exercises, preservation/terminal verification, candidate archive/reconstruction and canonical v31 restoration are outside this bounded signoff and remain required before final package disposition.

## Additional A0 admission consumer review

Static: complete tools/release/check_science_contract_admission.sh and narrow diff, declared pre-edit package expansion, real-Git fixture. The change maps changed directory chapter paths to the validated approved/active parent and adds all declared chapter bytes to worktree authority identity. Original parent approval, registry, A1 and A3 branches are preserved; no frozen impact-map or gate-definition edits observed.

Ran: .venv/bin/python -m pytest -q tests/python/test_sc_contract_directory.py -k admission: 1 passed, 79 deselected. Independently executed the real-Git isolated fixture function and additional mutations. Valid directory chapter change is admitted; additional chapter bytes change authority_sha256; removing an actual obligation while registry/entry mentions remain fails. Changing parent status and matching registry to in_review also fails the original approved/active gate.

Finding B-C18 / medium / OPEN: the worktree authority digest includes the admission script itself but omits its newly imported tools/sc_contract_directory.py execution dependency. Independently appended helper bytes in the isolated fixture; both admission runs passed with identical authority_sha256. Different parsing/admission semantics could therefore share the reported authority identity. Include helper source in the existing authority_paths set and add a focused helper-byte identity regression. Parent notified; no policy/authority registry mutation required.

Bounded disposition: parent mapping and protected gates accepted; helper identity finding pending correction. This review neither certifies a green Rust suite nor changes adoption HOLD/recovery obligations.

Focused B-C18 resolution: FIXED / independently rechecked. The canonical parser helper is now explicitly included in authority_paths. Ran the admission pytest selection again: 1 passed, 79 deselected in 0.30 s. Independently repeated a second helper-byte mutation after fixture setup; both admissions succeeded and authority_sha256 changed. Corrected admission script SHA256 5a44470d75042a04912f7a52b05a72767a49d1c3c67494891e5a530b15ab327d. Narrow A0 consumer reconciliation is ACCEPTED, retaining earlier approval/removed-obligation negatives and unchanged frozen policy gates. No open finding remains from this focused A0 review; overall package HOLD/recovery and terminal verification requirements remain.

## Exercises 02/04 and external candidate recovery

Static: both exercise answers and ledgers; frozen context-usability rubric; every candidate chapter dependency table; relevant root/science/standards instructions; V14 phase-vapor and receiver primitive-enthalpy requirements; complete restore_candidate.py and recovery manifest.
Ran: independent source-range byte calculations; actual external reconstruction and strict/unit checks; refusal tests.

Exercise 02 answer assessment: PASS for represented-snow replay correctness requirements selection. It retains canonical stencil/admissibility/LU/backtracking/accepted-step rules, evaluator source-order and graph completeness, typed integrity versus pre-capability ineligibility versus post-start error distinctions, same-map custody, V10/covered regime boundaries, historical v31 versus EXP-R/PC1/SG1 obligations, and paused status. Excluding frozen-litter/WB14 and exact-owner algorithm reconstruction is bounded by inactive represented-snow ownership and absence of changes to those algorithms; it is not permission to skip changed receiver/constructor/publication logic. The answer still identifies the required real full-evaluator/solve/output/rollback tests. Selected primary chapters cover the frozen task-2 rubric.

Exercise 04 answer assessment: PASS for identifying snow-free frozen-litter accepted-receipt closure requirements. The answer covers V14 signed liquid/ice sensible-plus-latent vapor, separate immutable beginning pools, liquid saturation, bounded freeze/melt/ending capacity and H=U-Lf*Wi; V21 raw/retained/spill subtraction and chronology; V22 native/ordinary matching; V15/V16 high/carry arithmetic after physical basis conversion; topology/support/exact-one owner joins; independent primitive reconstruction and WAT5/p61/native test duties. Its exclusions do not waive any of these. Snow/replay/experiment/whole restart implementation claims would trigger additional authority, but this task makes none.

The L_s(T) observation limits constitutive recomputation, not the stated accepted-receipt ledger task: SC-SURFACELIQUID V2 algorithm step 3 binds separately signed mass times phase-specific enthalpy; actual LitterVaporReceipt contains explicit liquid/ice specific-enthalpy operands, and the closure consumer independently multiplies them. Using authenticated primitive specific enthalpy cannot be represented as independently validating its temperature-to-latent family. The exercise correctly does not invent that family or claim constitutive validation. This review adds no scientific formula or authority adjudication.

Instruction accounting: no demonstrated mandatory instruction omission for these read-only requirements-selection exercises. Exercise 02 counted root/science/standards/work-package instructions and solver standard/ADR. Exercise 04 counted root/science instructions and did not perform package implementation, solver authoring or actual terminal verification. A real implementer/verifier package bootstrap would add its role/package/governance reading; these observations must not be relabeled as complete production-workflow bootstrap savings.

Independent arithmetic:
- Exercise 02: 21 full source files, 1196953 bytes. LSE subset 291226 versus baseline 264863, increase 26363 bytes / 9.9534%. Named final obligation rereads sum exactly 26146 bytes.
- Exercise 04: 17 full source files, 1217616 bytes, plus separately reported 880 search-exposed source bytes. LSE subset 298442, increase 33579 bytes / 12.6779%. No reduction for either physical/solver task.
- Finding B-M01 / medium / OPEN: exercise04 claims repeated requested ranges total 191997 bytes. Independently summing its explicitly listed inclusive ranges from final UTF-8 files, preserving overlap/multiple requests, gives 202918 bytes. Parent asked to reconcile requested versus actually repeated ranges and any source drift; do not silently infer exact delivered exposure. Full-source totals above are unaffected.

Recovery validation: executed .venv/bin/python artifacts/restore_candidate.py <new TemporaryDirectory child>. It restored 29 overlay + 11 baseline-dependency files; independently verified all 40 lengths/hashes against candidate-recovery.json. From that external tree, actual strict checker PASS (17 BEI/100 definitions) and unit checker PASS. Reusing the existing output rejected exit 2 with original bytes unchanged; attempting a new checkout-internal output rejected exit 2 without creating it. Script preloads/verifies source identities before directory creation, confines manifest paths, and refuses existing/repository destinations. The .git directory is only a parser confinement marker. The artifact explicitly does not construct a full Cargo workspace; no Rust/A0 execution from this minimal tree is claimed.

Bounded verdict: accepted requirements-selection evidence and safe reproducible structural/unit candidate archive. Physical/solver task reduction acceptance is NOT met; repeated-range metric B-M01 remains to reconcile. Overall adoption HOLD and canonical v31 restoration/terminal verification remain unchanged.
