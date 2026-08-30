# Verification A

Status: `COMPLETE — HOLD`

Evidence mode: `Static + Ran`

## Verified source identity

The production/contract correction is clean commit
`6953a36b881e7167b47c76040208d1024818060a` (tree
`3ba958a710ae03c9b255268792a423aa91f8efcf`). The terminal source-policy
amendment adds only:

- the test-only authoritative include binding in
  `open_snow_tail_tests.rs` (blob `23f0e3859f508711850dc0d72f4218f45dd252e4`,
  diff SHA-256
  `da1bca7bf268565db7cba44d1e31c41fb6bdde9b06c5cff639d397a6fdd023ec`);
- authority impact-map generation 37 with an exact
  `SC-SNOWENERGY-001` binding for that test path (blob
  `ff0d9047b36c3081d8b8f746532601660f3f4e02`).

Their combined terminal source-policy diff SHA-256 is
`5951d26091b0f515108e5651c751dbe91004eb8cb910db77c49ed14b8ec97405`.
Concurrent worktree changes outside those two paths are package evidence and
review artifacts; I did not treat them as a different executable identity.

## Findings

### VA-001 — HIGH — required full correctness profile remains failed

The retained full profile is complete but not passing. Its log contains 96
unique failed tests and 29 unique timed-out tests, each emitted twice by the
configured retry behavior: 125 nonpassing of 3,628 attempted, 3,503 inferred
passing, and zero not-run. SHA-256 is
`dbdd682aa9c654f08955f65d7b74addfad999691be21c678ecd6da977f0b35ee`.
The terminal source-order binding repairs the three package-owned source-scan
failures and independently passes their full five-test module, but the other
122 failures/timeouts have no governing waiver or causality disposition that
can convert the required gate to `PASS`. This leaves RA-002 and RB-001 open.

### VA-002 — HIGH — required warnings-denied lint remains failed

The retained workspace warnings-denied Clippy run stops with
`filter_map_bool_then` in `openwepp-coupled-time` and `similar_names` in
`openwepp-biogeochemistry`; SHA-256 is
`aac68d695f1d8f2e06f687c01aa199cc25d48f8d708a958763266e4323d11637`.
No passing affected-package/reverse-dependent substitute or governing
exception is recorded. This leaves RA-002 and RB-002 open.

## Accepted-finding closure

| Finding | Verification | Evidence |
|---|---|---|
| `RA-001` | `CLOSED` | v29 consistently requires bitwise authentic-candidate density without interpolation while exact density mismatch remains nonconverged; the focused density vector and production helper agree. |
| `RA-003` | `CLOSED` | `CoveredFinalizationStabilizationV1` is the production-owned state seam; the focused vector proves restart observation, retention through nonconvergence, exactly-once consumption, following acceptance, and exact-floor nonactivation. |
| `RB-003` | `CLOSED` | v29 binds `INV-SNOWENERGY-054` in the branch table, primary guard map, formal vectors, and child obligation map. |
| `RB-004` | `CLOSED` | the clean-commit canonical log digest matches; independent reconstruction gives 491 accepted / 205 rejected and the reported width histogram, with unchanged closure passing. |
| `RB-005` | `CLOSED` | actual counts are 2,721/529 lines; baseline and include semantic bodies both hash to `97aec7cad748caac7a2b3c6fbf2c1023074495f6b4ce233c95893bb9bd10bdd5`; the required WARN and follow-up intent are recorded. |
| `RB-006` | `CLOSED` | the intended write set, helper/version language, line counts, source-order binding, and generation-37 impact-map binding are reconciled. Exact-source A0 admission now passes with authority SHA-256 `ce2befbdb7214be8194f01d3f8645663ce916a232ff476cc21692986034dad1a`. |

## Independent commands

- `nix develop -c cargo fmt --all -- --check` — `PASS`.
- `git diff --check` and `git diff --check 792af753e..6953a36b8` — `PASS`.
- `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(covered_convergence_policy_tests)'`
  — `PASS`, 19/19, run ID
  `1075c79c-91c7-4ae4-b71d-1e8c5cc69e56`.
- `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(precomputed_terminal_accepted_executor_tests)'`
  — `PASS`, 5/5, run ID
  `49c7065f-f0e5-4895-9dd1-a4cc976a9e69`.
- `nix develop -c cargo check --workspace --all-targets --all-features` —
  `PASS`.
- `bash tools/release/check_science_contract_admission.sh --base-ref be40a9435 --worktree`
  — `A0_ADMITTED`, 49 contracts, four science surfaces, terminal authority
  SHA-256
  `ce2befbdb7214be8194f01d3f8645663ce916a232ff476cc21692986034dad1a`.

## Retained canonical evidence reconciliation

The clean one-day log SHA-256 is
`c6ba3bdb3a9bfd5d0bdd35e83fdb2f448dcd97dba67d70811d418e64cb856417`.
Independent parsing reconstructs 491 accepted and 205 rejected trials and
accepted widths `{60: 49, 120: 92, 180: 320, 240: 17, 300: 3, 420: 3,
480: 1, 900: 3, 1800: 3}` seconds. It records 32 fixed-point caps, 45 scaled
comparison rejections, zero discrete rejections, and 339.10 s body wall.
Mass residual `3.55271367880050093e-15 kg m^-2`, energy residual
`1.39698386192321777e-9 J m^-2`, receipt-energy residual
`9.96351445792242885e-10 J m^-2`, and receipt-temperature residual
`1.07434061646927148e-11 K` all remain inside the unchanged recorded bounds.
The timing sidecar matches 416.72 s elapsed and 6,314,564 KiB maximum RSS.

The current source-order and impact-map changes are test/policy-only and are
not dependencies of the production runner used by that clean canonical run;
the clean executable result remains applicable to the terminal production
identity. I did not rerun the one-day or full profile.

## Non-blocking debt and follow-up

- `open_snow.rs` remains a correctly recorded 2,000-line `WARN`; perform the
  named fixed-point-loop extraction on its next authorized behavioral touch.
- The accepted-endpoint source tests now inspect the correct include, but
  textual ordering assertions remain mechanically sensitive; prefer an
  executable transaction seam if one can later prove the same rollback and
  publication-order obligations without expanding runtime API surface.

## Verification verdict

`HOLD`. The corrected fixed-point implementation, focused tests, canonical
performance result, and package-specific closure evidence verify successfully,
and RA-001/RA-003/RB-003/RB-004/RB-005/RB-006 are closed. Package `COMPLETE`
is still prohibited by the failed mandatory full correctness profile and
warnings-denied Clippy gate (RA-002/RB-001/RB-002). No local evidence or
independent-verification label waives those failures.
