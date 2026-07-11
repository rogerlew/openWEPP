# Characterization

Not entered. Existing focused tests pass `14/14`, but they do not bind required
finite-value rejection. Adding characterization for current `NaN` acceptance
would freeze a known contract violation; correcting it changes accepted-input
semantics outside this behavior-preserving CQR package.
