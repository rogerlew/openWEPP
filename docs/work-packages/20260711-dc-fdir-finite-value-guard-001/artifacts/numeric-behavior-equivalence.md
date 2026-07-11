# Numeric and behavior equivalence

Status: PASS for finite inputs; authorized invalid-domain correction isolated
Evidence mode: Static and Ran

The semantic change is restricted to parsed IEEE non-finite spellings, which
now fail at the typed boundary. Finite token conversion, comparisons, event
ordering, warning/default values, and float expression order are unchanged.
The compatibility probe retains the prior finite `value > 2.0` split and
`datver_or_header` syntax failure.

Ran: all 26 pre-existing/expanded finite and structural characterization tests
that passed before production edits continued to pass after correction; the
single intended red finite-domain test also became green, for 27/27 total.
The final strict sprinkler, strict furrow, and compatibility no-datver/nozzle
tests assert equality against exhaustive `FixedDateIrrigationFile` structures,
including every float, record, event, closure flag, warning message/line, and
provenance enum. These durable expected structures, plus static confirmation
that extraction preserved statement/expression order, are the numeric-identity
oracle for accepted finite fixtures. No claim is made about unenumerated input
files beyond the contract properties exercised by the table-driven suite.
