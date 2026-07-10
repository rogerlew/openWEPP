# Characterization Plan

Before production decomposition, add target-local tests that render every
`DirectRuntimeError` variant and compare the complete output string. This is the
direct downstream consumer of the `Display` match and covers all branches,
including `DirectDayExecutionFailure`'s one-based lane/day formatting.

The tests will also retain existing call-site tests for typed fail-closed
variants. No test will assert a reformatted, normalized, or partial diagnostic.
