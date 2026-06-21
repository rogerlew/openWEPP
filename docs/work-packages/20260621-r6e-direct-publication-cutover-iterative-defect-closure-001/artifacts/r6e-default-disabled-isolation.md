# R6E Default-Disabled Isolation

Evidence mode: Static + Ran.

Status: pass for R6E delta.

R6E did not activate direct publication by default and did not change the
compatibility runtime-selection path.

Evidence:

- `DirectPublicationFrameCutover` remains controlled by the explicit
  `--direct-publication-frame-cutover` flag;
- `build_retained_direct_publication_frame` still returns `Ok(None)` unless
  runtime selection is `DirectPublicationFrameCutover`;
- focused CLI cutover test confirms opt-in failure writes no public outputs;
- direct-publication helper split is behavior-preserving and does not introduce
  default-path construction.

Full protected output identity for compatibility mode remains a closure gate for
the eventual successful cutover package, not for this held R6E marker
refinement.
