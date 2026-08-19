# Restart-authority HOLD intake

Evidence class: `Static + Ran`

Starting commit is `bb3cc3a0ed...` (`bb3cc3a0e` as the requested nine-character
abbreviation), branch `main`, four commits ahead of `origin/main` at
`1cac432a4a5d2a0de87122bd68b69ab83cffe21a`. The worktree and `git diff
--check` were clean. No pull, reset, rebase, push, branch, activation, selector,
default, deployment, or output change is authorized.

The literal intake command comparing `git rev-parse --short=10` with the
nine-character string `bb3cc3a0e` necessarily fails; the actual ten-character
prefix is `bb3cc3a0ed`. Full ancestry proves the intended checkpoint.

Implementation intent is restart-authority remediation only. Production
restart implementation remains blocked until exact-current authority review
and dual terminal verification pass.

