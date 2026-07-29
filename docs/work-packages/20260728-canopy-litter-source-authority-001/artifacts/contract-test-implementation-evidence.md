# Contract-test Implementation Evidence

Status: `PASS`

Evidence class: `Ran`

The contract-derived integration test was authored before production edits.
Its initial red run produced 1 pass and 4 expected failures because
`surface_litter_forcing` was not yet in the schema.

Terminal command:

```text
cargo nextest run --test canopy_litter_external_boundary_contract
```

Result: 16 passed, 0 failed. The terminal suite proves both admitted modes and
rejects source/classification digest drift, interval/derived execution,
non-exhaustive measured daily, malformed canonical bytes, duplicate dates,
negative mass, material/class mismatch, contradictory support, incomplete
drying provenance, site/OFE mismatch, path escape, and numeric payload on an
unrepresented tissue.
