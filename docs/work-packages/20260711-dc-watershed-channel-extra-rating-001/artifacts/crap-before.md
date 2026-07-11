# CRAP before

Status: complete
Evidence mode: Ran

Deduplicated target rows requiring disposition:

| Function | CC | Coverage | CRAP | Disposition |
| --- | ---: | ---: | ---: | --- |
| `WatershedChannelParseError::fmt` | 11 | 0% | 132.000 | cover stable typed display surface |
| `ChannelWarningCode::as_str` | 6 | 0% | 42.000 | cover stable code/display mapping |
| `parse_channel_block` | 36 | 96.429% | 36.059 | decompose only after safety-net closure |

All other target rows were at or below 30. Raw evidence is
`crap-before.json`.
