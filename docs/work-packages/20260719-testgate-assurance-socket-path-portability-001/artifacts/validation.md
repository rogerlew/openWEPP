# Validation

Evidence class: `Ran`

With `TMPDIR` set to the retained failing coverage directory
`/tmp/tgPXAv/execution/.work/target/affected-crap/tmp`:

```text
cargo nextest run --test assurance_dossier_build_contract \
  transition_preflight_rejects_symlink_evasions_before_release_directory
1 passed; 12 skipped; 0.130 seconds
```

Additional focused gates:

- `cargo clippy --test assurance_dossier_build_contract -- -D warnings` — PASS.
- `cargo fmt --check` — PASS.
- `git diff --check` — PASS.

No manual broad suite, GitHub workflow, or forest1 action ran. The mechanical
terminal plan selected 10 bounded-component nodes. Nine passed. Affected CRAP
passed the corrected dossier case, then failed in a separate assurance-v2
socket case because the runner overrides `TMPDIR` with the deep
`target/affected-crap/tmp` path. Receipt
`065aec86b96505f794d7f8122a638272b928ad78107c7aabbc07dffa0f377b6a` is
retained as `LOCAL_UNTRUSTED` / `FAIL`; no result is relabeled.
