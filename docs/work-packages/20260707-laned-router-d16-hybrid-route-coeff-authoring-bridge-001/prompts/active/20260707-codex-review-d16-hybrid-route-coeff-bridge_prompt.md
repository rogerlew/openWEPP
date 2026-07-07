# Codex Review Prompt: D16 Hybrid Route-Coefficient Authoring Bridge

You are reviewing
`docs/work-packages/20260707-laned-router-d16-hybrid-route-coeff-authoring-bridge-001/`.

Subagent authorization: this package explicitly authorizes
spawning/delegation to review and verification subagents for source-authority
review, bridge-legitimacy review, package gate verification, and bounded
codebase questions. Expected outputs are package-local
`artifacts/review-*.md` and `artifacts/verification-*.md`. Write access is
read-only.

Review stance: adversarial authority review. Findings must lead, ordered by
severity, with file:line references where applicable. Label evidence as
`Static:` or `Ran:`. A package that invents route coefficients or silently
turns legacy compatibility fields into Papanicolaou operands is a NO-GO.

Required questions:

1. Does the package correctly execute the two allowed authority paths:
   source-authored native `ow-lanuse-1` inputs or a contract-first
   legacy-to-native bridge?
2. Does any artifact overclaim that current external roots contain runnable
   active inputs or source-authored coefficients?
3. Does the bridge audit correctly reject row/ridge/random-roughness/residue
   inference under `LANUSE-AUTH-3` and the D11 evidence?
4. Is the active missing-coefficients guard direct enough for this package's
   negative evidence?
5. Are all `BLOCKED` / `NOT RUN` gates legitimate for a hold disposition?
6. Does the worker handoff name a concrete first action rather than another
   diagnostic-only scan?

Write the result to a package-local `artifacts/review-*.md` file.
