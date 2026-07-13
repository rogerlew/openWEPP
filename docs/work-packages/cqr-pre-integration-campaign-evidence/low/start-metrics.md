# Low Start Metrics

Evidence class: **Ran + Static**

Status: `PASS`

## Source And Protocol

- Repository: `/home/workdir/openWEPP`.
- Clean source commit: `83f73e3dcaa3330785f89d08e626717f409d0fba`.
- Literal slug/phase: `low` / `start`.
- The required `comparator_suite_runner` performed one instrumented workspace
  execution. No test rerun or coverage-profile cleanup occurred after it.
- The run retained 311 `.profraw` files through both corrected report commands.

The binding one-profile protocol was:

```text
cargo llvm-cov clean --workspace
/usr/bin/time -v -o docs/work-packages/cqr-pre-integration-campaign-evidence/low/start/run.time cargo llvm-cov --workspace --ignore-run-fail --no-report > docs/work-packages/cqr-pre-integration-campaign-evidence/low/start/run.log 2>&1
cargo llvm-cov report --lcov --output-path /tmp/openwepp-cqr-preint-low-start-default.lcov
cargo llvm-cov report --json --output-path /tmp/openwepp-cqr-preint-low-start-default.json
cargo metadata --no-deps --format-version 1 | jq -r '.packages | sort_by(.manifest_path)[] | .name' > docs/work-packages/cqr-pre-integration-campaign-evidence/low/start/report-packages.txt
cargo llvm-cov report <one --package argument for each entry in report-packages.txt> --lcov --output-path docs/work-packages/cqr-pre-integration-campaign-evidence/low/start/start.lcov
cargo llvm-cov report <the same package arguments> --json --output-path docs/work-packages/cqr-pre-integration-campaign-evidence/low/start/start.json
cargo crap --workspace --lcov docs/work-packages/cqr-pre-integration-campaign-evidence/low/start/start.lcov --min 0 --format json --output docs/work-packages/cqr-pre-integration-campaign-evidence/low/start/start-crap.json
```

The virtual-workspace default report reproduced the Medium tooling behavior:
LCOV was zero bytes and JSON reported zero lines despite an exit-zero result.
The report-only correction expanded the exact 18 members from `cargo metadata`
into repeated `--package` arguments. It reused the same retained profile and
did not execute or clear tests. The two empty/default logs and timings are
archived beside the corrected reports.

## Results

| Step | Exit | Elapsed | Max RSS |
| --- | ---: | ---: | ---: |
| Instrumented workspace run | 0 | `36:03.71` | 836,172 KB |
| Default LCOV report | 0 | `0:03.30` | 161,720 KB |
| Default JSON report | 0 | `0:03.99` | 252,704 KB |
| Corrected LCOV report | 0 | `0:03.28` | 164,080 KB |
| Corrected JSON report | 0 | `0:04.23` | 383,660 KB |
| CRAP report | 0 | `0:01.13` | 214,120 KB |

The corrected coverage JSON contains 109,878 instrumented lines, 96,512
covered, for 87.835599% workspace line coverage.

## Artifact Integrity

| Durable artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `start/start.lcov` | 4,547,312 | `1003a2ae914c81a13785296609d19a16d8d881d338db478ad6220a5ac01414d1` |
| `start/start.json` | 19,975,828 | `b9c2bbb6c67c037471fae91696d18051808797959a74d9284bf945eaa0c9a0eb` |
| `start/start-crap.json` | 2,957,096 | `958a69aa841ea7632c9a5956adc8c17b8258376bddcaf127407d0dbc47fda137` |
| `start/start-production-over30.json` | 2,656 | `9501960825bc75401c8bb98c2ccf353fe128f5f5baaee103a32737e00306bb93` |

The reports, exact package list, run/report/CRAP logs, and all
`/usr/bin/time -v` records are durable under `low/start/`. The CRAP JSON is the
numeric CRAP authority.

## Instrumented-Failure Attribution

The run command exited zero under `--ignore-run-fail`, while its underlying
test execution reported four failures:

- `h2637_active_fails_closed_without_routing_coefficients`;
- `h2637_active_and_disable_are_mutually_exclusive`;
- `h2637_active_and_shadow_are_mutually_exclusive`; and
- `r3c_lane_transfer_span_projects_multilane_topology`, whose process-global
  audit-count assertion observed two instead of one at
  `direct_runtime_r3c_r4b.rs:779`.

The first three are the unchanged parallel shared-environment H2637 selector
family. The fourth is the unchanged parallel audit-counter family. Their test
source SHA-256 values remain
`e6a8b65fe0fe02951a0751fbc6c187dbd75bec189ac049e5a39deea917edcd9d`
and
`9117d2ff4e0a0d9ecc5f30ae1fe1dfd2aecee28574fbe3dea5aed034a9ddaf7c`,
respectively. Neither source changed between the authoritative Medium final
coverage source and this Low start commit, and neither family belongs to the
Low target census. The Medium final ordinary full profile passed all 1,930
tests, independently distinguishing this known instrumented-concurrency class
from a product regression. No failure was rerun.

`boundary_case.json: FAILED` and
`compat_quoted_header_9002_policy_first.sol: FAILED` are intentional checksum
and drift-guard subprocess output inside passing tests, not failed tests.

## Census And Transition Reconciliation

The exact production filter yields 13 deduplicated rows across 12 module paths.
The complete rows are preserved in `start/start-production-over30.json`: five
input-parser error displays; two legacy-bridge error displays; one meteorology
error display; three runner snowbench rows across two modules; one simulation-
contract registry error display; and one watershed network-frame error display.

The Low-start filtered artifact is byte-identical to Medium final and has the
same SHA-256,
`9501960825bc75401c8bb98c2ccf353fe128f5f5baaee103a32737e00306bb93`.
Therefore the Medium-to-Low transition reconciles at exactly 13 rows and 12
modules, with zero added, removed, or changed row. No source-related coverage
or CRAP drift is present at tranche start.

Disposition: Low start measurement and predecessor reconciliation `PASS`.
