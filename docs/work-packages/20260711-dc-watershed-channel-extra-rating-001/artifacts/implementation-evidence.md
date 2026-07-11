# Implementation evidence

Status: complete
Evidence mode: Static and Ran

The parser now shares ordinary channel/rating semantic validation with a
side-effect-free, memoized suffix probe. Canonical retained closure wins; exact
E006 is returned only when a full-domain immediate candidate after
`icntrl!=4` fails retained closure and its single deletion closes all remaining
declared blocks plus EOF. Neither falls through to the untouched ordinary
error; enabled-rating duplicates remain generic E002. No input is repaired or
partially returned.

After the 38-test safety net closed, ordered enum, parameter, and effective-
control phases were extracted from `parse_channel_block` without changing
parse/guard/warning or floating-point order. Focused parser 38/38 and WSHED-W5
20/20 pass. The real network frame projects exact optional rating values for an
enabled record and `None` for a disabled record; production frame code was
already correct and unchanged.
