# Final Disposition

Status: `HOLD / Child-3 runtime implementation complete but terminal authority gate unresolved`

The package was reopened after commit `96c46c88e01e4faaecccd084e402ebb6dcb1e6cd`
for a bounded implementation correction. Forest-litter conductivity now uses
the immutable beginning hydrology-owned litter store under the canonical
`0.1 + 0.03*W_l/(rho_w*dz_l)` equation instead of aliasing the top-soil
conductivity. Per-tile LSE VIS/NIR ground optics are the sole E01--E03
lower-boundary owner; the single vegetation forcing albedo pair is neither a
heterogeneous-tile restriction nor a covered-ground optics input. Focused
runtime gates and fresh independent science review pass. This disposition
remains HOLD on the separate oracle-reconciliation package and the required
clean full-workspace gate.

The historical custody HOLD below remains immutable evidence of the dependency
that blocked the first execution attempt. Commit `a7d692da4` lifted that
dependency, and the existing Child-3 package resumed. All accepted material
Child-3 Rust and science findings were remediated, the strict default-off public
endpoint and seven required nonzero benchmark surfaces passed, and the final
science review returned PASS.

The exact hold and rejected aliases are documented in
`real-hydrology-surface-liquid-hold-audit.md` and independently confirmed in
`review_real_hydrology_surface_liquid_hold.md`. A bare-mineral-soil-only
release is prohibited by the campaign objective, so no runtime completion,
benchmark, terminal verification or real-consumer claim is made.

Terminal closure is withheld for one newly discovered load-bearing authority
contradiction. The immutable frozen V3/V5 vegetation fixtures do not regenerate
from their checked-in independent calculators. Isolated regeneration proves the
repository is not mutated, but it also proves exact byte inequality. Python
3.12 and Python 3.11 Nix environments regenerate the same divergent V3 SHA
`7e64d63729b538ff5721ded768eb62be4be195a7903464a2ac7a3ab2083bff00`,
while frozen authority requires
`1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109`.
Propagating that change would migrate the complete V3--V8 identity chain and is
not authorized by this Child-3 runtime package. The full workspace gate therefore
remains FAIL/HOLD: 2,990 tests ran, 2,974 passed, 16 failed. That raw run also
contains environment/tooling and stale historical-test failures beyond the
authority mismatch; it was not rerun and none is relabeled PASS here. Workspace
Clippy, doctests, cargo-deny, formatting and diff hygiene pass with preserved
comparator logs.

Because the required full-workspace correctness gate is not green, the active
kickoff prompt is not archived and the package does not claim the authorized
`COMPLETE` disposition.

Production selectors, defaults, execution, state and outputs remain unchanged.
No activation, cutover, publication, calibration or empirical-validation claim
is included.
