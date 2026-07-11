# Contract-test implementation

Status: complete
Evidence mode: Ran

Invalid fixture `compat_nchnum_clamped.chaninp` remains raw 99 plus two IDs and
now requires exact `CHN-E-002`. Paired valid fixture
`compat_nchnum_raw_closed.chaninp` contains 99 IDs and requires raw 99/list-99,
normalized 2/first-two, plus `CHN-W-004`. WSHED-W5 requires raw observability
while `routing_globals.nchnum` consumes normalized 2.

Ran: parser focused suite 21 total: 19 pass, 2 intended fail. Existing invalid
vector was incorrectly accepted with `nchnum_input=2`; valid 99/99 defaulted to
zero instead of preserving raw. Consumer focused suite 19 total: 18 pass, 1
intended fail (`nchnum_input` observed 0 vs 99). Production parser and consumer
had empty git diffs when recorded.

After the production correction and coverage completion, the parser suite
passes 35/35 and the WSHED-W5 consumer suite passes 19/19. The intended-red
cases now prove exact `CHN-E-002`, raw 99/list-99 preservation, normalized
2/first-two projection, tail diagnostics, and the real consumer's normalized
count.
