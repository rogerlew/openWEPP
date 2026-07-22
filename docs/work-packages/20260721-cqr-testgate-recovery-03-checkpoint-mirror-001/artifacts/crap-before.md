# CRAP Before

Static: the two actionable rows are retained in `target-selection.md`. Exact
affected baseline measurement reproduced both rows at clean `02745e8d`:

| Function | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: |
| `mirror_node_checkpoint` | 22 | 16.3265% | 305.5363 |
| `create_absolute_directories` | 11 | 0% | 132 |

`mirror_error` is the only other production row and scores 2.
