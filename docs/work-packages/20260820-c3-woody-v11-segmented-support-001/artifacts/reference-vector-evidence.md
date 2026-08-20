# Reference Vector Evidence

Status: authority amendment candidate / 54 cases PASS

`reference_calculator.py` imports neither production Rust nor subprocesses. It
independently executes 13 migration and 41 chronology cases. Migration covers
ties-to-even, adjacent binary64 values, exact bit roundtrip, non-finite values,
and the u128 range. Chronology covers forcing-order separation; 1 ns edges;
start/interior/end events and zero remainder; participant and slab poison;
ordered water/NH4/NO3 custody and overbooking; scheduled-once; event/slab
replay; before/after-event equivalent restart; consecutive parents; atomic
commit, abort rollback and publication delay.

The three-slab nonassociative control separately reconstructs sequential staged
subtraction and the ordered `+0.0` cumulative fold for water, NH4, and NO3. All
three valid sequential endings differ in bits from regrouped
`parent_beginning - cumulative`. A wrong-regrouped-ending vector and an
explicit regrouped-owner alias both reject with `VEG-E-124`; existing separate
water/NH4/NO3 overbooking poisons remain executable.

The actual-shaped two-occupancy water control keeps four occupancy debit
receipts distinct from two shared hydrology-owner transitions. Occupancy
post-use values deliberately differ from shared layer endings; only transition
`8.0 -> 6.5` chains across slabs. Independent poisons reject an occupancy
identity alias, a missing debit link, a forged owner-candidate transition, and
authorization overbooking against the staged shared beginning.

Resource sums use ordinary ordered binary64 addition from exact `+0.0`, reject
nonfinite inputs/intermediates, and remain separate for water/NH4/NO3. The
state recurrence makes swapped forcing
supports observably different. Publication digests cover accepted values only.
Closed schemas expose authenticated canonical JSON rather than accepting an
unauditable physical-state/configuration blob. The contract test rejects a
return to opaque V10 payload fields and checks all required alias case IDs.

Full V10 constitutive compatibility is not approximated in Python. The
implementation gate executes V10 and V11 directly and applies the fail-closed
recursive projection inventory in `full-support-compatibility-ledger.md`.
