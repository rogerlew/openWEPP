# Intake

Evidence class: `Ran` and `Static`

- Frozen base: `0873bdae960f7f8c76401845acb476750fdd020e`.
- Initial branch state: clean `main`, synchronized with `origin/main`.
- Queue authority: TESTGATE-PLAN-01 is `next` in `docs/ROADMAP.md`.
- Predecessor: TESTGATE-ALIGN-01 completed v1 schemas and explicitly handed off
  planner/verifier implementation in shadow mode.
- Security posture: fail closed; no secrets, credentials, network trust,
  executable confinement, protected refs, or publication mutation enter scope.
- Testing posture: focused loops plus one stable-tree conservative terminal
  closure set; redundant broad reruns are prohibited.
