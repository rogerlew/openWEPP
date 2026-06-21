# R6H WAT ID Authority

Status: queued.

R6G used `DIRECT_WAT_WEPP_ID = 1` for the inherited current fixture. R6H must
prove or correct direct WAT id semantics before claiming WAT publication
cutover.

| Question | Required evidence | Result |
|---|---|---|
| Is the WAT `wepp_id` file/output-family id, hillslope id, or another legacy publication id? | Contract, pinned-baseline source, output-format authority, or explicit decision. | Queued |
| Does the direct WAT builder handle non-trivial OFE/lane cases without aliasing WB13 identity? | Focused fixture/test where rejected identity candidates differ. | Queued |
| Does metadata agree with the accepted WAT id semantics? | Row/schema/metadata parity or documented canonical authority. | Queued |

## Rejected Closure

Current-fixture equality alone is not sufficient WAT id authority for R6H.
