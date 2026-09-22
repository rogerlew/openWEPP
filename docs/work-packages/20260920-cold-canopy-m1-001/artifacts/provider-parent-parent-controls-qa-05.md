# Provider-parent expected-red controls QA — 05

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static review of the 760-entry frozen cut, aggregate SHA-256 `f7a99af6e8980694a183eb181b8ec13b3cc1e31a8f89d0343d8cc3db54bb88b3`; reviewed control SHA-256 `9720e73b3361cfe8fd6bd911a58aa83587b45471ff58cb73ec33dcec561c4a51`. No body, compilation, test execution, or physical work occurred.

## Findings

No blocking regression from the parent-controls04 release.

The added assertions strengthen the existing controls without weakening their typed receipt, resource/material, all-30-record, or caller-observation coverage:

- The positive path now requires the exact typed `NoLiveParent(ParentNotFinalizable)` error from a second commit and the exact `VEG-E-143` receiver/duplicate-transfer refusal when replaying a consumed capability.
- The late path establishes support zero as an actual accepted prefix and directly checks the live clock’s single slab receipt and accepted end before attempting injected supports 1–29.
- A late failure now verifies the typed late-receiver code/stage/reason, the attempted transaction identity, and concrete positive terminal-receiver contributors with finite positive tile mass and matching transaction identity. It preserves the actual caller snapshot and requires the consumed-capability replay refusal.

## Non-blocking follow-up

The raw accessor/implementation wiring, compilation, test execution, and all runtime/physical gates remain unreviewed and unrun. They require their own frozen implementation cut and evidence.

## QA disposition

**PASS — retain parent-controls04 test-body release with these strengthened assertions.** This remains a test-only contract release, not body or launch clearance.
