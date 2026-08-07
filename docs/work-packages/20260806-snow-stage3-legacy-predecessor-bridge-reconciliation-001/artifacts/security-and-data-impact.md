# Security And Data Impact

Status: `prospectively frozen`.

Evidence class: `Static`.

Local source/history and ignored evidence only. Historical inputs are read-only.
Checkpoint builds use isolated shared local clones under the package target,
`--locked --offline`, distinct Cargo targets, and a scrubbed environment.
Subprocesses use explicit argv/workdirs; generated runfiles rewrite all seven
absolute input/output paths into their cell namespace. Execution refuses
overwrite and retains failures. No secrets, network, protected data, external
mutation, `git worktree` metadata, or public export.
