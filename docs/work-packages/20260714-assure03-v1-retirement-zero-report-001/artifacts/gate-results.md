# ASSURE-03 Gate Results

Status: PASS — renewed terminal heavy gates and dual verification complete

Evidence class: Static + Ran

The renewed r4 post-`VB-001` implementation freeze passed both required
aggregates and remained byte-identical throughout. The older r3 aggregate and
its bounded metadata audit are historical chronology only. Earlier failed or
interrupted attempts remain visible in `heavy-gate-runner.md` and carry no
terminal claim.

## Package-Gate Disposition

| Package gate | Current disposition | Evidence |
| --- | --- | --- |
| Exact v1 reconstruction | PASS | All 51 frozen blobs recover with exact size/hash; actions are bidirectionally checked. Focused run `76c752cb-aff2-4135-86e1-fe3d439aba05`. |
| Atomic v2 acceptance / v1 retirement | PASS | ADR-0038 accepted; v2 documents active; v1 standard retired; ASSURE-02 acceptance record complete. |
| Ordinary CI validation only | PASS | Workflow uses `--mode validate` and only validation-named upload on ordinary events; contract test passes. |
| Explicit release preflight and zero-report route | PASS | Release-mode transition aggregate exited 0 after preflight, zero-report snapshot, binary/sidecar assembly, and release lint. Negative tests reject marker, nonempty catalog, retired file, and marker/catalog/retired symlinks before release-directory creation. |
| Real zero-report CLI/build/check | PASS | Focused run `3143e492-8993-4a68-a8da-119765236e6f` passed 13/13. Both renewed aggregates then exercised the exact-catalog admission and real zero-report route successfully. |
| One public file / zero export / no vendoring | PASS | Public tree and generated export checked by focused test and source inspection. |
| Snow/frost science preservation | PASS | Review A independently matched retained identities, counts, residual, narrative, findings, and limitations. |
| Zero-report immutable/confined snapshot | PASS | Focused tests cover exact two-file manifest, mutation, unsafe ID, output symlink, snapshot-ID symlink, descendant symlink, and unsupported special entries. |
| Classified searches and local links | PASS | Review A found no active v1 route and resolved 53 paths; parent pre-verification resolver checked 112 local links with none missing. |
| Full workspace fmt/Clippy/Nextest/deny | PASS | Both renewed r4 aggregates passed. Nextest runs `35e07ed8-ee99-4b26-89ef-2d675b5adb1d` and `e3208b83-1287-4723-be48-ef6b600bf5fd` each passed 1,974 tests with zero failures/errors. |
| Adjudicated CRAP `<= 30` | PASS | Both fresh acquisitions were closure eligible: threshold 30, 8,422 production entries, 222 sources, `raw/adjudicated/actionable = 2/2/0`, 13 touched files, zero touched actionable. |
| Markdown, links, spelling, diff | PASS | Direct Markdown lint/validation, 112 local links, shell syntax, workflow YAML parse, and `git diff --check` passed. `uk2us` differences were reviewed and not applied as described below. |
| Dual review/disposition/verification | PASS | Both independent reviews initially returned HOLD. Every finding was accepted and remediated; terminal Verification A and B passed with no open finding. |

## Terminal Aggregate Identities

| Route | Result | Key identity |
| --- | --- | --- |
| Ordinary validation | PASS, exit 0, 2,810.786 s | Nextest `35e07ed8-ee99-4b26-89ef-2d675b5adb1d`; retained 19-file evidence tree; ordered manifest `cc2b394362b03b2f78e68e1b2681e220d3fffa8544c79adc3228ed6b3d3019cd` |
| Release-mode transition exercise | PASS, exit 0, 2,828.559 s | Nextest `e3208b83-1287-4723-be48-ef6b600bf5fd`; snapshot `260714assure03r4`, manifest `d1f613ab0a1b47d3012fbc2edb55f7492485b2b9d9a5b1cd4a20dc3cc0e8f16f` |
| Renewed r4 source freeze | PASS | 40-file ordered manifest `a5355bf907b0e23efae776ba3c464404e21e6c2f669d4ff1a39a00008c6248b8` remained byte-identical through both aggregates, artifact finalization, and cleanup. |
| Final closure governance delta | PASS | Current manifest `1178d3b69e83a4e612bedb94f038dce0dd7d18074c251bcb27775d870d407bd7`; exact ten-row delta is eight closure documents plus the active-to-archived prompt move. Both terminal verifiers independently reproduced it; all 18 protected hashes match r4. |
| Historical r3 bounded metadata delta | PASS, superseded | The independently reproduced record remains valid for its historical freeze but is not the terminal identity for r4. |

The temporary transition-assembly tree was fully identified and then deleted;
its confirmed absence is recorded in `heavy-gate-runner.md`.

## Spelling-Preview Disposition

Final scoped-source `uk2us` preview reported three source files rather than a
zero-difference set:

- `docs/work-packages/README.md`: all matches are historical text or include
  technical identifiers such as `CoE` and package paths that the converter
  would corrupt; no ASSURE-03 line requires a change.
- `usersum/README.md`: `pre-release` is accepted project prose and is retained
  to avoid a nonmaterial post-heavy source change.
- `usersum/snow-frost-modeling-and-validation.md`: `non-agricultural` is an
  accepted technical compound, and `modelling` is part of a cited paper title
  whose source spelling must be preserved.

Disposition: **PASS with reviewed no-change differences**. No identifier,
historical record, technical term, or citation title was mechanically rewritten.
This disposition artifact also differs under `uk2us` because it quotes those
reviewed source spellings; that self-reference is not an additional source
finding.

## Explicit Non-Claims

- Stability: **NOT RUN** in the package's initial or terminal transition-route
  commands because `--skip-stability` is explicit. ASSURE-03 does not claim a
  conformant release candidate, release qualification, or production release.
- Release-candidate publication: the workflow now requires a separately
  successful stability job plus validation, preflight, assembly, and
  `success()` before candidate-named upload. Failed attempts use a non-candidate
  evidence name.
- External scientific peer review: **NOT CLAIMED**. The package uses internal
  coding-agent review of the migration and public wording.
