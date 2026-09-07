# Canonical adjunct reproduction evidence

Ran: `reproduction/authority-inputs/freeze_authority.py` completed three
base-to-canonical patch reconstructions with exact byte equality. No builds,
runtime gates, canonical or executable-source changes were performed.

Manifest: `reproduction/authority-inputs/manifest.json`, SHA-256
`63d57160e82ddec88fe307af7c7a104dc4a96ffafc7be526cdb8da3586debbd3`.
It preserves A initial common, A full-release-01 PC1, F common, and R final SG1
authority inputs separately, including the common SNOW contract. Each complete
snapshot and patch has byte length and SHA-256. The PC1 prefix independently
matched `c993020e…` before serialization. Exact common blobs came from commit
`2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3`; the original three patches target
`e89befa4678eadec039b3e7f7fe0a176af8e9dc5`.

Read/ownership: `tools/agents/find-agents --for` resolved root/work-package
guidance; previously read complete instructions remain applicable. Read current
source-manifest and full `verify_source_kit.py` before artifact creation.
Owned writes are only the new `authority-inputs/` subtree and this record;
existing source kits, scripts, canonical files, gates and other agents' edits
are preserved. First producer attempt failed before any output because its
repository parent index selected `/workdir`; corrected to the actual checkout
and reran successfully. No failed run is labeled PASS.

Scope limitation: the narrow source-kit verifier includes its declared compiler
inputs (now including 15 JSON documents); it does NOT reconstruct a full root
integration-test workspace or prove all document readers can run. The adjunct
README requires a FULL exact-base checkout, runtime source kit, and selected
canonical patch, preserving all other base documents. Additional gate-specific
runtime-read artifacts remain governed by the corresponding evidence record.

## Corrected arm-base composition

The initial README incorrectly sent F/R runtime patches to e89 even though
their source manifests declare base 2b. Its individual authority patch checks
passed, but did not prove that erroneous composition. The corrected README
requires each arm's exact runtime-kit base FIRST. Original snapshots, patches
and manifest remain unchanged; `composition-manifest-02.json` supersedes only
the arm recipe and adds byte/hash-bound exact-base adjuncts.

Ran: `authority-inputs/add_base_adjuncts.py` checked A-source-03 base e89 and
F-source-03/R-source-03 base 2b, applied each new authority operation to exact
2b canonical blobs in scratch and required byte equality. F is a zero-byte
identity operation (tested with `git apply --allow-empty`; reproduction needs
only existing-file hash checks). R uses `R-final-SG1-from-2b.patch`, 8823 bytes,
SHA-256 `38ed62a786a234da6565abebed0ed4f5b004b3b58557eaba860c2958cd6aec6e`.
A retains its e89 common/PC1 patches. No source/canonical changes or builds.

## Current source-04 binding

Ran: `authority-inputs/bind_source04.py` checked the actual current F-source-04
and R-source-04 manifest hashes/byte lengths/source identities and their exact
base 2b; A remains A-source-03/base e89. Both authority delta byte/hash checks
passed unchanged. New `composition-manifest-03.json` is 3064 bytes, SHA-256
`0a66c5e4c12896d81ddbb9f9123e94a3950ff59608e8044bbe0abd5c533cfeb1`.
Original and version-02 manifests remain unchanged. Version 03 binds current
runtime kits to the previously verified base-matching adjuncts; it does not
claim a new build, runtime gate or full-workspace reproduction execution.

## Final A04/F05/R05 binding

Ran: `authority-inputs/bind_final_kits.py` verified actual manifest bytes/hashes,
declared complete source identities and exact bases for A-source-04 (e89),
F-source-05 (2b), and R-source-05 (2b). Authority delta hashes remain unchanged.
New `composition-manifest-04.json`: 3070 bytes, SHA-256
`ed78068ed45ef1e17ac6b56f017b56fc0d224caf61bcabe6001a2a240f58b497`.
All earlier manifests remain preserved. This final-kit recipe does not rewrite
historical run source provenance; exact historical reproductions use their
original source kit plus matching authority cut. No source/canonical edits,
builds, gates or comparative measurements were performed by this update.

## Current canonical selection, distinct from historical A labels

Ran: `authority-inputs/bind_current_authority.py` directly verified both current
canonical files in main A and isolated F/R against frozen snapshots. Current
A04 is SG1 (`7a002bca…`), so its e89 base uses `patches/R-final-SG1.patch`;
F05 remains common (`109be57c…`) at 2b with no authority change; R05 is SG1 at
2b and uses `patches/R-final-SG1-from-2b.patch`. SNOW remains `94eb7a73…`.
Complete hashes/bytes and separate current recipes are bound in
`composition-manifest-05.json`, 5667 bytes, SHA-256
`0a03c8e58192368dd72c6ea8e30afe40ad8a4c72c0a18459faec746b7cd04b42`.
Historical A-initial-common/A-full-release-01-PC1 inputs and version 04 are
preserved; current kits select only version 05's `current_arm_recipe`. No
canonical/source edits, builds, gates or timing execution occurred here.
