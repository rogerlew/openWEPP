# R6H WAT ID Authority

Status: held.

R6G used `DIRECT_WAT_WEPP_ID = 1` for the inherited current fixture. R6H must
prove or correct direct WAT id semantics before claiming WAT publication
cutover.

| Question | Required evidence | Result |
|---|---|---|
| Is the WAT `wepp_id` file/output-family id, hillslope id, or another legacy publication id? | Contract, pinned-baseline source, output-format authority, or explicit decision. | Current code preserves compatibility parity with `DIRECT_WAT_WEPP_ID = 1`; broader semantic authority remains follow-on. |
| Does the direct WAT builder handle non-trivial OFE/lane cases without aliasing WB13 identity? | Focused fixture/test where rejected identity candidates differ. | Lane `ofe_id` is direct `lane_id`; multi-lane direct executor test proves input dimensionality, but no full multi-OFE WAT parquet fixture was closed. |
| Does metadata agree with the accepted WAT id semantics? | Row/schema/metadata parity or documented canonical authority. | Current fixture parity preserved; broader metadata authority remains follow-on. |

## Rejected Closure

Current-fixture equality alone is not sufficient WAT id authority for R6H.

## Disposition

R6H did not change WAT `wepp_id` semantics. The active cutover blocker now
fails earlier on `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`; after that is
closed, the next WAT-id closure still needs non-trivial row/schema/metadata
evidence before WAT cutover can be called complete.
