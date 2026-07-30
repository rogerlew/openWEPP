# Final Disposition

Disposition: `COMPLETE / PASS`.

Evidence class: Ran + Static.

The package resolves the requested stand-scale science:

- use a complementary hemispherical sky/canopy partition;
- use effective canopy and snow emissivity of exactly one for this candidate;
- use the exact Dilley clear-sky plus Unsworth-Monteith cloud route;
- use air temperature only as a named homogeneous-stand effective-canopy
  approximation with stable-night and gap/edge claim limits; and
- exclude explicit trunk physics from the first hillslope stand candidate.

Successor admission is intentionally split:

- `EB-02 canonical-contract research/amendment`: `GO_WITH_PREREQUISITES`;
- `EB-02 runtime implementation`: `HOLD`.

Runtime implementation remains held until the canonical contract binds a typed
deterministic canopy-state-to-sky-view operator, reconciles or replaces the
legacy cloud mapping, and selects one active snow-surface-temperature
provider. The sky-view operator must derive from existing canopy cover, LAI,
structural cover, and height only where scientifically defined. It may not
require another user coefficient or remote-sensing dataset, and it may not
simply relabel canopy cover or LAI as sky view.

No additional article is needed for this decision. Lawler and Link (2011) or
the full Pomeroy et al. (2009) paper would become useful only if the campaign
expands into discontinuous-canopy, gap-edge, or explicit-trunk physics.

Dual corrected-tree review and dual terminal verification pass, including the
derived-sky-view amendment re-review and amendment-aware terminal rerun. This
package changes no executable behavior or canonical science authority.
