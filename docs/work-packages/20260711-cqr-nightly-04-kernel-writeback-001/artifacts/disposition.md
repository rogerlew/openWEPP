# Finding Disposition

Status: CLOSED.

Accepted findings were fixed:

- exact accept/apply message IDs were added;
- range/lower/upper/invalid-bounds check IDs, kinds, message IDs, and ordering
  were asserted;
- bounded NaN and both infinities now prove mixed-violation non-finite priority;
- clippy `similar_names` and `float_cmp` findings were corrected;
- the long indexed evaluation test was split into three focused tests;
- test counts, current hashes, focused counts, and round references were
  reconciled in artifacts.

No review or verification finding remains. Round-3 closure gates all passed.
