# ASSURE-03 Post-Heavy Bounded Delta Audit

Evidence class: `Ran`

Verification: `A`

Finding: `VA-001`

Disposition: `PASS — CLOSED`

## Question

Do the two asserted post-heavy documentation edits exactly explain the current
tree's departure from the heavy runner's 40-file content manifest, and do the
heavy code, test, and workflow results remain applicable?

The asserted edits were:

1. Replace the unchecked package progress line with the completed heavy-gate
   progress item.
2. Change the active prompt's required-reading budget from `86,510` to
   `86,652` bytes.

This audit wrote no source or existing artifact. It created only this audit
artifact after completing the in-memory reconstruction.

## Result

Yes. Independently reversing exactly those two transformations in memory
reconstructed the heavy runner's prior 40-file manifest SHA-256 exactly:

`1643369bcb82d9b4b24cb61934d657790ceb87fbc7f9651f7eae4603166462be`

The current 40-file manifest SHA-256 is:

`ff7ca2c0072fc5edfbf39ea8a3cfeda13566d3bd1ec58e6e28d34f5774ee4fc5`

Exactly two manifest rows differ. The other 38 paths and content hashes are
unchanged. The status and present-file path identities also equal the heavy
runner's recorded values.

## Exact File Selection And Manifest Encoding

The audit reproduced the heavy runner's selection and manifest format:

```bash
source_status() {
  git status --porcelain=v2 --untracked-files=all |
    rg -v ' docs/work-packages/[^ ]+/artifacts/'
}

source_paths() {
  source_status |
    awk '{print $NF}' |
    while IFS= read -r p; do
      if [[ -f "$p" ]]; then
        printf '%s\n' "$p"
      fi
    done |
    LC_ALL=C sort -u
}

current_manifest() {
  source_paths |
    while IFS= read -r p; do
      sha256sum "$p"
    done
}
```

Thus each manifest row is the lowercase SHA-256, two ASCII spaces, the
repository-relative path, and a newline. Rows are ordered by the
`LC_ALL=C`-sorted present-file path set. Package `artifacts/**` paths are
excluded before path extraction.

Observed selection identities:

- status: 67 rows; SHA-256
  `38c55a522f7464ec6cacb93411687e40118248fa341d366b6d335d930b02e4f0`
- present-file paths: 40 rows; SHA-256
  `ddaeb9d0beeef73ff53782e68292a4db127ccda2d76e6c37f2cd9c86922b202a`

Both equal the heavy runner's records. Both changed files remain the same
untracked `?` status entries that were already in the selected path set:

```text
? docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/package.md
? docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/prompts/active/20260714-codex-execute-assure03_prompt.md
```

The full-index binary Git diff also retained the heavy identity, 437,314 bytes
and SHA-256
`d94946ec7e38711d8fbcca05a90c7bc8fb2d218fcde94226c89a0252a6788233`.
That is expected because `git diff HEAD` does not include these two untracked
files. The present-file manifest, not the Git diff, detects their content
changes.

## Bounded Patches

The package transformation from the heavy freeze to the current tree was:

```diff
-- [ ] Run focused and full closure gates.
+- [x] (2026-07-15) Passed focused remediation gates and both terminal full
+  transition-route aggregates, including fresh adjudicated CRAP at threshold
+  30 with zero actionable rows.
```

The completed item is three physical source lines. Replacing those exact three
lines and their terminating newlines with the single prior line is the inverse
used by this audit.

The active-prompt transformation was:

```diff
-Required-reading budget: 86,510 local bytes for Core, `OK`; map:
+Required-reading budget: 86,652 local bytes for Core, `OK`; map:
```

Each inverse required exactly one match. The package inverse additionally
required the three current lines to be adjacent and byte-exact; a missing,
duplicate, reordered, or differently wrapped match caused the reconstruction
command to exit nonzero.

## In-Memory Reconstruction Algorithm

The audit used `awk` filters over standard input/output; neither filter used an
in-place option or wrote a source file.

```bash
reverse_package() {
  awk '
    BEGIN { n=0 }
    $0 == "- [x] (2026-07-15) Passed focused remediation gates and both terminal full" {
      if ((getline second) <= 0 || second != "  transition-route aggregates, including fresh adjudicated CRAP at threshold") exit 91
      if ((getline third) <= 0 || third != "  30 with zero actionable rows.") exit 92
      print "- [ ] Run focused and full closure gates."
      n++
      next
    }
    { print }
    END { if (n != 1) exit 93 }
  ' "$package"
}

reverse_prompt() {
  awk '
    BEGIN { n=0 }
    $0 == "Required-reading budget: 86,652 local bytes for Core, `OK`; map:" {
      print "Required-reading budget: 86,510 local bytes for Core, `OK`; map:"
      n++
      next
    }
    { print }
    END { if (n != 1) exit 94 }
  ' "$prompt"
}
```

The SHA-256 of each filter's output became that path's reconstructed prior
hash. The audit regenerated the 40 manifest rows in the exact selection order,
substituting those two reconstructed hashes and using `sha256sum` on the
current file for every other path. Finally:

```bash
reconstructed_manifest | sha256sum
paste <(current_manifest) <(reconstructed_manifest) |
  awk '$1==$3 && $2==$4 {same++} $1!=$3 || $2!=$4 {changed++}
       END {print same, changed}'
```

reported the recorded heavy digest and counts `38 2`. A supplemental bytewise
comparison with the heavy runner's still-available local scratch manifest also
reported `MATCH`; closure does not depend on that scratch file because the
published heavy digest is reproduced independently.

## Prior And Current File Identities

Package:
`docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/package.md`

- reconstructed prior: 12,926 bytes; SHA-256
  `3684efcf7d8cdc56d7adf82b54f31ad5408445f3b0b225ba62d62015e93e08f8`
- current: 13,068 bytes; SHA-256
  `445101744025b86085d39c7032919f58395feeb9e23b30ccaca133c4201522cc`

Active prompt:
`docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/prompts/active/20260714-codex-execute-assure03_prompt.md`

- reconstructed prior: 2,621 bytes; SHA-256
  `6b9b868eab6ec1323f547bac62086423aad8aea61a89e6605d63ea4def8e5c7c`
- current: 2,621 bytes; SHA-256
  `deed63535f4cac9d3cdb1c696e1b91b10274dfb16e5e18105eddafcc149dad4f`

The current and reconstructed manifests differed only on these two path rows.
All other 38 row pairs had identical paths and SHA-256 values.

## Mtime Chronology

Filesystem mtimes, shown with nanoseconds and the local UTC-07:00 offset, were:

1. heavy gate artifact: `2026-07-15 03:06:02.060983789 -0700`
2. package: `2026-07-15 03:08:17.978600807 -0700`
3. active prompt: `2026-07-15 03:08:30.267838033 -0700`

The package mtime is 135.917617018 seconds after the heavy artifact; the prompt
mtime is 148.206854244 seconds after the heavy artifact and 12.289237226
seconds after the package. Rounded UTC times are `10:06:02Z`, `10:08:17Z`, and
`10:08:30Z`, respectively. Mtimes support the asserted ordering but are not the
content proof; the exact inverse hashes and manifest reconstruction provide
that proof.

## Applicability And Closure

The heavy runner's exact whole-tree content identity is historical and must not
be described as the current 40-file identity. The current identity is
`ff7ca2c0072fc5edfbf39ea8a3cfeda13566d3bd1ec58e6e28d34f5774ee4fc5`.

However, the heavy code, test, workflow, release-route, and adjudicated-CRAP
results remain applicable to the current tree. The bounded reconstruction
proves that every one of the other 38 selected file hashes is unchanged. The
only deltas are package progress evidence describing the completed gates and a
required-reading byte-budget correction in the active execution prompt.
Neither changes production code, tests, workflow behavior, gate scripts,
fixtures, public outputs, or CRAP adjudication authority.

No heavy rerun is required to disposition these two documentation-only deltas.
The original qualification boundary remains unchanged: the heavy evidence is
transition-route verification with stability explicitly skipped, not a
conformant release-candidate or release-qualification claim.

Verification A finding `VA-001` is closed as `PASS`.
