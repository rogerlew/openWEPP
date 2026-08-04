# Performance And Storage Evidence

Status: `queued`

Evidence mode: `Not run`

Record scaffold/candidate binary provenance, fixture identity, environment,
warm-up, seven alternating raw wall-time and peak-RSS samples per binary,
medians, percentage deltas, result-carrier/type footprints, copy/allocation
inventory, trace-disabled payload-absence proof, and trace-enabled byte cost.

Closure bounds are no more than 5% median regression for trace-disabled wall
time or peak RSS and no more than 1% trace-enabled byte growth, with exact
diagnosis required for any nonzero schema/content difference.
