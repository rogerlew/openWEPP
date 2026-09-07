# Independent terminal verification B

Static: root/package/role-verification instructions; testing-and-gate-strategy sections 7, 9–10 and 18, correctness authority and applicable science obligations; full directory-format v1; corrected parser/CLI, unit and A0 diffs; archived Rust bridge; recovery implementation/manifests; reviewer findings/dispositions; primary source/map, context reports and raw Rust/Clippy logs. Ran: independent checks below. Verifier A's artifact was not read.

Role/session: `/root/verify_b`, independent bounded command/reconstruction verifier. Interpretive effort requested medium by assignment; effective runtime settings UNOBSERVED. Repository writes limited to this artifact; adversarial fixtures and bridge probe were isolated under `/tmp`.

Reviewed identity: baseline `b932db101cce07d0860b45b5ecaa8ddb7f455b58`; corrected substantive HEAD `3ec62445368a4c72bd242c6c98fb1d3a77cf6dd7`, plus the inspected narrow format-status working delta. Format document SHA256 `4b7aa41200faaac647c8d8624597779a13c50bee444c0ca738ff7fd29653df1c`. The later reviewer-A appendix independently addresses that same status delta, without changing acceptance. Final publication packaging remains the parent's responsibility; any substantive source/claim change reopens affected verification.

## Independent execution

All repository commands ran from `/workdir/openWEPP`. Python was `.venv/bin/python`. `A` below denotes this package's `artifacts` directory; `F` is `/tmp/lse-verify-b-pl6zbie0/candidate`; `E` is `F/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`. These are abbreviations in this record, not hidden environment dependencies.

| Check / exact command or reproducible operation | Observed result |
|---|---|
| `.venv/bin/python -m pytest -q -p no:cacheprovider tests/python/test_sc_contract_directory.py tests/python/test_check_sc_binding_exposure.py` | Exit 0; **83 passed in 3.57 s**. |
| `.venv/bin/python A/restore_candidate.py F` | Exit 0; 40 verified files. Independently recomputed every output byte length and SHA256 against `candidate-recovery.json`. No Cargo.toml: this is the documented structural/unit fixture, not full executable reproduction. |
| `.venv/bin/python F/tools/check_sc_binding_exposure.py --strict E` | Exit 0; directory-v1, 17 BEI rows, 100 actual definitions. |
| `.venv/bin/python F/tools/release/check_sc_unit_compliance.py --path E` | Exit 0; no findings on original reconstructed candidate. |
| Recovery CLI to existing F, new `/workdir/openWEPP/verifier-b-prohibited-output`, new child through a symlink to repository, and dangling output symlink | Each exit 2, before output creation or overwrite. |
| Isolated copy of recovery script/manifests/archive with overlay SHA256 replaced by 64 zeroes; then independently `../escape.md` and `/tmp/escape.md` manifest paths | Each exit 2; respectively identity mismatch / unsafe manifest path. No output directory created. All actual archive/source bytes untouched. |
| Delete the sole marked `INV-LANDSURFACEENERGY-164` row from reconstructed dependency-replay.md while retaining entry alias, binding registry and BEI mentions; run candidate strict CLI and shared-loader CLI | Both exit 1 with missing explicit anchor. Restoring the row restores valid input. Mentions cannot supply the missing definition. |
| Change real interface.md `T_s` Variables and Units cell from `K` to `TBD`; run strict and unit CLIs | Structural exit 0; unit exit 1, `SCUNIT-E-003`, actual chapter line 79. Structure is not unit/science equivalence. Restored original bytes afterward. |
| `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | Exit 0; **legacy 16 BEI rows**, explicitly not candidate adoption. |
| `.venv/bin/python tools/release/check_sc_unit_compliance.py --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | Exit 0. |
| `PATH="$PWD/.venv/bin:$PATH" bash tools/release/check_science_contract_admission.sh --base-ref b932db101cce07d0860b45b5ecaa8ddb7f455b58 --worktree` | Exit 0; A0_ADMITTED contracts=49, science_surfaces=0, authority SHA256 `1d176d1694d0d79eeb22d233f25557b0372d6a11a9aa4436f3a28d542ea2f0ea`. |

The independently executed pytest suite includes mandatory removed-definition-with-mentions, qualified obligation deletion through whole-set consumer, registry-backed alias-map deletion (`SCUNIT-E-011`), all-chapter units, strict/default deferred behavior, legacy continuations, malformed/quoted/fence-hidden dispatch, nonregular/unreadable/invalid-UTF8 files, escaped dependencies, symlink sentinel preservation, history/provenance/alias/coverage negatives, and A0 parent/chapter/helper digest mutation. Inspected assertions demand expected rejection and diagnostics; no bypass or weakened expected PASS was introduced.

For an additional actual Rust consumer check, compiled the unchanged archived bridge with a temporary parent module whose success assertion requires real INV-164 statement text:

`CARGO_MANIFEST_DIR=/workdir/openWEPP nix develop -c rustc --edition=2021 /tmp/lse-verify-b-pl6zbie0/bridge_probe.rs -o /tmp/lse-verify-b-pl6zbie0/bridge_probe`

Compile exit 0. `/tmp/lse-verify-b-pl6zbie0/bridge_probe E` exits 0 on original candidate, 1 after removing the actual INV-164 definition while mentions remain, 1 after changing `contract_format:` to malformed `contract_format :`, and 0 after restoration. The probe uses the actual archived `tests/integration/support/sc_contract_text.rs` and canonical parser, not an alternate loader. This is bounded bridge behavior, not a new full Cargo/nextest gate run.

Exploratory checks are retained honestly: my first real-definition probe used the shortened nonexistent ID `INV-LSE-164` and failed its setup assertion before mutation; corrected full-ID execution is above. An H_hi boundary alias substitution remained unit PASS under the existing registry/token rules. It is not evidence that arbitrary alias substitutions are rejected. The independent registry-backed deletion test and real placeholder-unit corruption provide the narrower claimed evidence. Initial log-parser delimiter selection also produced zero payloads and failed my assertion; corrected parsing independently compared all 18 payloads below. None of these harness mistakes was labeled product PASS.

## Identity, exact terminal diff and retained failures

Ran independent Python hashing/Git-object comparisons, all assertions successful:

- All 10 source-manifest entries match the pinned baseline Git bytes. Canonical LSE is restored at 264,863 bytes, SHA256 `7a002bcac2ad640716b52f4bd59326f241d5fd3084d26e56f6a703eb496ed61d`; no canonical chapter directory or bridge adoption remains.
- Every candidate contract/checker/consumer manifest member matches archive bytes; all 12 external dependencies match live unchanged files. All 40 recovery outputs match their manifest. The 15-file candidate contract set totals 410,021 bytes.
- All seven original Rust consumers equal baseline Git bytes. Six archived checker/Python-test implementation files equal current canonical counterparts. There is no terminal live production, Rust, Cargo, science-contract, frozen-runtime or historical-package difference from baseline.
- `git diff --name-only b932db101cce07d0860b45b5ecaa8ddb7f455b58` outside this package contains only format-status wording, two package catalog locators, the new Python tests/shared parser and binding/unit/A0 tools. Initial unrelated untracked `$pkg/` and `tmp/` remain untouched.
- The status wording preserves grammar, legacy qualification limits and all adoption gates; it truthfully distinguishes implemented structural checker from executed HOLD. It is accepted as a bounded status correction.

Detached manifests checked: source-manifest SHA256 `fee931d1cde1bba14cfd5087b5061b1dafbbab05709606a96ed28bdbe544f42f`; candidate-manifest `6883f7bc631d1a6c4173af84bd208bafe5c39718fd79f7820f1cf35eab474321`; recovery `9ca04d0addb60803d5cf12a47f88efe71fdf636753d522b17e0e4af784eda131`.

Ran independent parsing of primary `logs/rust-baseline-comparison.log`, `rust-candidate-reviewed.log`, and `rust-restored-final.log`: each says 129 tests, 111 passed, 18 failed, 0 skipped. Extracted the post-Summary failure blocks; normalized only process IDs and source line/column locations in panic headers. **All 18 test identities and complete remaining panic payloads are identical across all three logs.** This is stronger than ratifying the failure-set JSON. The raw required Rust gate remains FAIL (recorded exit 100); it is not PASS because baseline matches. `clippy-candidate.log` ends with failure to compile the unchanged production dependency due to 1136 previous errors; recorded exit 101 remains FAIL. This verifier did not independently rerun those heavy suites or Clippy and does not claim otherwise. Their actual commands and initial/intermediate failures remain in gate-results and logs. The corrected Python/bridge behaviors were independently executed above.

## Preservation and context claim checks

Ran source-map checks against actual restored baseline and archived chapters: coverage includes every source line 26–3346; all 36 exact fenced bodies survive verbatim. Independently compared mapped target lines: 2654 exact line matches plus 26 encoded/header transformations. Inspected samples at original purpose/provenance/unit rows, Change Log boundaries 1894–1918, dependency-replay restrictions and final production HOLD. Complete scientific-equivalence review is not inferred from these counts; reviewer preservation evidence remains separately bounded, and this verifier does not relabel its 100 Statements/74 Evidence/474 rows/275 paragraphs as a newly executed independent full science audit.

Ran independent reconstruction of every before/after context-report line-range byte count and full-file SHA256 from baseline Git and immutable archived candidate; recomputed phase exposure sums and combined per-path line unions. External/instruction selections, ranges and hashes are identical between before and after for each task.

| Task | Baseline total | Final selected total | Final LSE bytes / change from 264863 |
|---|---:|---:|---:|
|01|2438218|2494374|321019 / +21.2019%|
|02|1170590|1200346|294619 / +11.2345%|
|03|333482|168600|99981 / −62.2518%|
|04|1184037|1217616|298442 / +12.6779%|

These are reproducible declared source inventories, not observed baseline sessions, delivered repetition, tokens, quota or workflow-runtime measurement. The task-04 additional 880 search bytes are excluded separately on both sides; recovery request versus duplicate portion remains separately disclosed. Initial incomplete/failed exercises and carried-context corrections are retained, not rewritten into four fresh first-pass PASS sessions. Only identity selection demonstrates reduced LSE bytes. Applicable dependency expansion and mandatory whole-contract authority are not waived by this measurement. Corrected answer sufficiency is a requirements-selection review, not physical closure or production proof.

## Findings, limits and verdict

No new blocking defect found in assigned corrected command/path/recovery behavior. Accepted fixes B-C01–18 and consumer dispatch/definition/unit checks have primary-source or directly executed evidence as detailed above; unchanged review portions remain bound to the verified archive. Minor publication note about required-reading-map's stale measurement-pending sentence is CLOSED: independently inspected its correction. Also inspected final conformance/context historical-status clarifications, gate-results command spacing/status explanation, and earlier-custody-bundle distinction. Verified the stated earlier manifest SHA256 `4ebc1be192c9cb8631424bf7d340fe5c304f089f5f08323c22209f69d0a26b1c`; it is explicitly not the corrected candidate. These packaging deltas change no source, acceptance or numeric result.

**PASS for the bounded terminal command/recovery/identity verification and acceptance of truthful executed-HOLD disposition only. Adoption remains BLOCKED by required FAILs.** Explicitly checked the gate non-deferral rule: no current required Rust/Clippy gate is waived, moved to a later increment, or replaced by failure-set equality. The missing v31 production seam, frozen V9 runtime identity and unrelated old authority failures cannot be supplied by permitted documentation/loader changes; production implementation, frozen-identity mutation or test weakening would exceed authorization. The authorized fallback retains original v31 and whole-contract reading. This verification authorizes neither canonical candidate publication nor selective-reading activation, experiment execution, physics acceptance or remote push.

Parent may finalize truthful publication records after both terminal verifiers finish. No new source/candidate work is requested by this verdict; any adoption claim requires the unresolved gates to pass under appropriate authorization. Effective session context, baseline-agent telemetry and actual heavy-suite execution by this verifier remain UNOBSERVED / NOT RUN, respectively.
