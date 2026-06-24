9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-24 20:27:46.985748
# Source Data: Surgo
# 
# Mukey: 486088
# Major Component: 26405497 (comppct_r = 30.0)
# Texture: loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 78786775   A              13.0    9.17        0.0         4.0         1.45    21.0    41.6    12.0     3.0
# 78786774   Bt             74.0    2.82        2.0        13.0         1.35    31.5    35.3    10.6    1.25
# 78786773   R              99.04.619861490641729         -           -        1.5196     7.0    66.8    10.0     7.0
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: -
# ksat: -
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
# 78786775::wilt_pt estimated from wfifteenbar_r and rock
# 78786775::field_cap estimated from wthirdbar_r and rock
# 78786774::wilt_pt estimated from wfifteenbar_r and rock
# 78786774::field_cap estimated from wthirdbar_r and rock
# 78786773::using default rock content of 55.5%
# 78786773::ksat_r estimated from rosetta2
# 78786773::wilt_pt estimated from rosetta2
# 78786773::field_cap estimated from rosetta2
# 78786773::bd estimated from sand, vfs, and clay
# res_lyr_i None
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
# Build Date: 2026-06-24 20:27:47.542469
# Source File: :/wc1/runs/fe/feline-wrangler/soils/486088.sol
# 
# Replacements
# --------------------------
# luse -> shrub
# stext -> loam
# ki -> 1000000
# kr -> 5.00E-05
# shcrit -> 1
# avke -> 35
# bd ->
# ksflag -> 1
# ksatadj -> 0
# ksatfac -> 1.5
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.5
# xmxlai -> 5
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
0	 'shrub'	 'loam'	 1.5 	 0.3
'Vitale-Itca-Rubble land complex, 2 to 60 percent slopes'	 'GR-L'	 4	 0.16	 0.75	 1000000	 5e-05	 1
	130.0	 1.45	 35	 10.0	 0.3258	 0.1723	 41.6	 21.0	 3.0	 17.5	 47.2	 0.08686	 0.3945	 0.007993	 1.394	 13.28	 0.1335	 0.2519
	200.0	 1.35	 35	 1.0	 0.324	 0.192	 35.3	 31.5	 1.25	 25.0	 59.2	 0.1055	 0.4401	 0.007738	 1.357	 15.37	 0.1666	 0.2921
	740.0	 1.35	 10.152	 1.0	 0.324	 0.192	 35.3	 31.5	 1.25	 25.0	 59.2	 0.1055	 0.4401	 0.007738	 1.357	 15.37	 0.1666	 0.2921
	1000.0	 1.52	 16.6315	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
1 10000.0 0.01
