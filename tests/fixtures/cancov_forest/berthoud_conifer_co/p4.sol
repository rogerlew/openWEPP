9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 20:41:05.589076
# Source Data: Surgo
# 
# Mukey: 768928
# Major Component: 26663709 (comppct_r = 40.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 79608604   Oi     X        5.0   300.0        0.0         0.0          0.5    15.0    35.0     5.0    85.0
# 79608603   A              28.0   28.22       33.0         7.0          1.3    15.0    68.8    16.9     2.0
# 79608605   AC             48.0   28.22       46.0        23.0         1.48    15.0    60.0     9.9     1.0
# 79608601   C1            112.0   91.74       49.0        10.0         1.52     5.0    85.0     7.6     0.5
# 79608602   C2            152.0   91.74       37.0        22.0          1.5     5.0    80.0    15.4     0.5
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
# 79608603::wilt_pt estimated from wfifteenbar_r and rock
# 79608603::field_cap estimated from wthirdbar_r and rock
# 79608605::wilt_pt estimated from wfifteenbar_r and rock
# 79608605::field_cap estimated from wthirdbar_r and rock
# 79608601::wilt_pt estimated from wfifteenbar_r and rock
# 79608601::field_cap estimated from wthirdbar_r and rock
# 79608602::wilt_pt estimated from wfifteenbar_r and rock
# 79608602::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-06-26 20:41:06.977358
# Source File: :/wc1/runs/ol/old-fluorosis/soils/768928.sol
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
'Matcher family-Rock outcrop-Lithic Cryorthents complex, 40 to 150 percent slopes'	 'STV-FSL'	 5	 0.16	 0.75	 400000	 8e-05	 2
	200.0	 1.3	 60	 10.0	 0.258	 0.14	 68.8	 15.0	 2.0	 11.3	 55.0	 0.07802	 0.4348	 0.01487	 1.475	 75.07	 0.1054	 0.2226
	280.0	 1.3	 101.592	 10.0	 0.258	 0.14	 68.8	 15.0	 2.0	 11.3	 55.0	 0.07802	 0.4348	 0.01487	 1.475	 75.07	 0.1054	 0.2226
	480.0	 1.48	 101.592	 10.0	 0.122	 0.064	 60.0	 15.0	 1.0	 11.3	 80.47	 0.07422	 0.3856	 0.01331	 1.424	 27.22	 0.1071	 0.2244
	1120.0	 1.52	 330.264	 1.0	 0.054	 0.018	 85.0	 5.0	 0.5	 11.3	 87.29	 0.05495	 0.3717	 0.0258	 1.964	 194.1	 0.05597	 0.1107
	1600.0	 1.5	 330.264	 1.0	 0.072	 0.022	 80.0	 5.0	 0.5	 11.3	 82.37	 0.05547	 0.3731	 0.02285	 1.758	 126.3	 0.05927	 0.1347
1 10000.0 0.01
