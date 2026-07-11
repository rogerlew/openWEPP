# Numeric Equivalence

Static: production Rust is byte-identical to scaffold `02f43b43`, so numeric
expressions, accumulation order, fallible-read order, and output semantics are
unchanged.

Ran: the same-day multi-OFE test independently reconstructs weighted runoff
depth `6.5` mm and volume `2.6` m3 from distinct Area/Q operands within `1e-12`;
QOFE is deliberately `7.5` mm and outlet-only lateral flow is `0.3` mm.
Existing internal weighted aggregation tests also remain green.
