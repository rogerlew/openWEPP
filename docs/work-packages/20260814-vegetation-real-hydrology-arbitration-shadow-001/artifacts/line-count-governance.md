# Line-Count Governance

Status: `WARN accepted for bounded Child-2 increment / split required before Child 4`

Evidence class: `Static + Ran`

`vegetation_real_hydrology_shadow.rs` is 2,118 lines at the current Child-2
review checkpoint. Production code and both focused test modules currently
share the file so private identity and rollback seams can be exercised without
widening the public API. The production functions independently satisfy strict
Clippy line-count checks; the excess is test volume rather than one oversized
algorithm.

This is an explicit warning, not a claim that the file is an acceptable final
real-consumer architecture. Before Child 4 adds routed multi-OFE coordination,
the tests must move to a sibling test module and the runtime boundary must be
split into snapshot, arbitration, candidate and V7 bridge modules. Child 4 may
not grow this file past the present checkpoint.

The current bounded exception avoids a high-conflict mechanical move during
active correctness review. It does not waive strict Clippy, public API,
duplication, or package terminal-diff review.
