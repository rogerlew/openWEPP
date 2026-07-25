# Fallback Evidence

Evidence class: Ran.

Missing publication/control inputs produced a canonical `INVALID` receipt with
no selection and `collection_launched=false`. The retained receipt could
authorize recollection only after deterministic reinspection reproduced it and
the explicit directive was
`execute cqr nightly for 3 modules`.

Changing the retained reason caused byte-identity reproduction to fail.
Reusing the intake receipt as an authorization output was also rejected.
