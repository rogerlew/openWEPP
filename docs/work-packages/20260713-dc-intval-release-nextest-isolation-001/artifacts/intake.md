# Intake

Status: `PASS`

Evidence class: **Ran + Static**. Pinned-input and exact-release output is
archived; interactive focused output is supporting and was not archived.

- clean start commit: `1a6a03494745e77e352c3c1c9ab190d6fb0746a7`;
- `/workdir/wepp-forest` commit:
  `375ccc296ed1ea491f599ff1b1a25b415d494a2a`;
- cohort SHA-256:
  `42b7d827d842ecbe75843175a80ab4f67a097784156658df8fb849161eb98958`;
- watchlist SHA-256:
  `42214345a228d27a0536b771dd73068dc897d369f54cb8a197457dea675e26ab`;
- applicable guidance: root `AGENTS.md`, `tests/AGENTS.md`, and
  `docs/work-packages/AGENTS.md`;
- protected production, physics, contracts, fixtures, thresholds, selectors,
  authority/stability lanes, and skip behavior remained outside the write set.

Static pre/post comparison proves the regression discriminates the stale and
corrected commands. Interactive focused runs reported the expected red gate,
post-fix 1/1, and three H2637 guards 3/3, but those raw outputs were not
archived and do not carry closure. The archived exact release log supplies
terminal run evidence: full nextest passed 1,945/1,945 with no H2637 collision.
No retry, serial workaround, ignored selector guard, or skip flag was used.
