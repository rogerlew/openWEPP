9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-25 15:51:54.134126
# Source Data: Surgo
# 
# Mukey: 508208
# Major Component: 26411664 (comppct_r = 30.0)
# Texture: loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 78806123   A1             15.0    9.17        8.0        33.0         1.23    22.5    39.8    11.8     2.5
# 78806124   A2             38.0    9.17       10.0        43.0         1.25    22.5    39.8    11.8     1.5
# 78806125   Bk             48.0    9.17       10.0        43.0         1.25    22.5    39.8    11.8    0.75
# 78806126   R      R       73.0   0.005         -           -        1.5196     7.0    66.8    10.0     7.0
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: Lithic bedrock
# ksat (um/s): 0.00500
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
# 78806123::wilt_pt estimated from wfifteenbar_r and rock
# 78806123::field_cap estimated from wthirdbar_r and rock
# 78806124::wilt_pt estimated from wfifteenbar_r and rock
# 78806124::field_cap estimated from wthirdbar_r and rock
# 78806125::wilt_pt estimated from wfifteenbar_r and rock
# 78806125::field_cap estimated from wthirdbar_r and rock
# 78806126::using default rock content of 55.5%
# 78806126::wilt_pt estimated from rosetta2
# 78806126::field_cap estimated from rosetta2
# 78806126::bd estimated from sand, vfs, and clay
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
# Build Date: 2026-06-25 15:51:54.689284
# Source File: :/wc1/runs/ba/barred-pro/soils/508208.sol
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
'Rock outcrop-Starley family complex, 30 to 70 percent slopes'	 'CBV-L'	 4	 0.16	 0.75	 400000	 3e-05	 1
	150.0	 1.23	 50	 10.0	 0.332	 0.158	 39.8	 22.5	 2.5	 18.0	 61.65	 0.09288	 0.4498	 0.006742	 1.422	 32.23	 0.1438	 0.2631
	200.0	 1.25	 50	 10.0	 0.222	 0.102	 39.8	 22.5	 1.5	 15.0	 76.5	 0.09258	 0.4448	 0.006827	 1.419	 29.43	 0.1431	 0.2625
	380.0	 1.25	 33.012	 10.0	 0.222	 0.102	 39.8	 22.5	 1.5	 15.0	 76.5	 0.09258	 0.4448	 0.006827	 1.419	 29.43	 0.1431	 0.2625
	600.0	 1.25	 33.012	 10.0	 0.218	 0.096	 39.8	 22.5	 0.75	 15.0	 76.5	 0.09258	 0.4448	 0.006827	 1.419	 29.43	 0.1431	 0.2625
1 10000.0 2e-05
