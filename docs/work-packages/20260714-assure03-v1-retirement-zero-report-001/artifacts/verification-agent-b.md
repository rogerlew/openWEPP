# ASSURE-03 Verification B — Build, Release, And Security Integrity

Verification class: internal coding-agent verification; not external
scientific peer review

Evidence class: Static + Ran

Terminal verdict: **PASS**

The verifier made no repository content edits. All `B-001` through `B-006`
and `VB-001` through `VB-002` closure requirements pass on the renewed r4
tree. No new finding was raised.

## Independent Source And Gate Identity

- source status: 67 rows, 10,135 bytes, SHA-256 `38c55a522f7464ec6cacb93411687e40118248fa341d366b6d335d930b02e4f0`
- full-index diff: 439,133 bytes, SHA-256 `d60c66de0a040fd1a241773c336144fd26698a655014074ee0efbdc82ff77a49`
- present changed-path list: 40 paths, SHA-256 `ddaeb9d0beeef73ff53782e68292a4db127ccda2d76e6c37f2cd9c86922b202a`
- ordered content manifest: SHA-256 `a5355bf907b0e23efae776ba3c464404e21e6c2f669d4ff1a39a00008c6248b8`
- focused Nextest: 13/13 passed, UUID
  `e9ba6fb6-a1b8-43b8-b397-01e4e0f6fd3f`
- assurance-crate Clippy with warnings denied, shell syntax, workflow YAML
  parse, and `git diff --check`: PASS

The retained fresh CRAP evidence passed all 16 checksum checks. Its current
222-source manifest is byte-identical to retained SHA-256 `3a28ecde…`; the
threshold is 30 and `raw/adjudicated/actionable = 2/2/0`, with 13 touched
production files, no touched or untouched actionable row, closure eligibility,
and status PASS. The r4 heavy record reports both 1,974-test aggregates passed
with zero failures or errors.

## Fail-Closed And Consumer Verification

The exact catalog SHA-256 matches preflight authority `cb9cb601…`. Copied real
aggregate probes rejected every tested invalid state before release-directory
creation: a duplicate catalog key, any changed catalog byte, regular or
dangling-symlink transition markers, a catalog symlink, retired-root and nested
symlinks, a retired regular file, retired-root FIFO and Unix socket entries,
and a nested FIFO. The clean exact catalog passed preflight.

The reconstructed r4 snapshot manifest matched `d1f613ab…`, with exactly two
published files plus `manifest.json`. Unsafe IDs, a snapshot-ID symlink, a
descendant symlink, and a Unix socket failed closed.

The workflow separates ordinary validation evidence from explicit candidate
assembly. Candidate assembly requires the manual switch, successful workspace
validation, separately successful stability, successful preflight, and
successful assembly. Candidate upload is success-only; failure evidence uses
`openwepp-release-failure-evidence-*`.

The exact v1 recovery inventory also agrees: 51 rows, 27 removed paths, 24
preserved-or-revised paths, and zero content or action disagreement.

## Claim Boundary

Stability was **NOT RUN** in the package transition exercises. This verification
supports the zero-report transition implementation; it does not qualify a
release candidate, a production release, WEPPcloud vendoring, or scientific
conclusions.

## Final Closure-Tree Recheck

Verdict: **PASS**; the prior Verification B result remains applicable.

The verifier independently matched the final closure identities: status
`f59a166d…`, full diff `56e1dbed…`, paths `13cf0251…`, and manifest
`1178d3b69e83a4e612bedb94f038dce0dd7d18074c251bcb27775d870d407bd7`.
The r4-to-closure comparison is exactly 31 unchanged common rows, eight changed
closure-document rows, one removed prompt path, and one added archived prompt
path. All 18 protected implementation, test, workflow, release, assurance, and
public/science hashes matched r4. The prompt inverse reproduced `deed6353…`.

The retained 19-file validation tree remained 8,201,956 bytes with manifest
`cc2b3943…`; all 16 CRAP checksums passed. Retired routes remain absent, the
export remains empty with vendoring unauthorized, and stability/no-release-
qualification boundaries remain intact. No finding and no verifier edit.
