# Terminal verification: frozen-vector parity

Evidence class: `Static + Ran`

Exact commit: `646e95b40b098789e48f8bad1e519a019a9a3610`

Verdict: `PASS / PASS`

Verifier A ran the focused Rust parity test (1/1), the independent V10
authority/regeneration target (3/3, Nextest
`233f34d6-945a-4ad2-9858-e9e80a1d05bc`), LSE all-target Clippy with warnings
denied, and exact diff checks.

Verifier B used a clean detached copy and ran focused Rust parity (1/1), the
authority/regeneration target (3/3, Nextest
`d7c54cfa-1cf7-4051-984f-e255baac7578`), and exact diff checks. The ignored
repo-local Python interpreter was linked into the detached copy before the
successful exact-commit regeneration run.

Both independently confirmed five actual `leaf_trial_state` vectors, exact
branch categories, numeric `Ci`/`Ag`/`An`/reconstructed `Rd`/`rs` coverage,
literal negative-zero preservation, unchanged tolerance, and a
behavior-neutral production extraction. Neither found a blocking issue.
