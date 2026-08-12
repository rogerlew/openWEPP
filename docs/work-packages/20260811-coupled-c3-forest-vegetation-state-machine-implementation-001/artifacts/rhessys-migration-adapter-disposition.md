# RHESSys Migration Adapter Disposition

Status: `FROZEN`

Implement a versioned offline adapter inside `openwepp-vegetation::migration`. Its raw input vocabulary is separate from canonical serde fields. Output records source path/hash, mapping version, per-field provenance, canonical hash when complete, and an exhaustive ordered unresolved-field list. Caller supplements are explicit; neither RHESSys values nor sentinel spellings become runtime defaults or aliases.
