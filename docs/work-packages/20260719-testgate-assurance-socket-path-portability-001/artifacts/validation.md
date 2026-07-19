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
terminal plan remains pending.
