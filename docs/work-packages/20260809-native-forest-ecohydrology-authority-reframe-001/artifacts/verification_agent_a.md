# Terminal Verification A

Evidence class: `Ran + Static exact-current-diff review`

Verdict: `PASS — EXECUTED-HOLD IS THE CORRECT TERMINAL DISPOSITION`

Verifier A did not inspect Verifier B's output.

## Findings

### `VERIFY-A-HIGH-001` — closed

The kickoff bytes are correctly archived at
`prompts/completed/20260809-native-forest-ecohydrology-authority-reframe-001_kickoff_agent_prompt.md`,
and their reproduced SHA-256 is
`6d435d7f9e63ebf81559bf3d16ea03ff2981eeca528d844fd5ee487ee0a62b5d`.
At initial verification, three package-local pointers still described an active
prompt or an archive that occurred only after passing all exit criteria:

- `README.md` says the kickoff is under `prompts/active/`;
- `prompts/active/README.md` says to execute a kickoff in that directory and
  archive it only after all exit criteria pass; and
- `prompts/completed/README.md` says only terminally verified prompts are
  archived there.

Post-fix inspection confirms all three pointers now state that no executable
prompt remains and that terminal disposition includes truthful executed holds.
The archived kickoff and digest are unchanged. Finding closed.

### `VERIFY-A-MEDIUM-002` — closed

The first correction recorded 249 lines. Final lifecycle recording then added
four bounded disposition lines; `artifacts/line-count-governance.md` now records
`package.md` as 253 lines, and `wc -l` independently confirms 253. The threshold
disposition remains correct. Finding closed.

## Verified Results

- `SC-VEGETATION-001` version 4, its index row, and the focused source-level
  contract test consistently encode caller-owned site values/state, bounded
  `ASSUMED_FOR_EXECUTION` demonstrations, independent native-forest component
  fluxes, layer-resolved roots, rejection of agricultural `Kcb`/LAI demand
  redistribution, and the narrow Penman-Monteith posture.
- The held successor package and active successor prompt consume the reframe,
  retain the historical closure package only as source/gap evidence, and remain
  held on complete schema/constitutive authority and their own contract-first
  gates. They do not claim production release.
- All review findings are accepted and visibly corrected. The calibration
  readiness matrix has the required orthogonal statuses and ten obligation
  dispositions.
- The changed tracked paths are within the declared write set. No production
  Rust, Cargo, assurance test/implementation, runtime consumer, output,
  selector, or deployment path changed.
- The initial full-workspace evidence was invalidated by disk exhaustion. The
  recorded recoverable remediation is corroborated by the two ignored
  `target/task-tmp/stale-openwepp-gate-*` trees. The remediated full run still
  failed in the assurance publication suite, and the exact isolated test fails
  again below. Because the vegetation diff does not touch that test or its
  implementation and assurance repair is outside the declared write set, the
  package must close `EXECUTED-HOLD`, not `complete`.
- The full-workspace gate is not deferred or reported as passing. Its lift
  condition—repair or authoritatively disposition the assurance failure, then
  rerun the exact full profile on this diff—is precise.
- Contract/test/package line counts are below the 2000/3000-line governance
  thresholds; root `AGENTS.md` remains 153 lines.

## Replayed Commands

Ran from `/home/workdir/openWEPP` against the exact current worktree:

```text
markdown-doc lint --path docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001 --format plain
PASS: 34 files, 0 errors, 0 warnings

markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md --format plain
PASS: 1 file, 0 errors, 0 warnings

TMPDIR=/home/workdir/openWEPP/target/task-tmp cargo nextest run --test vegetation_boundary_authority_contract --profile quick
PASS: 10 passed, 0 skipped

cargo fmt --all -- --check
PASS: no output

git diff --check
PASS: no output

TMPDIR=/home/workdir/openWEPP/target/task-tmp cargo nextest run --test assurance_v2_publication_contract draft_subject_root_is_stable_but_cannot_publish --profile quick
FAIL as recorded: 1 ran, 0 passed, exit 100; assertion at tests/integration/assurance_v2_publication_contract.rs:541 expected an Invalid error containing DRAFT

sha256sum prompts/completed/20260809-native-forest-ecohydrology-authority-reframe-001_kickoff_agent_prompt.md
PASS: 6d435d7f9e63ebf81559bf3d16ea03ff2981eeca528d844fd5ee487ee0a62b5d

wc -l AGENTS.md docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md tests/integration/vegetation_boundary_authority_contract.rs package.md
153, 573, 502, 253 lines respectively after final lifecycle recording
```

Post-fix replay:

```text
markdown-doc lint --path docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001 --format plain
PASS: 34 files, 0 errors, 0 warnings

git diff --check
PASS: no output

wc -l docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001/package.md
PASS: 253 lines
```

Both findings are closed. The documentation-only fixes do not invalidate the
focused contract test or the evidence establishing the unrelated exact-head
full-workspace blocker. Verifier A accepts the terminal `EXECUTED-HOLD`
disposition with no residual finding.
