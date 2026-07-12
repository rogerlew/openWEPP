# Hourly Wave-Routing Lineage

Pinned `wshchr.for` authority defines time-zero separation, `it=1..ntchr`,
lateral averaging, segment count, KW/static/variable MC dispatch, outlet
epsilon and terminal storage. The refactor extracts only grid validation,
initial-state/qref, spatial-grid and terminal-storage stages. Recurrence
arithmetic remains ordered exactly.

Returned state feeds WS10 publication. The W11C runner integration consumes
static/variable water, storage, peak and sediment outputs across seven
scenarios; private tests are supporting evidence only.

