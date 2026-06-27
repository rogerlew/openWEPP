9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 22:04:02.974848
# Source Data: Surgo
# 
# Mukey: 278707
# Major Component: 27355373 (comppct_r = 61.0)
# Texture: loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 81661563   Oe     X        3.0    55.0        0.0         0.0         0.26     7.0    66.8    10.0    80.0
# 81661568   A              13.0    10.0        2.0         2.0         0.86     6.0    47.0    13.0    13.0
# 81661567   E              15.0    10.0        1.0         1.0         1.49     3.0    58.0    24.0     4.0
# 81661565   Bs1            18.0    10.0        2.0         4.0          1.0     6.0    53.0    14.0     7.0
# 81661571   Bs2            33.0    10.0        2.0         4.0         1.12     3.0    53.0    14.0     4.0
# 81661569   Bs3            46.0    10.0        2.0         4.0         1.12     1.0    55.0    22.0     4.0
# 81661564   BC             54.0    10.0        1.0         3.0          1.5     1.0    58.0    20.0     1.0
# 81661570   Cd1    R       94.0  1.0001        1.0         2.0         1.76     1.0    58.0    20.0     0.5
# 81661566   Cd2           165.0  1.0001        1.0         2.0         1.76     1.0    58.0    20.0     0.5
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: Densic material
# ksat (um/s): 1.00010
# 
# defaults applied to missing chorizon data:
# sandtotal_r  ->      66.800
# claytotal_r  ->       7.000
# om_r         ->       7.000
# cec7_r       ->      11.300
# sandvf_r     ->      10.000
# smr          ->      55.500
# 
# Build Notes:
# 81661568::wilt_pt estimated from wfifteenbar_r and rock
# 81661568::field_cap estimated from wthirdbar_r and rock
# 81661567::wilt_pt estimated from wfifteenbar_r and rock
# 81661567::field_cap estimated from wthirdbar_r and rock
# 81661565::wilt_pt estimated from wfifteenbar_r and rock
# 81661565::field_cap estimated from wthirdbar_r and rock
# 81661571::wilt_pt estimated from wfifteenbar_r and rock
# 81661571::field_cap estimated from wthirdbar_r and rock
# 81661569::wilt_pt estimated from wfifteenbar_r and rock
# 81661569::field_cap estimated from wthirdbar_r and rock
# 81661564::wilt_pt estimated from wfifteenbar_r and rock
# 81661564::field_cap estimated from wthirdbar_r and rock
# 81661570::wilt_pt estimated from wfifteenbar_r and rock
# 81661570::field_cap estimated from wthirdbar_r and rock
# 81661566::wilt_pt estimated from wfifteenbar_r and rock
# 81661566::field_cap estimated from wthirdbar_r and rock
# res_lyr_i 7
# 
# THIS FILE AND THE CONTAINED DATA IS PROVIDED BY THE UNIVERSITY OF IDAHO
# 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
# TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
# PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL UNIVERSITY OF IDAHO
# BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
# CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
# SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
# INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHERE IN
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
# ARISING IN ANY WAY OUT OF THE USE OF THIS FILE, EVEN IF ADVISED OF THE
# POSSIBILITY OF SUCH DAMAGE.
# 
# 
# If you change the original contexts of this file please
# indicate it by putting an 'X' in the box here -> [ ]
# 
# 
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::9002.0migration
# Build Date: 2026-06-26 22:04:04.595495
# Source File: :/wc1/runs/un/undescended-conserve/soils/278707.sol
# 
# Replacements
# --------------------------
# luse -> deciduous forest
# stext -> loam
# ki -> 400000
# kr -> 3.00E-05
# shcrit -> 1
# avke -> 50
# bd ->
# ksflag -> 1
# ksatadj -> 0
# ksatfac -> 1.5
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 2
# xmxlai -> 5
# keffflag -> 0
# lkeff -> -9999
# plant.data.decfct -> 0.2
# plant.data.dropfc -> 0.2
# 
# h0_min_depth = None
# h0_max_om = None
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::modify_initial_sat(initial_sat=0.75)
Any comments:
1 1
0	 'deciduous forest'	 'loam'	 1.5 	 0.3
'Peru-Marlow association, 3 to 15 percent slopes, extremely stony'	 'SIL'	 7	 0.04	 0.75	 400000	 3e-05	 1
	130.0	 0.86	 50	 10.0	 0.2427	 0.1442	 47.0	 6.0	 13.0	 11.3	 12.64	 0.07103	 0.5035	 0.004623	 1.557	 283.4	 0.1119	 0.2037
	150.0	 1.49	 50	 10.0	 0.1815	 0.0816	 58.0	 3.0	 4.0	 11.3	 6.9	 0.05296	 0.3597	 0.01301	 1.527	 49.49	 0.07197	 0.1729
	180.0	 1.0	 50	 10.0	 0.2049	 0.1019	 53.0	 6.0	 7.0	 11.3	 10.7	 0.06709	 0.4687	 0.006486	 1.538	 180.5	 0.1013	 0.2008
	200.0	 1.12	 50	 10.0	 0.1646	 0.0627	 53.0	 3.0	 4.0	 11.3	 10.7	 0.05915	 0.433	 0.007203	 1.568	 143.0	 0.08533	 0.181
	330.0	 1.12	 36.0	 10.0	 0.1646	 0.0627	 53.0	 3.0	 4.0	 11.3	 10.7	 0.05915	 0.433	 0.007203	 1.568	 143.0	 0.08533	 0.181
	460.0	 1.12	 36.0	 10.0	 0.1523	 0.0526	 55.0	 1.0	 4.0	 11.3	 10.7	 0.05601	 0.4295	 0.007657	 1.585	 163.5	 0.0793	 0.1726
	600.0	 1.5	 36.0	 1.0	 0.102	 0.0228	 58.0	 1.0	 1.0	 11.3	 7.84	 0.04998	 0.3546	 0.01322	 1.548	 55.39	 0.06675	 0.1641
1 10000.0 0.0036
