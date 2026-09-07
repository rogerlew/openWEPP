# Canonical authority inputs for reproduction

These are complete byte-exact canonical adjuncts, not authority replacements.
`manifest.json` binds original snapshots and e89-based patches by bytes/SHA-256.
`composition-manifest-02.json` corrects its arm recipe and binds the additional
2b-based deltas. `composition-manifest-05.json` binds the CURRENT A-source-04,
F-source-05 and R-source-05 manifests and actual current canonical hashes,
including the distinct `current-A-SG1` recipe. Version 04 preserves the preceding
kit binding; versions 02/03 preserve earlier source-kit bindings. Current kit
bindings do not retroactively change the runtime source of historical runs;
use each historical run's retained kit when reproducing that exact run.
Every arm starts from its OWN runtime source kit's exact base:
`e89befa4678eadec039b3e7f7fe0a176af8e9dc5` for A-source-04;
`2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3` for F-source-05/R-source-05.
The authority patch base must match that checkout, not another arm's base.

| Current identity | Checkout base | Authority adjunct | LSE snapshot |
| --- | --- | --- | --- |
| current-A-SG1 / A-source-04 | e89 | `patches/R-final-SG1.patch` (e89-to-SG1 despite historical filename) | Exact SG1 cut `7a002bca…` |
| current-F-common / F-source-05 | 2b | NONE: existing canonical files already match; `patches/F-common-from-2b.patch` records the zero-byte identity operation | Common commit blob `109be57c…`, not later PC1/SG1 |
| current-R-SG1 / R-source-05 | 2b | `patches/R-final-SG1-from-2b.patch` | Exact SG1 cut `7a002bca…` |

Historical A-initial-common and A-full-release-01-PC1 labels/inputs remain in
earlier manifests and run receipts: e89-to-common `patches/common-2b.patch` and
e89-to-PC1 `patches/A-full-release-01-PC1.patch`, respectively. They are NOT
the authority recipe for current A04. Version 05's `current_arm_recipe` is the
exclusive current-kit selector; retained `arm_recipe` labels are historical.

The e89 patches include both LSE and SNOW changes; the 2b-to-SG1 patch changes
only LSE because SNOW already matches. SNOW is the same exact
`94eb7a73…` blob for every cut. Complete hashes and byte lengths are in the
manifest; short prefixes here are labels, not verification inputs.

## Full-workspace reconstruction

1. Read the selected runtime source kit's `source-manifest.json` FIRST. Create
   a fresh detached FULL checkout at its exact `base` (not its `checkout` field
   and not a globally assumed e89), retaining
   all tracked `docs/`, fixtures, tools, configuration and source. Do not use
   the narrow scratch tree produced by `verify_source_kit.py` as a workspace
   reproduction. It intentionally checks only declared compiler-input paths.
2. Select the exact runtime source kit and evidence record for the desired arm;
   verify its manifest/patch/archive hashes. Apply its `tracked-source.patch`
   with `git apply --check` then `git apply`, and extract its
   `new-source-files.tar.gz` using the safe archive checks in
   `verify_source_kit.py`. Validate every source-manifest row. The latest
   source kits explicitly include 15 compiler JSON documents, but intentionally
   exclude these canonical authority adjuncts and documentary outputs.
3. Verify the original and current composition manifests' full hashes and the
   matching base row above.
   For F, apply NO authority patch: verify its already-present common canonical
   bytes/hashes. The empty identity patch was tested with `git apply --allow-empty`
   but is not needed to reproduce F. For A/R, from that same fresh checkout,
   run `git apply --check /absolute/path/to/authority-inputs/patches/<cut>.patch`
   and then `git apply` with the identical absolute path. Select exactly one
   row above; never apply an e89 authority patch to 2b or layer authority patches.
   The e89-to-SG1 patch is the current A04 recipe, NOT the R-source-05 recipe.
   Patches touch only the two
   named canonical files and are disjoint from the runtime-source patches.
4. Hash both resulting canonical files and require the exact manifest bytes
   and SHA-256. All other authority and package documents still come from the
   FULL base checkout, except separately recorded evidence/harness adjuncts.
5. Use the arm's actual recorded build, environment and full-gate commands.
   Reproduce any additionally runtime-read package artifacts identified by
   that gate's evidence; this two-file kit does not purport to inventory every
   documentary reader. A source reconstruction PASS is not a build/test PASS.

`freeze_authority.py` records how this artifact set was serialized: exact Git
blobs for common inputs, current canonical SG1, and its prefix only after the
entire prefix matched the previously frozen PC1 hash. No unavailable index,
historical binary, or superseded implementation was reconstructed. It is an
exclusive-create producer, not an in-place updater; rerunning against populated
artifact paths fails rather than overwriting evidence. Its scratch verification
applies each patch to exact base blobs and compares both resulting files byte
for byte. It runs no compilation or runtime tests.

`add_base_adjuncts.py` preserves those originals and adds the exact 2b identity
and 2b-to-SG1 operations, checks actual runtime-kit base fields, applies/checks
the new operations in scratch, and requires exact target canonical bytes.
`bind_source04.py` preserves that verification and binds the current source-04
manifests to those same unchanged deltas after checking exact bases and hashes.
It does not claim another build or runtime gate was run.
`bind_final_kits.py` similarly binds final A04/F05/R05 manifests after checking
the parent-declared full source identities, exact bases and unchanged deltas.
`bind_current_authority.py` checks the actual canonical files in main A and the
isolated F/R checkouts, then records separate current recipes and full hashes;
historical A authority labels remain preserved but do not select current A.
