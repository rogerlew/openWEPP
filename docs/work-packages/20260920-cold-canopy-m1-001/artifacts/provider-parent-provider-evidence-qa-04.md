# Provider evidence QA — 04

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Ran provider-control receipt and static recovery/custody inspection. This reviewer did not rerun any command.

## Findings

No blocking finding in the narrow source04 provider evidence.

`provider-parent-controls-run-04.json` records exit 0, no timeout, and `8 tests run: 8 passed` (test phase `61.425s`; total `63.040960332378745s`). The receipt records all source, observer-base, binary, pin, and support-link unchanged flags as true. The two emitted identity rows contain distinct cycle payload, run, calendar, first-parent-forcing, and first-GSI-receipt digests; each reports 4,320 records and 144 parents. Their adapter-manifest digest is `2a46507c5c232e08fd88365469e0d3bb84e21aeabad1ce76b8fc1ef23ad8fab3`, and implementation digest is `6671ab7418a0719c2d395a5dfbed02e6ef1b52e4545a7ccaae11d4b9655d582d`.

`provider-parent-binary-preservation-04.json` preserves the source04 test executable under a durable path with SHA-256 `086205641a824e71c3acdf798ee881cb4dec5e3dd4faa82a42018fccc1698fbe`. `provider-parent-recovery-04.json` reconstructs the 759-entry source04 tree from the immutable observer base and reviewed patch, reporting exact match to aggregate SHA-256 `a717cc043b64603c19238642f5db0c1aefc09577ca2d0907dcede15eeaedfdd2`.

## Provider evidence disposition

**PASS — source04 provider admission/control evidence is established.** This supports the private provider’s two-cycle payload, receipt, mutation, canonical encoding, and actual-caller initialization/refusal controls only.

Complete-parent staging/commit, receiver evidence, positive late rejection, conservation/balance, both fixed 72-hour cycles, restart, cost, and final qualification remain **HOLD**. This result does not waive any of them.

## Required final quality commands and bounds

Use the existing package and gate strategy, with exact frozen source/support/argv receipts at the time each command is selected:

1. `cargo fmt --all -- --check`.
2. Affected and reverse-dependent `cargo clippy --all-targets -- -D warnings`.
3. Affected doctests and the package’s focused test-inclusive command: `cargo nextest run -p openwepp-vegetation -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator`, plus the separately inventoried orchestrator feature commands and affected named integration targets.
4. Current A0 admission, both contract BEI checks, SC unit compliance, and every applicable A1 hard-invariant and A3 constitutive-authority suite. The stable authority route must name the exact test/command before execution; metadata or prior source checks do not substitute for it.
5. `cargo deny check`, because the detached cut changes workspace dependency resolution through `Cargo.lock`.
6. Critical regression: `cargo nextest run --workspace --profile full`, together with remaining contract, real-consumer, conservation/reconstruction, chronology, restart, serialization, publication, and anti-evasion obligations selected by the actual parent integration diff.

The 180-second cap applies to each physical process. The full-workspace regression can invoke physical tests and is not classified as nonphysical. Independently required longer validation retains its existing declared bound as an explicit exception to that per-process cap; it must still fit the recorder’s full-bound-plus-1800-second reserve rule and must never be relabeled nonphysical. No unbounded or new framework command is authorized by this review.
