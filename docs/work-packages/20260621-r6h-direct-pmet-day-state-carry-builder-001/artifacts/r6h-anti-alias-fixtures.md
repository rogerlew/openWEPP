# R6H Anti-Alias Fixtures

Status: queued.

| Fixture/test | Alias risk | Required distinction | Result |
|---|---|---|---|
| Interleaved PMET carry test | Day `n+1` uses stale precomputed PMET operands | Expected value differs when prior-day committed direct state is changed. | Queued |
| WAT parity regression | R6G residual hidden by writer self-consistency | WAT parity must be compared after direct artifact production. | Queued |
| Multi-OFE/lane fixture | Day-global input aliases lane-specific state | Two lanes/OFE values diverge and direct publication preserves the distinction. | Queued |
| WAT id authority fixture | Fixture-only `wepp_id = 1` masks wrong id semantics | Rejected identity candidates differ. | Queued |
| CLI fail-closed contract | Partial outputs written while R6H gate fails | Cutover exits nonzero and writes no partial outputs unless all gates pass. | Queued |
