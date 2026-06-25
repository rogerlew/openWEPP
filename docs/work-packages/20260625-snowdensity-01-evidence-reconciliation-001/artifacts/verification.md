# Verification

Evidence mode: Ran + Static.

Commands run from `/home/workdir/openWEPP`:

```text
git status --short
```

Initial state was clean.

```text
jq -r '.sites[] | [...] | @tsv' target/snowfrost_fidelity_h/three_way_comparison.json
```

Ran to derive the SNOTEL density/depth/PySnobal summary values recorded in
`snotel-density-delta-ledger.*`.

```text
pdftotext references/copyrighted/D_Shen_020312.pdf target/snowdensity01/D_Shen_020312.txt
rg -n "snow|density|WEPP|sett|distribut|drift|..." target/snowdensity01/D_Shen_020312.txt
```

Ran to inspect the Shen thesis. The extracted text is uncommitted under
`target/`.

```text
sed -n '110,340p' /home/workdir/wepp-forest_260430_baseline/src/snowd.for
```

Ran to inspect settlement, density-mixing, and Eq. 3.7.5 source context.

```text
jq -e '.schema == "snowdensity-01-snotel-density-delta-ledger-v1"
  and .site_count == 5
  and .summary.fork_verdict_counts.STRUCTURAL == 5
  and .summary.openwepp_legacy_density_same_lineage == true
  and .summary.production_physics_changed == false' \
  docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/artifacts/snotel-density-delta-ledger.json
```

Passed.

```text
git diff --check
```

Passed.

```text
wctl doc-lint --path docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001
```

Ran after artifact creation and reported:

```text
0 files validated, 0 errors, 0 warnings
```

The scoped package path appears outside the markdown-doc catalog, so this is a
clean tool run but not a substantive Markdown validation pass.

Dual verification:

- Verifier A: JSON schema/value check and `git diff --check` passed.
- Verifier B: package scope review confirmed no production Rust or contract
  files changed, and the scoped doc-lint limitation is recorded above.

No Rust production files were edited, so cargo build/test/clippy/deny closure is
not a current-scope gate for this evidence-only package.
