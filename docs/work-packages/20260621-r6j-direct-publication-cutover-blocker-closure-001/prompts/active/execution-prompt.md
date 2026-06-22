# R6J Execution Prompt

Execute
`docs/work-packages/20260621-r6j-direct-publication-cutover-blocker-closure-001/package.md`
end-to-end in `/home/workdir/openWEPP`.

This is a Defect-Closure ExecPlan. Closure requires real R6 direct publication
cutover: HBP, WAT, PASS, loss, and manifest outputs must be written from typed
direct projection only, with required parity, reconstruction, no-compatibility,
default-disabled, endpoint/RSS, review, verification, and line-count gates. Do
not stop after wiring the manifest writer if another in-envelope blocker
appears; record it in the blocker ledger and continue the R6J loop.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for R6J
correctness review, no-compatibility proof review, gate-evidence audit, and
line-count governance review. Expected outputs are compact Markdown findings
summarized into `artifacts/review-disposition.md` and
`artifacts/verification.md`; subagents may not edit files.

Start by reproducing the R6I handoff state: current-fixture HBP/WAT identity
green, no R6G/R6H marker, and `DirectPublicationFrameCutover` fail-closed at
`manifest direct projection is not wired to the production manifest writer`.
Then close that blocker and keep iterating until
`COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER` or a legitimate
`HOLD-R6J-<SPECIFIC-BOUNDARY>` is proven under the package's HOLD checklist.
