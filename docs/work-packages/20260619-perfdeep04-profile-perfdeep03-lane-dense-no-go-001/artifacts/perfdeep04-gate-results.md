# PERFDEEP04 Gate Results

Evidence class: Ran.

## Tool Gates

Ran:

```text
perf --version
cat /proc/sys/kernel/perf_event_paranoid
```

Result:

```text
perf version 6.8.12
0
```

Ran:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: passed.

## Profiling Gates

| Gate | Result | Evidence |
|---|---|---|
| Opt-in PERFDEEP03 H2637 profile captured | PASS | `1164.31 s`, `519160 KB`, `61248` samples |
| Default-disabled comparison profile captured | PASS | `704.82 s`, `320640 KB`, `37051` samples |
| Lost samples acceptable | PASS | `0` lost samples in both profiles |
| User-space symbols resolved | PASS | Rust symbols resolved in `perf report` |
| Kernel symbols resolved | WARN | `/proc/kallsyms` restricted; not needed for package conclusion |
| Top cost centers ranked | PASS | `perfdeep04-profile-results.md` |
| Next package boundary named | PASS | `perfdeep04-next-package-recommendation.md` |

## Raw Artifact Inventory

Repository text artifacts:

```text
raw/perfdeep04-h2637-optin-perf-record.log
raw/perfdeep04-h2637-optin-perf-record-attempt2.log
raw/perfdeep04-h2637-optin-header.txt
raw/perfdeep04-h2637-optin-flat-top.txt
raw/perfdeep04-h2637-optin-children-top.txt
raw/perfdeep04-h2637-optin-children-report.txt
raw/perfdeep04-h2637-default-perf-record.log
raw/perfdeep04-h2637-default-header.txt
raw/perfdeep04-h2637-default-flat-top.txt
raw/perfdeep04-h2637-default-children-top.txt
raw/perfdeep04-h2637-default-children-report.txt
```

Large binary profile data is intentionally outside the repository:

```text
493M /tmp/perfdeep04/profile/perfdeep04-h2637-optin.data
299M /tmp/perfdeep04/profile/perfdeep04-h2637-default.data
```

## Markdown Gate

Ran:

```text
markdown-doc lint \
  --path docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001 \
  --path docs/work-packages/README.md \
  --path docs/ROADMAP.md \
  --path docs/architecture/array-native-runtime-specification.md \
  --format plain
```

Result:

```text
10 files validated, 0 errors, 0 warnings
```
