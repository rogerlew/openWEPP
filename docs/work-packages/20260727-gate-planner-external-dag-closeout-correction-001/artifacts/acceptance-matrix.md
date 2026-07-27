# Acceptance Matrix

Status: `SCAFFOLD REVIEW REQUIRED`

Evidence class: `Static`

| ID | Obligation | Acceptance evidence |
|---|---|---|
| DC-01 | Capabilities verify and consume exactly once | Red/green Generation-B transition test |
| DC-02 | Audit inventory reconstructs exactly once | Instrumented adversarial test and review |
| DC-03 | Ledger admits exactly once | Instrumented adversarial test and review |
| DC-04 | Package authority is prospective | Canonical package-chain PASS from scaffold |
| DC-05 | Focused behavior is stable | Nextest, Clippy, CAL Python, hygiene gates |
| DC-06 | Critical correctness is current | Canonically admitted exact-head full workspace and anti-evasion receipts |
| DC-07 | Harvard remains sealed | Static custody review and absence of opening token |
| DC-08 | Closure is independently accepted | Dual review and dual terminal verification |
