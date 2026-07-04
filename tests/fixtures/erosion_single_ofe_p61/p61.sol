9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-03-30 22:35:55.127380
# Source Data: Surgo
# 
# Mukey: 2686279
# Major Component: 22461396 (comppct_r = 60.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 66401498   A              30.0    28.0        0.0         9.0         1.25    14.0    66.8    10.6     3.0
# 66401499   Bw             46.0    28.0        0.0         0.0         1.35    14.0    66.8    10.6    0.75
# 66401500   BC             66.0    92.0        0.0        36.0         1.25    12.0    83.7     9.9    0.25
# 66401501   Cr            102.016.631501366310225         -           -        1.5196     7.0    66.8    10.0     7.0
# 66401497   R             152.016.631501366310225         -           -        1.5196     7.0    66.8    10.0     7.0
# 
# Restricting Layer:
# ksat threshold: 2.00000
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
# 66401498::wilt_pt estimated from wfifteenbar_r and rock
# 66401498::field_cap estimated from wthirdbar_r and rock
# 66401499::wilt_pt estimated from wfifteenbar_r and rock
# 66401499::field_cap estimated from wthirdbar_r and rock
# 66401500::wilt_pt estimated from wfifteenbar_r and rock
# 66401500::field_cap estimated from wthirdbar_r and rock
# 66401501::using default rock content of 55.5%
# 66401501::ksat_r estimated from rosetta2
# 66401501::wilt_pt estimated from rosetta2
# 66401501::field_cap estimated from rosetta2
# 66401501::bd estimated from sand, vfs, and clay
# 66401497::using default rock content of 55.5%
# 66401497::ksat_r estimated from rosetta2
# 66401497::wilt_pt estimated from rosetta2
# 66401497::field_cap estimated from rosetta2
# 66401497::bd estimated from sand, vfs, and clay
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
# Build Date: 2026-03-30 22:36:22.089293
# Source File: :/wc1/runs/as/assisted-weakness/soils/2686279.sol
# 
# Replacements
# --------------------------
# luse -> forest low sev fire
# stext -> sand loam
# ki -> 400001
# kr -> 0.00012
# shcrit -> 2
# avke -> 20
# ksflag -> 0
# ksatadj -> 0
# ksatfac -> 1.3
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.3
# xmxlai -> 4
# keffflag -> 1
# lkeff -> 10
# plant.data.decfct ->
# plant.data.dropfc ->
# 
# h0_min_depth = None
# h0_max_om = None
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::modify_initial_sat(initial_sat=0.75)
# wepppy.wepp.soils.utils.WeppSoilUtil::clip_soil_depth(max_depth=2000)
Any comments:
1 0
0	 'forest low sev fire'	 'sand loam'	 1.3 	 0.3
'Ultic Haploxerolls-Rock outcrop complex, dissected stream breaklands'	 'SL'	 6	 0.09	 0.75	 400001	 0.00012	 2
	200.0	 1.25	 20	 10.0	 0.226	 0.121	 66.8	 14.0	 3.0	 13.6	 19.01	 0.07717	 0.4424	 0.01367	 1.473	 83.03	 0.1066	 0.224
	300.0	 1.25	 100.8	 10.0	 0.226	 0.121	 66.8	 14.0	 3.0	 13.6	 19.01	 0.07717	 0.4424	 0.01367	 1.473	 83.03	 0.1066	 0.224
	460.0	 1.35	 100.8	 10.0	 0.1945	 0.0934	 66.8	 14.0	 0.75	 11.7	 9.0	 0.07501	 0.4186	 0.01452	 1.472	 60.06	 0.102	 0.2179
	660.0	 1.25	 331.2	 1.0	 0.202	 0.084	 83.7	 12.0	 0.25	 8.5	 58.4	 0.07348	 0.4529	 0.02043	 1.695	 219.9	 0.08057	 0.1694
	1020.0	 1.52	 59.8734	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
	1600.0	 1.52	 59.8734	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
1 10000.0 0.01
