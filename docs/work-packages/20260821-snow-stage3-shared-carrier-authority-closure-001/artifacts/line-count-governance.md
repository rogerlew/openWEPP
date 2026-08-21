# Line-count governance

Status: queued

Evidence mode: not-run

No production Rust is touched by this scaffold. At execution and terminal
disposition, run line counts for every touched Rust/test file. Repository policy
is `WARN` at 2,000 lines and `BLOCK` at 3,000 unless a generated/fixture
exception records owner and sunset.
