# Coverage After

Ran during the rejected attempt:

```text
CARGO_TARGET_DIR=/tmp/openwepp-cqr-20260711-t01-target cargo llvm-cov \
  -p openwepp-runner --bin openwepp-cli-totalwatsed3 \
  --test totalwatsed3_cli_contract --lcov \
  --output-path /tmp/openwepp-cqr-20260711-t01-after.lcov
```

The suite passed `6/6`. Target line coverage is `719/1048` (`68.607%`), up
from the like-for-like scaffold result `667/992` (`67.238%`). LCOV SHA-256:
`527364e35d19cdfe0742a2db042ae49b5469800426de8ce4febe3618926f4165`.
The percentage non-regressed, but independent review correctly found it cannot
replace region evidence, per-function floors, or the cover-first prerequisite.
The source is rolled back; this artifact is attempt evidence only.
