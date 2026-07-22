# Implementation

Static: scaffold commit `0c11a7b9` predates the test edit. The only code change
updates the searched execution boundary from obsolete
`let mut execution = execute_nodes_for(` to canonical
`let execution = execute_nodes_for(`. The final execution-context lookup and
its strict ordering assertion are unchanged. No production source changed.
