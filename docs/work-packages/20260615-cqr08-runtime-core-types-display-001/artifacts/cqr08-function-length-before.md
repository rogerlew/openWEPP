# Function Length Before

Static: before refactor, `00_core_types.rs` had 1004 lines.

Static: before refactor, the local suppression census found:

- `pub enum HillslopeRuntimeInputError` at line 37.
- `pub const fn code(&self)` at line 319.
- `#[allow(clippy::too_many_lines)]` at line 390.
- `fn fmt(&self, ...)` at line 391.

Static: `HillslopeRuntimeInputError::fmt` spanned lines 391 through 1001 in the
pre-refactor file and required the local `too_many_lines` suppression.

Ran: baseline CRAP rows confirmed the concentrated complexity:

| Function | Line | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `HillslopeRuntimeInputError::code` | 319 | 65.0 | 40.298507462686565 | 964.0467577461321 |
| `HillslopeRuntimeInputError::fmt` | 391 | 65.0 | 0.0 | 4290.0 |

Disposition: pre-refactor target required decomposition.
