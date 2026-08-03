# Review Finding Disposition

Status: `COMPLETE / BOTH RE-REVIEWS PASS`

The initial reviews correctly held the package for two classes of issue:

- realized-`ssrd`-weighted cloud was outcome-dependent and could not serve as
  independent primary evidence; and
- comparator chronology/completeness, ERA5 cloud-domain checks, and the direct
  documentation/hygiene gates needed explicit executable or recorded proof.

Disposition: accepted and remediated. The authoritative primary statistic is
now the unweighted 24-hour arithmetic mean of ERA5 `tcc`; realized-shortwave
weighting is sensitivity-only and excluded from the sanity conclusion. The
comparison tool now fails closed on duplicate or non-monotonic comparator
timestamps, incomplete common days, nonfinite or out-of-domain `tcc`, and
nonzero local-midnight boundary weights. Direct syntax, JSON, Markdown, diff,
protected-path, and bytecode checks are recorded. Both fresh exact-current
reviews pass with no remaining finding.
