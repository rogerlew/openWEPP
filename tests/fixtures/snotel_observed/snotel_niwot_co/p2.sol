9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-25 16:00:08.921678
# Source Data: Surgo
# 
# Mukey: 762986
# Major Component: 26663599 (comppct_r = 85.0)
# Texture: silt loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 79608357   A               5.0    9.17        3.0        24.0         1.32    18.0    15.0     5.0     1.0
# 79608358   AE             23.0    9.17        6.0        25.0         1.32    18.0    15.0     5.0     1.0
# 79608354   Bw             71.0   28.22        6.0        25.0         1.48     6.0    70.0    13.0     0.5
# 79608355   BCd           114.0   28.22       16.0        22.0         1.72     2.0    78.0    15.0     0.5
# 79608356   Cd            152.0   28.22       21.0        21.0         1.72     2.0    78.0    15.0     0.0
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
# 79608357::wilt_pt estimated from wfifteenbar_r and rock
# 79608357::field_cap estimated from wthirdbar_r and rock
# 79608358::wilt_pt estimated from wfifteenbar_r and rock
# 79608358::field_cap estimated from wthirdbar_r and rock
# 79608354::wilt_pt estimated from wfifteenbar_r and rock
# 79608354::field_cap estimated from wthirdbar_r and rock
# 79608355::wilt_pt estimated from wfifteenbar_r and rock
# 79608355::field_cap estimated from wthirdbar_r and rock
# 79608356::wilt_pt estimated from wfifteenbar_r and rock
# 79608356::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-06-25 16:00:09.979058
# Source File: :/wc1/runs/de/deathless-wangle/soils/762986.sol
# 
# Replacements
# --------------------------
# luse -> forest
# stext -> silt loam
# ki -> 1000000
# kr -> 5.00E-05
# shcrit -> 1.5
# avke -> 40
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
0	 'forest'	 'silt loam'	 1.5 	 0.3
'Leighcan family, till substratum, 5 to 40 percent slopes'	 'CB-SIL'	 6	 0.16	 0.75	 1000000	 5e-05	 1.5
	50.0	 1.32	 40	 10.0	 0.3201	 0.1308	 15.0	 18.0	 1.0	 15.1	 35.03	 0.08433	 0.4314	 0.003719	 1.518	 25.27	 0.1274	 0.2323
	200.0	 1.32	 40	 10.0	 0.242	 0.098	 15.0	 18.0	 1.0	 15.1	 70.33	 0.08433	 0.4314	 0.003719	 1.518	 25.27	 0.1274	 0.2323
	230.0	 1.32	 33.012	 10.0	 0.242	 0.098	 15.0	 18.0	 1.0	 15.1	 70.33	 0.08433	 0.4314	 0.003719	 1.518	 25.27	 0.1274	 0.2323
	710.0	 1.48	 101.592	 1.0	 0.138	 0.046	 70.0	 6.0	 0.5	 5.5	 63.43	 0.05772	 0.3731	 0.01768	 1.562	 65.73	 0.07139	 0.1712
	1140.0	 1.72	 101.592	 1.0	 0.064	 0.014	 78.0	 2.0	 0.5	 2.0	 77.68	 0.04805	 0.3163	 0.026	 1.662	 58.68	 0.05322	 0.1331
	1600.0	 1.72	 101.592	 1.0	 0.044	 0.008	 78.0	 2.0	 0.0	 1.8	 81.44	 0.04805	 0.3163	 0.026	 1.662	 58.68	 0.05322	 0.1331
1 10000.0 0.01
