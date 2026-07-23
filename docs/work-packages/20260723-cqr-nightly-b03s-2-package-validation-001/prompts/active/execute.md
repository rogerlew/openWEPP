# CQR B03S-2 Package Validation Prompt

Execution mode: execute this package end-to-end.

Autonomy: continue through characterization, decomposition, metrics, review,
verification, and disposition without intervention; stop only at a declared
hold boundary.

Required-reading budget: Core is root/work-package/CQR governance plus this
package. Conditional is the target source and its embedded history fixtures.
On-demand is adjacent Git/planner code needed by a specific branch.

Subagent authorization: this prompt explicitly authorizes the review,
verification, and comparator delegation declared in `package.md`. Do not push,
deploy, switch branches, manually dispatch TESTGATE, or rerun unchanged heavy
evidence.
