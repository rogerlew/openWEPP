# Implementation evidence

Status: complete
Evidence mode: Static and Ran

The parser now validates record 4 against raw nonnegative `nchnum` before any
normalization. Compatibility mode no longer converts that mismatch into a
defaulted parse. A valid 99-ID fixture preserves `nchnum_input=99`, derives the
bounded normalized topology, and checks every raw ID for diagnostics while the
normalized ID view contains only the consumed prefix.

The network-frame consumer test proves `routing_globals.nchnum` is populated
from `nchnum_norm=2`, while the parsed contract object retains raw value 99,
all 99 source IDs, and the normalized two-ID prefix. Production frame code
already read the normalized count, so no consumer production edit was
necessary. No downstream normalized-ID-list consumption claim is made.

After correction tests were green, `parse_required_branch` was decomposed into
raw-record parsing, conditional cardinality parsing, and ID normalization
helpers without changing validation order or diagnostics.

Focused evidence passed: parser 36/36 and WSHED-W5 19/19. Source SHA-256 is
`f7857a4cbd5a0bdb5f7ade1bf4e2d8871811988791f79dcb77fe5af33b59646d`;
parser test SHA-256 is
`bb7475b308acd1364e7c8037fc4495a321ec1de8d46abb78a0fdf4c62f620c9e`.
