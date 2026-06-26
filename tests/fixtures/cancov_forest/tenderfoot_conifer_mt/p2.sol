9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 20:32:40.967802
# Source Data: Surgo
# 
# Mukey: 2604436
# Major Component: 26932714 (comppct_r = 35.0)
# Texture: loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80406653   Oe     X        4.0    30.0        0.0         0.0          0.2     7.0    66.8    10.0    70.0
# 80406648   A              16.0    10.0        0.0        36.0         1.23    25.0    38.0    11.4     7.0
# 80406652   E              37.0     3.0        0.0        32.0         1.42    29.0    34.0    10.2     2.0
# 80406650   Btg1   R       63.0     1.0        0.0        33.0          1.4    42.0    28.4     8.1     1.0
# 80406651   Btg2           87.0     3.0        0.0        38.0         1.43    38.0    32.0     8.8     1.0
# 80406649   BCg           150.0     3.0        0.0        53.0          1.4    31.0    35.0    10.6     1.0
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: N/A
# ksat (um/s): 1.00000
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
# 80406648::wilt_pt estimated from wfifteenbar_r and rock
# 80406648::field_cap estimated from wthirdbar_r and rock
# 80406652::wilt_pt estimated from wfifteenbar_r and rock
# 80406652::field_cap estimated from wthirdbar_r and rock
# 80406650::wilt_pt estimated from wfifteenbar_r and rock
# 80406650::field_cap estimated from wthirdbar_r and rock
# 80406651::wilt_pt estimated from wfifteenbar_r and rock
# 80406651::field_cap estimated from wthirdbar_r and rock
# 80406649::wilt_pt estimated from wfifteenbar_r and rock
# 80406649::field_cap estimated from wthirdbar_r and rock
# res_lyr_i 3
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
# Build Date: 2026-06-26 20:32:41.417042
# Source File: :/wc1/runs/as/askance-regularity/soils/2604436.sol
# 
# Replacements
# --------------------------
# luse -> forest
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
# xmxlai -> 14
# keffflag -> 0
# lkeff -> -9999
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
# 
# h0_min_depth = None
# h0_max_om = None
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::modify_initial_sat(initial_sat=0.75)
Any comments:
1 1
0	 'forest'	 'loam'	 1.5 	 0.3
'Jefflake-Yellowmule-Lonniebee, very stony families, complex, 0 to 8 percent slopes'	 'CBV-L'	 3	 0.16	 0.75	 400000	 3e-05	 1
	160.0	 1.23	 50	 10.0	 0.382	 0.232	 38.0	 25.0	 7.0	 21.4	 60.96	 0.09709	 0.4556	 0.006773	 1.408	 29.95	 0.1515	 0.2727
	200.0	 1.42	 50	 10.0	 0.332	 0.2	 34.0	 29.0	 2.0	 23.3	 64.64	 0.1012	 0.4163	 0.007278	 1.363	 10.83	 0.1584	 0.2804
	400.0	 1.42	 10.8	 10.0	 0.332	 0.2	 34.0	 29.0	 2.0	 23.3	 64.64	 0.1012	 0.4163	 0.007278	 1.363	 10.83	 0.1584	 0.2804
1 10000.0 0.0036
