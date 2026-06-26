9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 22:11:35.299184
# Source Data: Surgo
# 
# Mukey: 665220
# Major Component: 14214185 (comppct_r = 14.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 40892871   H1             13.0  9.1743         -         10.0          1.0     4.5    64.6    17.1     7.0
# 40892872   H2             58.0  9.1743         -          5.0         1.05     4.5    64.6    17.1     7.0
# 40892873   H3            165.0 28.2287         -         18.0         1.45     3.0    80.2    15.4     7.0
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
# 40892871::wilt_pt estimated from rosetta3
# 40892871::field_cap estimated from rosetta3
# 40892872::wilt_pt estimated from rosetta3
# 40892872::field_cap estimated from rosetta3
# 40892873::wilt_pt estimated from rosetta3
# 40892873::field_cap estimated from rosetta3
# albedo estimated from om_r (7.0%)
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
# Build Date: 2026-06-26 22:11:35.613430
# Source File: :/wc1/runs/sc/scabby-demographic/soils/665220.sol
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
'Skerry-Monadnock-Lyman-Hermon (s5004)'	 'FSL'	 4	 0.0365	 0.75	 400000	 8e-05	 2
	130.0	 1.0	 60	 10.0	 0.1916	 0.091	 64.6	 4.5	 7.0	 11.3	 28.0	 0.06507	 0.4741	 0.009798	 1.535	 224.5	 0.09345	 0.1988
	200.0	 1.05	 60	 1.0	 0.1867	 0.0882	 64.6	 4.5	 7.0	 11.3	 24.0	 0.06382	 0.4618	 0.01028	 1.54	 196.7	 0.08998	 0.1946
	580.0	 1.05	 33.0275	 1.0	 0.1867	 0.0882	 64.6	 4.5	 7.0	 11.3	 24.0	 0.06382	 0.4618	 0.01028	 1.54	 196.7	 0.08998	 0.1946
	1800.0	 1.45	 101.6233	 1.0	 0.1251	 0.0562	 80.2	 3.0	 7.0	 11.3	 46.7	 0.05316	 0.3803	 0.02265	 1.801	 162.9	 0.05622	 0.1271
1 10000.0 0.01
