# Superseded Terminal Plan Diagnostic

Evidence class: `Ran` (superseded)

The shadow CLI generated the terminal plan out of tree so creating evidence did
not change its source subject:

    cargo run -p openwepp-gate-planner --bin openwepp-gate-plan -- plan \
      --repo . \
      --base 0873bdae960f7f8c76401845acb476750fdd020e \
      --stage terminal \
      --predecessor 112a15c4cc5cf28fb634cfc0662497f495b311556797dfa1c72208c7b738a086 \
      --boundary INCREMENT \
      --campaign TESTGATE-PLAN-01 \
      --output /tmp/testgate-plan-01-terminal.json

Result:

- terminal plan ID:
  `607c63df5d97e82888315de1828d5e4d2effad06400f3d93d10fd6b51053b54a`;
- terminal file SHA-256:
  `41118c6be56bc171269c36675f0245b490e76553307bdff331d382eb92de284c`;
- changed objects: 35;
- risk: `CRITICAL`;
- selected nodes: six;
- exact workspace Nextest inventory: 2,114 tests;
- unmapped inputs: 15, all escalated to critical.

`openwepp-gate-plan reconcile` returned `PASS`: six implementation-evidence
paths were added since intent, no intended path disappeared, risk did not
decrease, and no terminal obligation was deferred. The selected terminal DAG
is cargo-deny, cargo-fmt, gate-policy schema consistency, workspace Clippy,
workspace full Nextest, and adjudicated CRAP with stable prerequisites.

This plan predates final authorized-path, tool/environment, and independent
terminal-reconstruction remediation. It is retained as diagnostic provenance,
not current closure evidence. The single conservative sequence in
`gate-results.md` is the terminal execution evidence and stops on CRAP failure.
