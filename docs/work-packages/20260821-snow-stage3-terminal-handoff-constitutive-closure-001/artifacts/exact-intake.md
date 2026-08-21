# Exact intake

Status: `PASS` for starting identity and clean-tree checks.

Static: The directive requires `85d88fb903b302a33d43304a5001911f13f7d8d5`
on `main`, with `origin/main` at the same commit.

Ran: `git rev-parse HEAD`, `git rev-parse origin/main`,
`git branch --show-current`, `git status --short --branch`, and
`git diff --check` passed on 2026-08-21 before edits.

Static: Historical Child-1/terminal-handoff packages are protected exactly as
listed in `package.md`.

Static: The complete directive is retained at
`prompts/active/20260821-snow-stage3-terminal-handoff-constitutive-closure-001_kickoff_agent_prompt.md`;
its byte count is `48386` and its SHA-256 is
`368072aed3e296be531bd38d262246ceb80777ecf8b5655a98d77221bd1cecdd`.
`cmp -s` against `/tmp/stage-3-v11-instructions.md` passed.
