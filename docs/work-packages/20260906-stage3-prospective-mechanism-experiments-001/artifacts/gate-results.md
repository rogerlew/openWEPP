# Gate results (active)

Ran: offline toolchain preflight PASS, rustc1.95.0. Comparator role available.

| Command/surface | Result | Evidence |
|---|---|---|
| runner cargo check --tests initial | FAIL: JSON macro recursion | raw/common-check.log |
| check after record split | FAIL: test-support feature missing | raw/common-check-02.log |
| check after dev feature edge | PASS | raw/common-check-03.log |
| prospective F contract test | PASS 1 | raw/authority-F-test.log |
| prospective R contract test | PASS 1 | raw/authority-R-test.log |
| LSE observer tests initial | PASS | raw/authority-LSE-observer.log |
| carrier observer tests initial | PASS | raw/authority-orchestrator-observer.log |
| clock/source correction check | FAIL: forbidden unsafe FFI | raw/common-check-04.log |

Corrective response: safe existing rustix time API via test-only dependency;
no production unsafe exception. Clock/observer correction tests pending.
Early raw duration_ms fields are invalid wrapper metadata, explicitly excluded.
No performance, real consumer parity, full critical correctness or terminal
qualification is claimed from these checks. Mandatory remaining gates are active.
