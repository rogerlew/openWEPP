9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 21:55:52.493815
# Source Data: Surgo
# 
# Mukey: 398439
# Major Component: 27572006 (comppct_r = 87.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 82398366   A               8.0    92.0        0.0         0.0          1.6     4.0    85.0     5.0     2.0
# 82398367   Bw             43.0    92.0        0.0         0.0         1.65     4.0    86.0     4.0     0.5
# 82398368   C             200.0    92.0        0.0         0.0          1.7     1.0    96.0     5.0    0.25
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
# 82398366::wilt_pt estimated from wfifteenbar_r and rock
# 82398366::field_cap estimated from wthirdbar_r and rock
# 82398367::wilt_pt estimated from wfifteenbar_r and rock
# 82398367::field_cap estimated from wthirdbar_r and rock
# 82398368::wilt_pt estimated from wfifteenbar_r and rock
# 82398368::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-06-26 21:55:55.042715
# Source File: :/wc1/runs/ju/juvenile-separatist/soils/398439.sol
# 
# Replacements
# --------------------------
# luse -> deciduous forest
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
0	 'deciduous forest'	 'sand loam'	 1.5 	 0.3
'Menahga loamy sand, 15 to 30 percent slopes'	 'LS'	 4	 0.23	 0.75	 400000	 8e-05	 2
	80.0	 1.6	 60	 10.0	 0.1609	 0.062	 85.0	 4.0	 2.0	 11.3	 8.0	 0.05208	 0.3505	 0.02733	 1.968	 161.3	 0.05296	 0.1061
	200.0	 1.65	 60	 10.0	 0.1304	 0.0391	 86.0	 4.0	 0.5	 11.3	 8.0	 0.05135	 0.3388	 0.02855	 1.995	 151.0	 0.05204	 0.1022
	430.0	 1.65	 331.2	 10.0	 0.1304	 0.0391	 86.0	 4.0	 0.5	 11.3	 8.0	 0.05135	 0.3388	 0.02855	 1.995	 151.0	 0.05204	 0.1022
	2000.0	 1.7	 331.2	 1.0	 0.0641	 0.012	 96.0	 1.0	 0.25	 0.9	 8.0	 0.04588	 0.3249	 0.03689	 3.278	 608.5	 0.04588	 0.05512
1 10000.0 0.01
