# ASSURE-03 Final Closure Delta Audit

Evidence class: `Ran`

Disposition: `PASS`

## Scope And Boundary

This is the required closure-only bounded delta audit. It did not rerun a full
aggregate and did not edit source or governance files. It compares the current
non-artifact tree with the renewed r4 terminal identity and determines whether
post-r4 changes are strictly closure governance and artifact metadata.

This audit does not extend the r4 claim. R4 remains transition-route
verification with stability skipped; it is not a conformant candidate or
release-qualification result.

## Retained R4 Authority

The retained comparison records are:

- `/tmp/assure03-va-r4-status`
- `/tmp/assure03-va-r4-diff`
- `/tmp/assure03-va-r4-paths`
- `/tmp/assure03-va-r4-manifest`

Their identities exactly reproduce the renewed heavy-runner record:

- status: 67 rows, 10,135 bytes, SHA-256
  `38c55a522f7464ec6cacb93411687e40118248fa341d366b6d335d930b02e4f0`
- full-index binary diff: 439,133 bytes, SHA-256
  `d60c66de0a040fd1a241773c336144fd26698a655014074ee0efbdc82ff77a49`
- present-file paths: 40 rows, SHA-256
  `ddaeb9d0beeef73ff53782e68292a4db127ccda2d76e6c37f2cd9c86922b202a`
- ordered present-file content manifest: SHA-256
  `a5355bf907b0e23efae776ba3c464404e21e6c2f669d4ff1a39a00008c6248b8`

The repository `HEAD` remains
`3352388465f8b288aed4636e8f9752ca6c1cceb9`.

## Selection And Current Identity

The audit reproduced the r4 selection and encoding:

```bash
git status --porcelain=v2 --untracked-files=all |
  rg -v ' docs/work-packages/[^ ]+/artifacts/'
```

It extracted the last status field, retained only current regular files,
sorted paths with `LC_ALL=C sort -u`, and emitted each manifest row using
`sha256sum "$path"`. Artifact paths were excluded before path extraction.

Current identity:

- status: 67 rows, 10,137 bytes, SHA-256
  `f59a166d8ff3f149e5a6d5fea490880617dd146b6ab3460df4e037cd8038780c`
- full-index binary diff: 439,729 bytes, SHA-256
  `56e1dbed4dccd716527b098abd758bd29398d877dc187961b9a848b66d3339a3`
- present-file paths: 40 rows, SHA-256
  `13cf0251d09eecf114e7346e50eac55fe6b826883254cc89ff6ed847903bc47a`
- ordered present-file content manifest: SHA-256
  `1178d3b69e83a4e612bedb94f038dce0dd7d18074c251bcb27775d870d407bd7`

The r4 and current path sets have 39 common paths, one removed path, and one
added path. Among the 39 common rows, 31 content hashes are identical and eight
are changed. Thus the complete manifest delta is exactly ten rows.

## Complete Manifest Delta

Changed common paths:

- `docs/ROADMAP.md`: r4
  `1f198557d52f3ff9558b17a0eed27125093849a2c225b8178fbb322e7ec34724`;
  current
  `aff4ac34663440f9bcfedd0661de360b47b9ea662c53e0d25917b9f308a893e8`
- `docs/planning/scientific-assurance-v2-implementation-roadmap.md`: r4
  `c7b57afac979d86edf98dc65df2d61de77a0458eb319284479b6658d2810afb1`;
  current
  `e8f4d00f4ac7e9dd2855de244761425496c41525d8cb679f4002024d116bd874`
- `docs/planning/scientific-assurance-v2-migration-plan.md`: r4
  `43ee49628514f86b5c9e37441317e158f6bf7655f7fcf03a2a2726adab8f53b7`;
  current
  `87e0377a6aa3dffb326c3a21498f8eaadd34108790b246662cbdb700454ac899`
- `docs/work-packages/README.md`: r4
  `5798b6c62e1af30aacc6fb37f32c080fbd0b32387f7144cbe69943b84ea1be43`;
  current
  `b1b82ea21d4fcb1534cb4d36944011f10e6b0c873088e7a20e2cbe8b117b727a`
- `docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/package.md`:
  r4 `445101744025b86085d39c7032919f58395feeb9e23b30ccaca133c4201522cc`;
  current
  `7d2f052cc5788cfa3aaf7c12c2991ea000cb16fc25b3349da4c21df63a548138`
- `docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/prompts/README.md`:
  r4 `da2585a38b6ce6b292eb7661545ab855c6ed22650a3db0dbc564fd7f9fc1d0fb`;
  current
  `7a08f09a9a24bcd6fc9a7d9221b895a5f89462132aa0ea7f50f4fe7f537ab8a7`
- `docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/prompts/active/README.md`:
  r4 `8189b76c9608cfa35667c58580499dda97a50bbbd71c9a07da92bd1a025d21a0`;
  current
  `edddb1eea4c6db37c86b8f2acd0cdc8e4eebb1c726e0141033531f720508d1ff`
- `docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/prompts/archived/README.md`:
  r4 `a3f3f8630556beac88abfc6a8d798858fd6640c1bde2e896b313c7d34e3de562`;
  current
  `6b6ef70e77f97b35de551f5475abe9dfbaa655851eac280e07979aea8706f373`

Removed path:

- `docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/prompts/active/20260714-codex-execute-assure03_prompt.md`:
  r4 `deed63535f4cac9d3cdb1c696e1b91b10274dfb16e5e18105eddafcc149dad4f`

Added path:

- `docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/prompts/archived/20260714-codex-execute-assure03_prompt.md`:
  current
  `ef0a598b228d04f7d5fac2ca9d419831533329adbf43d488de3d77cacf2700e6`

The porcelain-status delta contains only removal of the untracked active-prompt
row and addition of the untracked archived-prompt row. No other status
classification or path changed.

## Closure-Only Classification

Direct inspection classifies all ten rows as closure governance:

- `docs/ROADMAP.md` makes `ASSURE-04A` the next prospective queue item after
  ASSURE-03 closure.
- The migration plan records completion by ASSURE-03.
- The implementation roadmap records ASSURE-03 as the completed foundation and
  requires a new operator instruction before ASSURE-04A.
- The work-package catalog records ASSURE-03 as `EXECUTED-COMPLETE` and retains
  the stability/release-qualification exclusions.
- `package.md` changes status to `EXECUTED-COMPLETE`, closes progress and
  retrospective records, archives the prompt, and records the bounded
  ASSURE-04A handoff.
- The three prompt READMEs state that execution is complete, no ASSURE-03
  prompt remains active, and the completed kickoff is archival evidence.
- The kickoff prompt moved from `active/` to `archived/` and its measured Core
  reading budget changed from `86,652` to `88,903` bytes.

The prompt move and budget change were independently isolated in memory.
Replacing the archived prompt's exact line

```text
Required-reading budget: 88,903 local bytes for Core, `OK`; map:
```

with the r4 line containing `86,652` produces SHA-256
`deed63535f4cac9d3cdb1c696e1b91b10274dfb16e5e18105eddafcc149dad4f`,
exactly the r4 active-prompt hash. Therefore the prompt delta is only the path
move plus that one reading-budget line.

No changed path is compiled source, a test, workflow, release script, assurance
source/generated output, public/science usersum content, fixture, or CRAP
authority.

## Protected Surface Equality

The status delta is only the prompt move. The retained and current manifests
also give these exact protected matches.

Compiled Rust and crate input, 6 of 6 match:

- `crates/openwepp-assurance/Cargo.toml`:
  `3cf85ca60c37a7e743a48d66113c36b5a85fe75d244b4e6dea95ae056b623df9`
- `crates/openwepp-assurance/src/cli.rs`:
  `251413b67a6268a9c3d882e6639cecb02040336e23c8116727d35294874acab8`
- `crates/openwepp-assurance/src/engine.rs`:
  `6bbc146c18fc03979b7bad69f767258ba30a1012f96693f16307da14f60abbf9`
- `crates/openwepp-assurance/src/error.rs`:
  `213eecb19a842c30a53b29fcc74ee44235ed15f79ec8bdad263b743f2fa9fd62`
- `crates/openwepp-assurance/src/hash.rs`:
  `2166e1eec7e445635bfc8ee3ba82e3e47a98e7b5cfac3956417f308adbc3fd80`
- `crates/openwepp-assurance/src/lib.rs`:
  `7e0f19d8a0c0bb7be56dbcff3755fd0950bd26ccc3be75387e0f4e0216a6e1f1`

Test, workflow, and release scripts, 5 of 5 match:

- `tests/integration/assurance_dossier_build_contract.rs`:
  `79c0b36631b6a70d69c7666cc4f95d41a2620fdffdf668d903111bcdecfba230`
- `.github/workflows/release-gates.yml`:
  `ebc17a4566e9adce709deb9aecdd82419bc050f7662522fe8f8c4936419c8a04`
- `tools/release/check_assurance_dossier_exports.sh`:
  `f9167676b0bb42d14916403dde59dde8d518e7de4123e80094330ffc450220cf`
- `tools/release/check_assurance_release_transition.sh`:
  `6da0016571b8cccadf4a0e32a7d32f8bd2b257ddf06188e46dae6f268bad479f`
- `tools/release/run_release_candidate_gates.sh`:
  `edffa4cf872ee3f972b50104901b2a334b6addf2093465fc19d58eaad2e9ad64`

Assurance source/generated surfaces, 4 of 4 match:

- `assurance/README.md`:
  `012dd7fb37e68ef145de1e2bc8863b9de4a0c31f7f71ec8f448946b8caf1226d`
- `assurance/catalog.yaml`:
  `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f`
- `assurance/generated/wepppy-usersum.yaml`:
  `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb`
- `assurance/templates/catalog.md`:
  `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70`

Public and science usersum surfaces, 3 of 3 match:

- `usersum/README.md`:
  `d6d2bcd1675acaabea659057c6d84257d1cec6a069d51e42b63420baa84eae89`
- `usersum/assurance/README.md`:
  `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70`
- `usersum/snow-frost-modeling-and-validation.md`:
  `6c861e573d0b087c1b49cea90a2ce2b62511d4bbfdbae000a38e30763f2588fe`

The retired filesystem shells and retired public route remain absent:
`assurance/dossiers`, `assurance/methods`, `assurance/schemas`, and
`usersum/assurance/snow-snotel-swe-depth-density.md`.

## Retained CRAP And Heavy Evidence

The retained validation-evidence tree remains byte-identical with r4:

- files: 19
- bytes: 8,201,956
- path-sorted per-file manifest SHA-256:
  `cc2b394362b03b2f78e68e1b2681e220d3fffa8544c79adc3228ed6b3d3019cd`

All 16 entries in its adjudicated-CRAP checksum manifest verify. Selected exact
identities remain:

- CRAP report:
  `01dd7f1f9d9d54f6a10f6b2844aaddd9b89bb5a6e42fe266aa362bcdc6f4b291`
- run status:
  `8930517bf400ed347b54f18171081f85fa6ef51173dce55c7ce6548bcee52842`
- LCOV:
  `4a4dad862b50d3de3bfa6dd748ff5818696ced7898b23f4cc71f1cb8aa6b18a1`
- raw CRAP JSON:
  `f093a86c129415309fefd99d41d25998a372ac620df185833a80174b29da3fe5`
- adjudication registry:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`
- authority results:
  `dd989b1d0067886d1ded66bb8048d7ab6c9cde1e0e5d2677ce8dc4543ef1aa56`
- cargo-nextest version:
  `b94f9fca6aa62c8d95f088fbde71d75f1aa2796bf4ab5715320def301eb08f85`

## Disposition

`PASS`: the complete post-r4 non-artifact delta is strictly closure governance
and prompt/archive metadata. Protected implementation, tests, workflow,
release scripts, assurance source/generated outputs, public/science usersum
surfaces, retired-route absence, and retained CRAP evidence are byte-identical
with r4.

The r4 heavy-gate results therefore remain applicable without a full aggregate
rerun. This conclusion does not add stability evidence or extend r4 into a
candidate, retained release, or release-qualification claim.

Post-write checks: scoped `markdown-doc lint` passed with zero errors and zero
warnings; `git diff --check` passed; `uk2us` preview of this audit artifact
produced no difference.
