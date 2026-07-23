# Gate Evidence

Ran: push run `29979508839` stopped before TESTGATE gate execution because the trusted runner image had no `gh` executable. No unchanged expensive gate ran.

Ran: `bash tools/ci/omarchy-runner/manage.sh build-image` produced image `sha256:a0dbc987aa4ea42041e1148739a04ee8b2ce805e38d0197c16d3f4545baf7f6d` (4,272,175,247 bytes). Its build verified the official GitHub CLI 2.96.0 archive digest `83d5c2ccad5498f58bf6368acb1ab32588cf43ab3a4b1c301bf36328b1c8bd60`.

Ran: direct image execution reported `gh version 2.96.0 (2026-07-02)`.

Ran: focused validation passed: shell syntax; Rust formatting; all 8 tests in `testgate_ci_executor_contract`; scoped Markdown lint with 11 files, 0 findings; and `git diff --check`.
