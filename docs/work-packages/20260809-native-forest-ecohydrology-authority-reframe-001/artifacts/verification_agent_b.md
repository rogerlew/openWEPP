# Independent Terminal Verification B

Evidence class: `Ran + Static exact-current-tree verification`

Verdict: `PASS FOR EXECUTED-HOLD`

Reviewer independence: Verifier A's artifact was not read before this verdict.

## Verified Outcome

The claimed `EXECUTED-HOLD / EXACT-HEAD FULL-WORKSPACE BLOCKER` is truthful.
The focused vegetation contract suite passes, but the required Critical full
workspace gate does not pass. The retained rerun log records both a timeout and
the assurance-publication assertion failure; it does not misclassify the full
run as successful. Independent isolated replay reproduced the stable assertion
failure at
`tests/integration/assurance_v2_publication_contract.rs:541`:

```text
draft_subject_root_is_stable_but_cannot_publish: FAIL
1 test run: 0 passed, 1 failed, 36 skipped; exit 100
```

The exact lift condition is appropriately narrow: repair or authoritatively
disposition that existing assurance contract in separately authorized scope,
make the isolated test pass, and then rerun
`cargo nextest run --workspace --profile full` on the exact vegetation diff.
Neither this package nor the coupled-vegetation successor may claim completion
before that exact full run passes.

Static inspection also verified:

- `SC-VEGETATION-001` version 4 and its index/test agree on caller-owned site
  configuration/state, bounded `ASSUMED_FOR_EXECUTION` demonstrations,
  independent native-forest flux components, the agricultural PMET poison
  rule, layer-resolved roots, and the limited Penman-Monteith posture;
- all science-review findings are accepted and reflected in the current
  contract, readiness matrix, and successor prompt;
- the readiness matrix reports the required orthogonal
  `AUTHORITY_MISSING`/`NOT_CALIBRATION_READY`/`NOT_ASSESSED` posture and all ten
  obligation dispositions without releasing the implementation successor;
- no production Rust, Cargo, assurance identity/test, runtime consumer,
  selector, output, or deployment path changed;
- the tracked terminal diff is confined to the declared contract, focused
  contract-test, successor, and lifecycle surfaces. The separately untracked
  preceding vegetation-closure package is disclosed in pre-implementation
  intent as inherited dirty state, not work claimed by this reframe;
- disk remediation was recoverable: scratch data remains under ignored
  `target/task-tmp`, root has approximately 6.8 GiB free, and no tracked file
  was altered by remediation;
- governed line counts remain below mandatory refactor thresholds; and
- the completed kickoff SHA-256 is
  `6d435d7f9e63ebf81559bf3d16ea03ff2981eeca528d844fd5ee487ee0a62b5d`.

## Finding

### `VERIFY-B-MEDIUM-001` — `accepted / closed`

The initial verification found that the kickoff had already moved to
`prompts/completed/` while four package/prompt lifecycle READMEs still described
it as active or archivable only after verification.

Post-fix inspection verifies that package `README.md`, `prompts/README.md`,
`prompts/active/README.md`, and `prompts/completed/README.md` now agree: no
executable prompt remains, and the byte-preserved kickoff is archived for the
terminal executed-hold disposition. The finding is closed without changing the
scientific amendment or the full-workspace hold.

No residual finding remains. The package may terminally disposition
`EXECUTED-HOLD`; this verifier does not claim that the Critical full-workspace
gate passed.

## Replayed Commands

Ran from `/home/workdir/openWEPP`:

```text
markdown-doc lint --path docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001 --format plain
  PASS: 34 files, 0 errors, 0 warnings
markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md --format plain
  PASS: 1 file, 0 errors, 0 warnings
markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001 --format plain
  PASS: 34 files, 0 errors, 0 warnings
markdown-doc lint --path docs/ROADMAP.md --format plain
markdown-doc lint --path docs/backlog/TRACKER.md --format plain
markdown-doc lint --path docs/backlog/20260806-rhessys-derived-vegetation-crate.md --format plain
markdown-doc lint --path docs/work-packages/README.md --format plain
  PASS: each 1 file, 0 errors, 0 warnings
git diff --check
  PASS
cargo fmt --all -- --check
  PASS
TMPDIR=/home/workdir/openWEPP/target/task-tmp cargo nextest run --test vegetation_boundary_authority_contract --profile quick
  PASS: 10 tests run, 10 passed, 0 skipped
TMPDIR=/home/workdir/openWEPP/target/task-tmp cargo nextest run --test assurance_v2_publication_contract draft_subject_root_is_stable_but_cannot_publish --profile quick
  FAIL as claimed: 1 run, 0 passed, exit 100
sha256sum prompts/completed/20260809-native-forest-ecohydrology-authority-reframe-001_kickoff_agent_prompt.md
  PASS: 6d435d7f9e63ebf81559bf3d16ea03ff2981eeca528d844fd5ee487ee0a62b5d
```

The Critical full-workspace command was not rerun by this verifier; its retained
failure evidence and independently reproduced isolated blocker are sufficient
to verify the claimed hold, not to claim the full gate passed.

Post-fix replay:

```text
wc -l docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001/package.md
  PASS: 253, matching line-count-governance.md after final dual-verifier lifecycle recording
markdown-doc lint --path docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001 --format plain
  PASS: 34 files, 0 errors, 0 warnings
git diff --check
  PASS
sha256sum prompts/completed/20260809-native-forest-ecohydrology-authority-reframe-001_kickoff_agent_prompt.md
  PASS: 6d435d7f9e63ebf81559bf3d16ea03ff2981eeca528d844fd5ee487ee0a62b5d
```
