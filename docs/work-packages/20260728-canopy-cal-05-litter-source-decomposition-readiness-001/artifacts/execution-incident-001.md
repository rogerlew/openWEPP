# Execution Incident 001

Evidence class: `Ran: fail-closed pre-publication diagnostic`

The first Harvard source-native diagnostic attempt stopped at plot A1 and
wrote no diagnostic output. The frozen `1e-9 g C m^-2 yr^-1` closure guard
rejected 24 of 28 retained plot-mean rows. Independent inspection found a
maximum absolute residual of `1.6166666227945825e-8 g C m^-2 yr^-1` between
serialized `foliar + pooled_nonfoliar` and serialized total.

The source values, units, classes, plots, periods, and interpretation remain
unchanged. Before rerun, the numeric serialization guard is amended to
`1e-7 g C m^-2 yr^-1`, over six times the observed maximum. Independent
prospective reviewers must approve this narrow amendment. The failed attempt
is retained and is not a scientific result.
