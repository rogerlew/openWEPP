# Implementation

`BoundaryClass::as_str` and `classification` now delegate to one private
`BoundaryClassDefinition` mapping pairing each unchanged public string with its
classification. The public API and every mapping remain exact; no status policy
or typed error behavior changed.
