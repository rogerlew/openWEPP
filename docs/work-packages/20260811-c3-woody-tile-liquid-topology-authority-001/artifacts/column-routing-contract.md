# Column Routing Contract

Status: `selected`

Evidence mode: `Static`

For each tile, sort occupancies top-to-bottom by validated rank. Begin with the
tile-local top rain. At each occupancy, E04 consumes the local incident amount;
free throughfall and both drainage terms become lower-canopy incident liquid.
Stemflow bypasses lower foliage to the same tile's ground recipient. The lowest
occupancy's remaining throughfall and drainage reach that tile's ground.
Nothing crosses tile identity and no stand aggregation precedes terminal routing.
