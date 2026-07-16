# Assurance Editorial Fast Path Heavy Gate Runner

Status: PASS — CRAP REMEDIATION CLOSURE

Evidence class: Ran

Date: 2026-07-16 UTC

The delegated heavy runner did not edit production source. The initial gate
sequence stopped at its first nonzero result. The later remediation sequence
ran every required gate against a newly frozen candidate and passed.

## Candidate Identity

- Frozen base and current `HEAD`:
  `25bcb17f4a62924976a19381e974a36612ed4845`.
- Accepted `normalization.rs` SHA-256:
  `89a83b4351581b08719cda4d8e42a7d1e75e9ef9bc0534077128b42c41388a1f`.
- Fifteen-path non-package candidate manifest SHA-256:
  `ab38aa48e55b2a4060f3744a69c18ce30c0b6455b6e66b101714a4e3e35c37f2`.
- Candidate manifest:
  `/tmp/assurance-editorial-heavy-accepted-manifest.txt`.

## Gate Results

| Gate | Exit | Time | Result | Evidence |
| --- | ---: | ---: | --- | --- |
| `cargo fmt --check` | 0 | 2.43 s | PASS | `/tmp/assurance-editorial-heavy-accepted-cargo-fmt.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | 6.71 s | PASS | `/tmp/assurance-editorial-heavy-accepted-cargo-clippy.log` |
| `cargo nextest run --workspace --profile full` | 0 | 591.32 s | PASS | `/tmp/assurance-editorial-heavy-accepted-nextest-full.log` |
| `cargo deny check` | 0 | 2.74 s | PASS | `/tmp/assurance-editorial-heavy-accepted-cargo-deny.log` |
| Fresh adjudicated CRAP gate | 1 | 2,300.40 s | FAIL | `/tmp/assurance-editorial-heavy-accepted-adjudicated-crap.log` |
| Actual `normalize --check` | — | — | NOT RUN | stopped after CRAP failure |
| Selected `validate` | — | — | NOT RUN | stopped after CRAP failure |
| `git diff --check` | — | — | NOT RUN | stopped after CRAP failure |

Full Nextest run ID:
`8c5c1f94-e2bb-4638-8afe-3b77a8883273`.

Nextest ran 2,063 tests: 2,063 passed, 3 skipped, and 7 slow. Its own test
time was 589.721 seconds; the complete command took 591.32 seconds. Maximum
resident set size was 209,276 KiB.

## Adjudicated CRAP Failure

The exact command was:

```console
bash tools/release/run_adjudicated_crap_gate.sh --base-ref 25bcb17f4a62924976a19381e974a36612ed4845 --output-dir docs/work-packages/20260716-assurance-editorial-fast-path-001/validation-evidence/adjudicated-crap
```

The fresh gate assessed 9,372 production entries. It reported 4 raw rows over
30, 2 currently adjudicated rows, 2 actionable rows, and 7 touched production
files. Both actionable rows are in touched production source; none are outside
the touched set.

| File | Function | Line | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: | ---: |
| `crates/openwepp-assurance/src/v2/normalization.rs` | `normalize_report_with_controls` | 155 | 29 | 85.14851485148515 | 31.75489881112413 |
| `crates/openwepp-assurance/src/v2/normalization.rs` | `clone_v2_tree` | 957 | 18 | 66.66666666666666 | 30.000000000000018 |

The latter value renders as 30 in the Markdown report but is strictly greater
than 30 at full precision, so the gate correctly classifies it as actionable.
The maximum actionable touched-file CRAP is 31.75489881112413.

The production-source manifest contained 229 paths and remained
`9e19cb291fa78e709777f514d950a843f213d1f8361593f9528d748b672a33e7`
before, after, and at finalization. The failure is therefore metric debt, not
candidate drift. The CRAP JSON SHA-256 is
`1a71a820c5456134f807ecf374ff7b5ccde5f8d5a590c62915b4d157dcf44cbc`;
the LCOV SHA-256 is
`b360f9763b476fba7a29b40b9691884c59ed6615b737c9456af562b630138fc4`.

Complete fresh evidence is under
`validation-evidence/adjudicated-crap/`, including the report, LCOV, raw CRAP
JSON, source manifests, registry snapshot, and checksums. At that point, heavy
technical closure remained open. No waiver or human approval judgment was
claimed.

## CRAP Remediation Rerun — Terminal PASS

The remediation retained the initial failure evidence above and wrote fresh
coverage evidence to a separate directory.

### Remediated Candidate Identity

- Frozen base and current `HEAD`:
  `25bcb17f4a62924976a19381e974a36612ed4845`.
- Final `normalization.rs` SHA-256:
  `eb4f51a0f2258ca32c819960db98f07f1adf2523e224b9a170e9372a2ecbd57b`.
- Fifteen-path non-package candidate manifest SHA-256:
  `37eb91bba3cb676c0458013df91d42ad3a2103e4bf3b5dc73ab051c951e856bf`.
- Candidate manifest:
  `/tmp/assurance-editorial-heavy-final2-manifest.txt`.

### Remediation Gate Results

| Gate | Exit | Time | Result | Evidence |
| --- | ---: | ---: | --- | --- |
| `cargo fmt --check` | 0 | 2.54 s | PASS | `/tmp/assurance-editorial-heavy-final2-cargo-fmt.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | 4.96 s | PASS | `/tmp/assurance-editorial-heavy-final2-cargo-clippy.log` |
| `cargo nextest run --workspace --profile full` | 0 | 591.23 s | PASS | `/tmp/assurance-editorial-heavy-final2-nextest-full.log` |
| `cargo deny check` | 0 | 0.97 s | PASS | `/tmp/assurance-editorial-heavy-final2-cargo-deny.log` |
| Fresh adjudicated CRAP remediation | 0 | 2,755.11 s | PASS | `/tmp/assurance-editorial-heavy-final2-adjudicated-crap.log` |
| Actual `normalize --check` | 0 | 4.87 s | PASS | `/tmp/assurance-editorial-heavy-final2-normalize-check.log` |
| Selected `validate` | 0 | 0.34 s | PASS | `/tmp/assurance-editorial-heavy-final2-validate.log` |
| `git diff --check` | 0 | 0.03 s | PASS | `/tmp/assurance-editorial-heavy-final2-git-diff-check.log` |

Full Nextest run ID:
`d2fa2208-86ad-4a98-91e7-fb2f0ed0aa9f`.

Nextest ran 2,063 tests: 2,063 passed, 3 skipped, and 19 slow. Its own test
time was 589.498 seconds; the complete command took 591.23 seconds. Maximum
resident set size was 209,316 KiB. The elevated slow count was timing evidence
under shared-system contention; no test failed.

### Fresh CRAP Remediation Evidence

The exact fresh command was:

```console
bash tools/release/run_adjudicated_crap_gate.sh --base-ref 25bcb17f4a62924976a19381e974a36612ed4845 --output-dir docs/work-packages/20260716-assurance-editorial-fast-path-001/validation-evidence/adjudicated-crap-remediation
```

The gate assessed 9,392 production entries. It reported 2 raw rows over 30,
2 currently adjudicated rows, 0 actionable rows, and 7 touched production
files. The maximum CRAP in touched production files was exactly 30.0 in
`publish_selected`; the maximum in `normalization.rs` was
15.101256515775034 in `prepare_normalization`.

The formerly actionable functions now had these fresh full-workspace values:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `normalize_report_with_controls` | 167 | 8 | 100 | 8 |
| `clone_v2_tree` | 1055 | 6 | 100 | 6 |
| `prepare_candidate` | 373 | 5 | 100 | 5 |

The remediation source manifest contained 229 paths and remained
`72d2fa3d449fc492a05818daa680b548d2aa6bb14b6c0428ab8af6b0e16873ae`
before, after, and at finalization. The CRAP JSON SHA-256 is
`5780a6f5c6b788f8342a6918648e3ed32b055679a53df85b85bc86d43bc8327a`;
the LCOV SHA-256 is
`99da70468234dcb4638d0582687fa068f68088500ad100b50839a035f2427f53`.
Complete evidence is under
`validation-evidence/adjudicated-crap-remediation/`. The original failed
acquisition remains under `validation-evidence/adjudicated-crap/`.

### Actual Consumer Checks

The named normalization check returned `changed=false`, an empty change list,
and identical old/new source root
`08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6`.
Selected validation admitted one report at version `1.0.0`, lifecycle `DRAFT`,
with `fixture_only=false` and the same report source root. `git diff --check`
produced no output.

Technical heavy closure is PASS. This result does not create or imply human
scientific, assurance-steward, or release-owner approval.
