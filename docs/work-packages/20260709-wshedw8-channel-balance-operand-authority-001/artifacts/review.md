# Review

Status: `EXECUTED-COMPLETE`
Evidence: `Static + ran`

## Findings

No open correctness findings remain for WSHED-W8.

## Confirmed Properties

- `chanwb` no longer uses watershed runoff as the channel-balance inflow for
  typed publication frames; it reads `WatershedPublicationFrame.channel_inflow_m3`.
- `Balance (m^3)` includes storage and remains null unless inflow, outflow,
  loss, and storage are all present.
- Public `value` remains mapped to watershed runoff for existing dynamic-value
  schemas, preserving the prior non-`chanwb` meaning.
- Runtime publication proves the real dispatch path can carry distinct channel
  inflow and outflow through routed state into `WatershedPublicationFrame`.
- Default typed publication keeps unavailable channel-balance operands null.

## Residual Limitations

- The current direct watershed channel lane publishes `Storage = 0.0` and
  `Loss = 0.0` as explicit routed operands. That is an implementation boundary,
  not a claim that physical channel storage or transmission loss is impossible.
  Future SC-ROUTE transmission-loss/storage work should replace these zero
  terms with nonzero typed operands where authority exists.
- WSHED-W8 does not implement hourly HBP water/sediment consumption; M-T3 owns
  that broader watershed-facing active Lane D consumer path.
