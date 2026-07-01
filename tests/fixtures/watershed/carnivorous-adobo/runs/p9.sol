9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-05-09 14:53:22.916473
# Source Data: Surgo
#
# Mukey: 676031
# Major Component: 14269504 (comppct_r = 20.0)
# Texture: sand loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 41069770   H1             23.0  9.1743        2.0         8.0         0.75     7.0    66.8    10.0     3.5
# 41069771   H2             36.0  9.1743       15.0        23.0          0.9     7.0    66.8    10.0     2.0
# 41069772   H3             61.0  9.1743        5.0        28.0         1.15     7.0    66.8    10.0     1.5
# 41069773   H4             71.0 21.3832         -           -        1.5196     7.0    66.8    10.0     7.0
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
# 41069770::wilt_pt estimated from rosetta3
# 41069770::field_cap estimated from rosetta3
# 41069771::wilt_pt estimated from rosetta3
# 41069771::field_cap estimated from rosetta3
# 41069772::wilt_pt estimated from rosetta3
# 41069772::field_cap estimated from rosetta3
# 41069773::using default rock content of 55.5%
# 41069773::wilt_pt estimated from rosetta2
# 41069773::field_cap estimated from rosetta2
# 41069773::bd estimated from sand, vfs, and clay
# albedo estimated from om_r (3.5%)
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
# WEPPcloud soil utility: 9002.0 migration
# Build Date: 2026-05-09 14:53:23.532786
# Source soil file id: 676031.sol (absolute source path omitted)
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
# ksflag -> 0
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
# WEPPcloud soil utility: modify_initial_sat(initial_sat=0.75)
Any comments:
1 0
0	 'forest'	 'sand loam'	 1.5 	 0.3
'Rock outcrop-Glaciers-Cryumbrepts-Cryorthods-Andic Cryumbrepts (s8651)'	 'SL'	 5	 0.148	 0.75	 400000	 8e-05	 2
	200.0	 0.75	 60	 10.0	 0.2358	 0.1159	 66.8	 7.0	 3.5	 11.3	 14.5	 0.07719	 0.5517	 0.009022	 1.463	 357.5	 0.1261	 0.245
	230.0	 0.75	 33.0275	 10.0	 0.2358	 0.1159	 66.8	 7.0	 3.5	 11.3	 14.5	 0.07719	 0.5517	 0.009022	 1.463	 357.5	 0.1261	 0.245
	360.0	 0.9	 33.0275	 10.0	 0.2101	 0.1025	 66.8	 7.0	 2.0	 11.3	 65.9	 0.0725	 0.5096	 0.009999	 1.495	 269.1	 0.109	 0.2225
	610.0	 1.15	 33.0275	 1.0	 0.1819	 0.0883	 66.8	 7.0	 1.5	 11.3	 68.18	 0.06619	 0.447	 0.01229	 1.53	 143.1	 0.09013	 0.198
	800.0	 1.52	 76.9795	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
1 10000.0 0.01
