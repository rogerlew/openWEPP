# RTR-016 Serial Qualification

Evidence class: Ran.

The exact four publication cases that timed out in attempt 5 were selected and
run with:

```bash
cargo nextest run \
  --config-file /home/workdir/testgate-serial-nextest.toml \
  --profile full \
  --test assurance_v2_publication_contract \
  -E 'test(approval_conflicts_and_release_mismatch_fail_before_publication) or test(all_mode_extra_staging_and_overlapping_roots_fail_without_public_mutation) or test(authority_bound_byte_negative_matrix_is_fail_closed) or test(authority_lifecycle_and_bound_byte_negative_matrix_is_fail_closed)'
```

No selection environment variable was set. The temporary configuration was an
exact copy of `.config/nextest.toml` except that the
`assurance-publication` group cap was reduced from four slots to two. Each
bound case retained `threads-required = 2`, so execution was serial; the
inventory and 720-second timeout were unchanged.

The run passed 4/4 in 267.751 seconds. The retained JUnit SHA-256 is
`27527b4ea1069c2fc6261f1a27a77e67622357f884a7d61b4b78cef34501b48e`.
The temporary configuration SHA-256 is
`66d611c62ac260c80063b363dcb64bb53e7c48f4d699f05c500c3a27272b40b9`.
The package copy preserves the exact JUnit bytes; the external temporary
configuration remains at `/home/workdir/testgate-serial-nextest.toml`.
