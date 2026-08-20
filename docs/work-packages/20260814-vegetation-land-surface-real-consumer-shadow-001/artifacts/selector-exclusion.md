# Selector And Publication Exclusion

Status: `PASS / terminal A+B confirmed no activation surface`

- The V9 real-consumer operation is an explicit method requiring a caller-owned
  `DirectV9RealConsumerShadow`; it has no default construction.
- `openwepp-runner` contains no reference to the operation or V9 shadow type.
- No environment variable, model selector, runtime default, output row, or
  production commit path names the attachment.
- Successful execution returns the ordinary production report and rows; shadow
  receipts remain accessible only through the explicitly supplied attachment.
- Failure discards the cloned production and complete shadow candidates.

This is default-off execution evidence only. It makes no activation,
publication, cutover, deployment, calibration, or empirical-validation claim.
