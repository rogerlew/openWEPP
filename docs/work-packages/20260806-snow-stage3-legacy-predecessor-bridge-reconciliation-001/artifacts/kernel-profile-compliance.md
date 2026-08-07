# Kernel Profile Compliance

Status: `NOT_APPLICABLE to mandatory v130 correction`.

Evidence class: `Static`.

The v130 diff changes evidence-custody governance only. It changes no production
kernel behavior or runtime projection controlling a kernel branch, so the
kernel-process profile is not triggered. Invariant, obligations, forcing hashes,
units/tolerances, governance guard, test vectors, Binding Exposure, and claim
limits are nevertheless explicit. Any later Rust observability amendment must
re-evaluate the profile on its exact diff.
