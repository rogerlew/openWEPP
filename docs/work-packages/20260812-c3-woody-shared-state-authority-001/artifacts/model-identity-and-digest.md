# V4 Model Identity and Digest

Status: `post-release remediation candidate / review pending`

Evidence mode: `Static + Ran`

- Model: `OPENWEPP_C3_WOODY_V4`.
- Contract: `SC-VEGETATION-001@8`.
- Base: immutable V3 digest
  `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`.
- Corrected V4 definition SHA-256:
  `8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`.
- Corrected V4 fixture SHA-256:
  `3072226f1d80359c548d87c1fa222be0c20b01627d9117e39163c39d9eb8824d`.
- Corrected generator SHA-256:
  `422f0a6fb778de73568259b0d1bad19f63e5b6fcac5fd608accace45b316bcd2`.
- Historical pre-remediation release identity: definition `571bac78...`,
  fixture `6862b507...`, generator `5ac8dfea...`.

The definition uses recursively sorted compact JSON and one trailing LF. It
binds the V8 shared-state amendment, live contract sections, adjacent contract
digests, exact shared schema, area equations, migration, fixture, generator,
and typed unsupported branches.

Protected predecessor bytes remain:

- V1 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`;
- V2 `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`;
- V3 `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`.
