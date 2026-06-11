# Gate Results

Evidence: Static + Ran
Date: 2026-06-10

| Gate | Command / method | Result | Notes |
|---|---|---|---|
| Binding exposure lint | `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | pass-deferred | Exit `0`; 27 rows, 27 science-review follow-ons. |
| Strict consolidation lint | `python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | expected fail | Exit `1`; strict mode rejects deferred rows. |
| Diff boundary | `git diff -- docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | pass | Additive BEI section only. |
| Kernel/runtime gates | not run | not applicable | Package made no kernel/runtime edits. |
| Cargo gates | not run | not applicable | Documentation-only package; no Rust write set. |
