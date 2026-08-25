# Terminal bounded observation seam V4 validator corrections

Status: `CANDIDATE / NO SOURCE AUTHORITY`

V3 validators remain except:

- iteration validation compares all nine flux fields, including signed vapor;
- transition reconstruction receives both `TerminalFluxEvidence` and the
  selected fourteen-field ledger and recomputes every derived ledger field;
- pair validation requires the retained resulting joint to equal fine-2's
  hydrology-complete ending joint;
- provider validation consumes `ProviderCallEvidence`: ordinals and chronology
  are contiguous, every entry has one outcome, ok calls biject to carriers,
  error calls do not, and the later floor admission adds no call record;
- clock noninterference requires both live `Eq` and post-return complete
  serialization equality.

The exact measured final operands and signed delta from V3 remain authoritative.
All positive and one-field poison cases cover the new flux, pair-joint,
provider-error and clock-byte relations.
