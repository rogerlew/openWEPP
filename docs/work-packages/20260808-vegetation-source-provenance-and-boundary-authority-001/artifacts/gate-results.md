# Gate Results

Status: PASS

Evidence mode: Ran on 2026-08-08.

## Reviewed-Input Identity

- `SC-VEGETATION-001.md`:
  `04edbb2c2b02f8b9efbd12ef2bc9656a2f8ebfe12f8af8e38bddd356e0d4df3a`
- `vegetation_boundary_authority_contract.rs`:
  `dde7238ce9789bb4b63aa2c978a989a8a9857572c6a166ae0b596d3d55b732da`
- `Cargo.toml`:
  `917d1c210c79c7906a1f3504bcc98bc23fb57d9cea0ba4d8c6ae7d565f70e4b5`
- `assurance/v2/identity.lock.json`:
  `c7688337b0ec504431f9cde8e76924cd3ba8074fdc1fba9b980adb5d279ba96f`

The required comparator-suite runner bound both workspace profiles to these
same reviewed bytes.

## Required Heavy Profiles

| Command | Result | Exact summary |
|---|---|---|
| `cargo nextest run --workspace --profile full` | PASS | 2323 passed, 0 failed, 33 skipped; 2320.055 s (`real 2321.82 s`) |
| `TMPDIR=<RAM scratch> cargo nextest run --workspace --profile quick --test-threads 4` | PASS | 2274 passed, 0 failed, 40 skipped; 3094.541 s (`real 3099.90 s`) |

The successful quick command retained the exact `profile.quick` selection and
all 2274 selected tests. Only Nextest scheduling was bounded to four tests and
`std::env::temp_dir()` was placed in a dedicated `mktemp` directory on
`/dev/shm`; the directory was removed after the run.

Two prior current-input quick attempts are retained as failed infrastructure
observations, not passes:

- the first exact default-concurrency run reached `181 passed, 1 timed out,
  40 skipped` before fail-fast cancellation when
  `receipt_preparation_is_reused_only_when_bytes_match` exceeded 600 seconds;
  there was no assertion failure; and
- the exact retry put 16 assurance scratch-copy tests beyond 600 seconds and
  the Nextest parent hung while reaping terminated children. It was signalled
  after 723.63 seconds with no assertion failure and no terminal summary.

Process inspection showed the affected workers blocked in kernel
`wb_wait_for_completion` on the saturated `/tmp` filesystem. A four-thread
`/tmp` diagnostic and a one-thread `/tmp` diagnostic reproduced that writeback
condition and were stopped without being claimed as gates. The RAM-backed run
then passed the formerly timed-out receipt case in about five minutes and
completed the entire quick inventory. Runner logs were transient local
evidence and are not package deliverables.

## Focused And Authority Gates

- `cargo nextest run --test vegetation_boundary_authority_contract`: PASS,
  8/8.
- `cargo nextest run --test assurance_v2_source_contract`: PASS, 12/12.
- `target/release/openwepp-assurance validate --all`: PASS, three DRAFT
  reports and zero public reports.
- Strict Binding Exposure for `SC-VEGETATION-001`: PASS, 1/1 contract row.
- Unit compliance for `SC-VEGETATION-001`, `SC-PLANT-001`, `SC-EVAP-001`,
  `SC-RESIDUE-001`, `SC-WATBAL-001`, and
  `SC-LANDSURFACEENERGY-001`: PASS.
- `cargo fmt --all -- --check`: PASS.
- Markdown lint for the new contract and package tree: PASS.
- `git diff --check`: PASS.

Final lifecycle-only documentation changes are rechecked by the focused gate
pass. Any review-remediation change to science/test/Cargo inputs requires new
heavy-profile input identities and reruns before terminal disposition.
