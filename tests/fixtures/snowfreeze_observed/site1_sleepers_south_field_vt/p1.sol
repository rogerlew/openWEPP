9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-24 20:01:56.921471
# Source Data: Surgo
# 
# Mukey: 282872
# Major Component: 27656756 (comppct_r = 83.0)
# Texture: silt loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 82710641   Ap             18.0    10.0        0.0         0.0          0.9     9.0    34.0    10.3     9.1
# 82710640   Bg             33.0    10.0        0.0         1.0          1.5     6.0    60.0    16.1     1.1
# 82710642   Cdg    R      165.0    0.55        1.0         5.0          1.8     7.0    66.0    17.7     0.5
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: Densic material
# ksat (um/s): 0.55000
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
# 82710641::wilt_pt estimated from wfifteenbar_r and rock
# 82710641::field_cap estimated from wthirdbar_r and rock
# 82710640::wilt_pt estimated from wfifteenbar_r and rock
# 82710640::field_cap estimated from wthirdbar_r and rock
# 82710642::wilt_pt estimated from wfifteenbar_r and rock
# 82710642::field_cap estimated from wthirdbar_r and rock
# res_lyr_i 2
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
# Build Date: 2026-06-24 20:01:57.837310
# Source File: :/wc1/runs/ha/hard-bitten-doze/soils/282872.sol
# 
# Replacements
# --------------------------
# luse -> agriculture crops
# stext -> silt loam
# ki ->
# kr ->
# shcrit ->
# avke ->
# bd ->
# ksflag -> 1
# ksatadj -> 0
# ksatfac -> 0
# ksatrec -> 0
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax ->
# xmxlai ->
# keffflag -> 0
# lkeff -> -9999
# plant.data.decfct ->
# plant.data.dropfc ->
# 
# h0_min_depth = None
# h0_max_om = None
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::modify_initial_sat(initial_sat=0.75)
Any comments:
1 1
0	 'agriculture crops'	 'silt loam'	 0 	 0
'Cabot silt loam, 3 to 8 percent slopes'	 'SIL'	 3	 0.09	 0.75	 4706630.0	 0.0051	 2.6576
	180.0	 0.9	 36.0	 10.0	 0.2931	 0.1218	 34.0	 9.0	 9.1	 5.0	 13.0	 0.07582	 0.5026	 0.003439	 1.584	 223.1	 0.1184	 0.2043
	200.0	 1.5	 36.0	 10.0	 0.1587	 0.0588	 60.0	 6.0	 1.1	 3.3	 9.91	 0.05746	 0.3636	 0.01382	 1.498	 41.35	 0.07892	 0.1848
	400.0	 1.5	 36.0	 10.0	 0.1587	 0.0588	 60.0	 6.0	 1.1	 3.3	 9.91	 0.05746	 0.3636	 0.01382	 1.498	 41.35	 0.07892	 0.1848
1 10000.0 0.00198
