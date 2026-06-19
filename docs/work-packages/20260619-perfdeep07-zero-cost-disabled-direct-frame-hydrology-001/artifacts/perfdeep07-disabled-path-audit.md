# PERFDEEP07 Disabled-Path Audit

Status: queued.
Evidence mode: not-run.

## Audit Target

Identify dense-first, indexed, direct-frame, hot-symbol, shadow, or
compatibility objects currently constructed or resolved when all PERFDEEP
opt-ins are disabled.

This audit must run before direct-frame implementation work. If the audit finds
default-disabled dense/direct-frame compatibility work, execution must either
remove/bypass it or stop before adding more fast-path plumbing.
