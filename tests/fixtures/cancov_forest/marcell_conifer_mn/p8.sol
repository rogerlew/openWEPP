9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 21:55:52.637388
# Source Data: Surgo
# 
# Mukey: 3026530
# Major Component: 27572171 (comppct_r = 60.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 82398599   A              10.0    92.0        0.0         0.0          1.6     4.0    85.0     5.0     2.0
# 82398600   E              58.0    92.0        0.0         0.0         1.65     6.0    92.0     8.0     0.5
# 82398601   E and Bt          94.0    92.0        0.0         0.0         1.75     6.0    92.0     8.0     0.5
# 82398602   C             200.0    92.0        0.0         0.0          1.6     1.0    96.0     5.0     0.3
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
# 82398599::wilt_pt estimated from wfifteenbar_r and rock
# 82398599::field_cap estimated from wthirdbar_r and rock
# 82398600::wilt_pt estimated from wfifteenbar_r and rock
# 82398600::field_cap estimated from wthirdbar_r and rock
# 82398601::wilt_pt estimated from wfifteenbar_r and rock
# 82398601::field_cap estimated from wthirdbar_r and rock
# 82398602::wilt_pt estimated from wfifteenbar_r and rock
# 82398602::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-06-26 21:55:54.763079
# Source File: :/wc1/runs/ju/juvenile-separatist/soils/3026530.sol
# 
# Replacements
# --------------------------
# luse -> forest
# stext -> sand loam
# ki -> 400000
# kr -> 8.00E-05
# shcrit -> 2
# avke -> 60
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
0	 'forest'	 'sand loam'	 1.5 	 0.3
'Eagleview and Menahga soils, 1 to 8 percent slopes'	 'LS'	 5	 0.3	 0.75	 400000	 8e-05	 2
	100.0	 1.6	 60	 10.0	 0.1609	 0.062	 85.0	 4.0	 2.0	 3.7	 8.0	 0.05208	 0.3505	 0.02733	 1.968	 161.3	 0.05296	 0.1061
	200.0	 1.65	 60	 1.0	 0.1239	 0.0533	 92.0	 6.0	 0.5	 4.7	 8.0	 0.0544	 0.3447	 0.03059	 2.431	 290.5	 0.05444	 0.08131
	580.0	 1.65	 331.2	 1.0	 0.1239	 0.0533	 92.0	 6.0	 0.5	 4.7	 8.0	 0.0544	 0.3447	 0.03059	 2.431	 290.5	 0.05444	 0.08131
	940.0	 1.75	 331.2	 1.0	 0.1258	 0.0559	 92.0	 6.0	 0.5	 4.7	 7.0	 0.05311	 0.3193	 0.03146	 2.324	 197.5	 0.05319	 0.08339
	2000.0	 1.6	 331.2	 1.0	 0.063	 0.0109	 96.0	 1.0	 0.3	 0.9	 8.0	 0.04714	 0.3498	 0.03563	 3.411	 827.9	 0.04714	 0.05525
1 10000.0 0.01
