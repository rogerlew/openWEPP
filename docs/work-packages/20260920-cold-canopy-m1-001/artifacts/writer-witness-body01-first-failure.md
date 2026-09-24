# First focused failure

The first `writer-witness-body01` focused run exited `100`: 15 of 16 controls
passed and `m1_stage2_negative_humidity_excess_never_admits_or_materializes`
failed because its first governed-excess proposal installed an ordinary update
while the replacement opportunity was still open. The controller now permits
ordinary strict-decrease installation at a root only after the first valid
replacement has consumed that opportunity. The final rerun exits `0` with all
16 controls passing.
