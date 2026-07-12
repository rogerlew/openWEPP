# CRAP After

Evidence class: **Ran**

`interpolate_unit_discharge` fell from 56 to 7. Other unique target-file rows:
conservation residual 1; bin integration 8.042; single-OFE 1; traced single-OFE
6.017; cascade 11.000. Every target-file row is at most 30 and no target-file
row regressed above its High-A start value. CRAP JSON SHA-256 is
`ad7e56d5b6953099cb0323e5db35045d289b720d7f988289471a3048d7da0210`.

Authoritative final workspace target rows also pass: conservation residual 1;
interpolation 7; bin integration 8.042; point sampler 3; single-OFE 1; traced
single-OFE 6.000; cascade 11.000. The fixed production ranking moved from 67
rows/45 modules to 66/44 and removed HA-01. However, the first final capture
lowered `execute_direct_publication_stream` coverage from 54.545% to 53.535%
and raised its CRAP from 44.141 to 45.991 due to the known selector-suite
parallel interference, so that capture was rejected. Same-source retry 1
restored the exact High-A start failure set and passes the ratchet: 66 rows/44
modules, HA-01 absent, and all 66 non-target keys exactly unchanged. Retry LCOV
SHA-256 is
`5e143335d8e3bb410d208ddad502b905db675556812faf4121aa47196a4bc8ae`;
CRAP SHA-256 is
`74468bd2fb75ad625caf43fb69b4014905a8680bbf94ff0bd27187b2966b9e1b`;
filtered ranking SHA-256 is
`e831466be9ba5ce0744dcdca25f5d318d1439cb55bc086dc445f0885acc8d312`.
