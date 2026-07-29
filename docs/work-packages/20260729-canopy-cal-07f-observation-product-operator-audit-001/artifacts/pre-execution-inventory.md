# CAL-07F Pre-Execution Inventory

Evidence class: `Static`

Starting commit: `643381ed2b42b0378d661b0deb0f04d2dbef7ef9`

Expected fixed inventories:

- two observation products: `gcc_mean` and `gcc_90`;
- two years: 2024 and 2025;
- two directions: rising and falling;
- three source levels: T10, T25, and T50;
- 12 rows per product;
- 37 frozen BASE ensemble members;
- 888 product/member/event/level comparison rows; and
- no parameter fitting or source-role change.

Daily curves come from the retained CAL-07 Data Record 4 file. Transition
source rows come from the checksum-bound CAL-07E input subset. Model crossings
come from the validated CAL-07D inventory. Seasonal windows are product/year
specific and split halfway between falling T10 and rising T10 so intra-annual
recovery crossings cannot masquerade as the opposite seasonal event.
