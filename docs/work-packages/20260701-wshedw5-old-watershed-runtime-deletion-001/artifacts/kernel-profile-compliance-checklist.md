# Kernel Profile Compliance Checklist

Status: `executed`

Evidence mode: `static + ran`

| Requirement | Result | Evidence |
| --- | --- | --- |
| Contract-first sequencing | `PASS` | No SC amendment required; W5 preserves direct typed physics and deletes carrier code. |
| No surrogate/provisional physics | `PASS` | Direct kernel still calls real WS11/WS12/WS18/WS20 helpers; tests assert active WS12 composition and transport-capacity sensitivity. |
| Typed guard preservation | `PASS` | Channel non-finite, impoundment domain, WS12 projection non-finite, and WS12 projection domain guards covered. |
| No silent defaults | `PASS` | Invalid typed inputs fail closed; no production normalization added. |
| Real consumer proof | `PASS` | Typed frame dispatch records routed state and typed publication consumes it. |
| Old runtime deleted | `PASS` | Source scan and source guard forbid old watershed request/writeback markers. |
| Full Rust gates | `PASS` | `cargo fmt --check`, workspace clippy, full nextest, and `cargo deny check` passed. |
