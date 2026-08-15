# Active Kickoff Prompt

Execute `20260814-nix-agent-devbox-feasibility-001` in order from its recorded
branch and base commit. Preserve the current agent's work on `forest`: do not
inspect or mutate its checkout, target directory, caches, temporary files, or
Git state, and do not run a benchmark or competing heavy workload until its
current work has landed and the host is confirmed idle. Implement the pinned
Nix development shell and collision-safe local agent layout on `ow-dev-01`
first. Freeze one exact landed comparison commit and one `flake.lock` for both
machines. Admit timing evidence only when correctness and identity match.

Execution mode: local, sequential infrastructure implementation with bounded
read-only host inspection.

Autonomy: continue through safe local phases; stop before any action that would
interrupt another agent, push remote state, retire `forest`, or expand into
production/science changes.

Subagent authorization: none. Do not spawn or delegate under this prompt.
