# Parse and Numeric Equivalence

Static: the refactor moves no numeric formula or conversion. Every existing
parse, validation, comparison, token offset, unit-bearing assignment, and
warning/error construction remains in branch order. Compatibility-only nozzle
default and depsrg normalization remain unchanged and are directly asserted.

Ran: the detached pre-decomposition 21-case public parser oracle passed, and
the current suite asserts complete typed diagnostics and parsed behavior across
sprinkler/furrow/strict/compatibility paths.
