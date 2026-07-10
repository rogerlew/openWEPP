# Implementation

The monolithic `DirectRuntimeError::fmt` match was replaced with a
compiler-exhaustive conversion to private identity, publication/index, and
runtime-guard display representations. Each representation owns a small,
exhaustive `Display` match; the public error type, variants, fields, strings,
format arguments, and one-based day/lane arithmetic remain unchanged.

The prior `clippy::too_many_lines` suppression on the public formatter was
removed. No fallback, default, panic, public API change, threshold change, or
numeric formula was introduced.

Target-local tests render every current `DirectRuntimeError` variant and compare
the complete output string. They exercise the actual public `Display` consumer,
all conversion arms, every private display arm, and the existing one-based
`DirectDayExecutionFailure` projection.
