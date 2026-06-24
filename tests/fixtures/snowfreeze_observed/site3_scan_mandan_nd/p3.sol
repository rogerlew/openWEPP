9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-24 20:15:21.906740
# Source Data: Surgo
# 
# Mukey: 2699220
# Major Component: 27700241 (comppct_r = 49.0)
# Texture: silt loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 82864237   Ap             18.0    9.17        0.0         0.0         1.39    22.0    21.0    12.0     3.5
# 82864238   Bw1            29.0    9.17        0.0         0.0         1.31    25.0    20.0    14.5     2.5
# 82864239   Bw2            61.0    9.17        0.0         0.0         1.38    25.0    23.0    17.0     1.0
# 82864240   2Bk           110.0    4.23        0.0         1.0         1.49    31.0    36.0    16.7     0.6
# 82864241   2C            200.0    9.17        0.0         0.0         1.48    31.0    38.0    12.5     0.3
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
# 82864237::wilt_pt estimated from wfifteenbar_r and rock
# 82864237::field_cap estimated from wthirdbar_r and rock
# 82864238::wilt_pt estimated from wfifteenbar_r and rock
# 82864238::field_cap estimated from wthirdbar_r and rock
# 82864239::wilt_pt estimated from wfifteenbar_r and rock
# 82864239::field_cap estimated from wthirdbar_r and rock
# 82864240::wilt_pt estimated from wfifteenbar_r and rock
# 82864240::field_cap estimated from wthirdbar_r and rock
# 82864241::wilt_pt estimated from wfifteenbar_r and rock
# 82864241::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-06-24 20:15:23.211918
# Source File: :/wc1/runs/fo/forced-bop/soils/2699220.sol
# 
# Replacements
# --------------------------
# luse -> tall grass
# stext -> silt loam
# ki -> 1000000
# kr -> 8.00E-05
# shcrit -> 1.5
# avke -> 20
# bd ->
# ksflag -> 1
# ksatadj -> 0
# ksatfac -> 1.5
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.4
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
0	 'tall grass'	 'silt loam'	 1.5 	 0.3
'Temvik-Wilton-Williams silt loams, 3 to 6 percent slopes'	 'SIL'	 6	 0.16	 0.75	 1000000	 8e-05	 1.5
	180.0	 1.39	 20	 10.0	 0.309	 0.167	 21.0	 22.0	 3.5	 19.0	 0.0	 0.0902	 0.4152	 0.004579	 1.454	 14.47	 0.1377	 0.2499
	200.0	 1.31	 20	 10.0	 0.305	 0.16	 20.0	 25.0	 2.5	 21.0	 0.0	 0.09658	 0.4397	 0.004455	 1.449	 18.64	 0.1485	 0.2623
	290.0	 1.31	 33.012	 10.0	 0.305	 0.16	 20.0	 25.0	 2.5	 21.0	 0.0	 0.09658	 0.4397	 0.004455	 1.449	 18.64	 0.1485	 0.2623
	610.0	 1.38	 33.012	 1.0	 0.298	 0.15	 23.0	 25.0	 1.0	 20.3	 0.0	 0.09565	 0.4209	 0.004997	 1.427	 13.23	 0.1471	 0.2627
	1100.0	 1.49	 15.228	 1.0	 0.3468	 0.2121	 36.0	 31.0	 0.6	 22.0	 10.9	 0.1034	 0.4032	 0.008164	 1.333	 7.933	 0.1638	 0.2852
	2000.0	 1.48	 33.012	 1.0	 0.3422	 0.2056	 38.0	 31.0	 0.3	 22.1	 10.0	 0.1033	 0.4065	 0.008555	 1.332	 8.766	 0.1638	 0.2861
1 10000.0 0.01
