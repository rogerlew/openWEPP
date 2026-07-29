# CAL-07D Intent And Validation Plan

Evidence class: `Static`

Implementation intent: diagnostic-only calibration-readiness investigation.

The package will not alter executable production behavior or canonical science
authority. It will independently reconstruct committed CAL-07C Beza evidence,
evaluate frozen observation-operator alternatives, and run attribution-only
indicator counterfactuals.

Risk classification: bounded diagnostic. The only executable additions are
package-local analysis, plotting, and validation scripts.

Required validation:

- dependency and source identity;
- exact GSI reconstruction;
- crossing and event inventory;
- assumption labeling;
- output finiteness and manifest integrity;
- figure rendering and sidecar completeness;
- Markdown and Python source quality; and
- terminal exact-diff reconciliation.
