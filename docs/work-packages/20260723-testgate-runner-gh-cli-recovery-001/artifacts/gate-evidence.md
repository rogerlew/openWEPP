# Gate Evidence

Ran: push run `29979508839` stopped before TESTGATE gate execution because the trusted runner image had no `gh` executable. No unchanged expensive gate ran.

Ran: the reviewed image build produced candidate `sha256:8a551a87d0784a74be1a76452beb1e4e6726cc36135722020e20a042e04bae84` (4,272,183,439 bytes). Its build verified the official GitHub CLI 2.96.0 archive digest `83d5c2ccad5498f58bf6368acb1ab32588cf43ab3a4b1c301bf36328b1c8bd60`. The candidate is not recorded as installed until live activation evidence exists.

Ran: direct image execution reported `gh version 2.96.0`; `/usr/local/bin/gh` is root-owned mode 0755; an authenticated read-only `gh api` call returned `rogerlew/openWEPP`.

Ran: at the exact tree committed as `e82f1e46c0bf03aa7fb1e6596cdad987b71f49cb`, focused validation passed: shell syntax; Rust formatting; all 9 tests in `testgate_ci_executor_contract`, including rejection of version-suffix drift; scoped Markdown lint with 10 files and 0 findings; and `git diff --check`. Independent Review B repeated the exact-HEAD integration target and observed 9/9 PASS.
