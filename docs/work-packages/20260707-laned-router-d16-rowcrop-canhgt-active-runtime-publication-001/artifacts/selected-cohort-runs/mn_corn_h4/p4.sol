9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-07 04:38:21.100959
# Source Data: Surgo
# 
# Mukey: 2372351
# Major Component: 27566503 (comppct_r = 55.0)
# Texture: silt loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 82379855   A,AB           30.0     9.0        0.0         0.0          1.4    22.5    24.8     9.9     3.0
# 82379856   Bw             76.0     9.0        0.0         5.0         1.45    22.5    22.4     8.9     0.5
# 82379857   2C            200.0   14.11        0.0        38.0         1.58     7.5    47.2    13.2    0.25
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
# 82379855::wilt_pt estimated from wfifteenbar_r and rock
# 82379855::field_cap estimated from wthirdbar_r and rock
# 82379856::wilt_pt estimated from wfifteenbar_r and rock
# 82379856::field_cap estimated from wthirdbar_r and rock
# 82379857::wilt_pt estimated from wfifteenbar_r and rock
# 82379857::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-06-07 04:38:25.437093
# Source File: :/wc1/runs/al/algebraic-radium/soils/2372351.sol
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
'Frontenac-Lacrescent complex, 30 to 70 percent slopes, rocky'	 'SIL'	 4	 0.16	 0.75	 4813575.0	 0.0084	 3.5
	200.0	 1.4	 32.4	 10.0	 0.3189	 0.1697	 24.8	 22.5	 3.0	 19.3	 7.5	 0.09083	 0.4112	 0.005064	 1.437	 13.38	 0.1391	 0.2531
	300.0	 1.4	 32.4	 10.0	 0.3189	 0.1697	 24.8	 22.5	 3.0	 19.3	 7.5	 0.09083	 0.4112	 0.005064	 1.437	 13.38	 0.1391	 0.2531
	760.0	 1.45	 32.4	 1.0	 0.3116	 0.148	 22.4	 22.5	 0.5	 18.0	 16.88	 0.09032	 0.4011	 0.004978	 1.432	 10.64	 0.1385	 0.2522
	2000.0	 1.58	 50.796	 1.0	 0.19	 0.048	 47.2	 7.5	 0.25	 6.5	 65.9	 0.05901	 0.3463	 0.0101	 1.456	 20.53	 0.08813	 0.1952
1 10000.0 0.01
