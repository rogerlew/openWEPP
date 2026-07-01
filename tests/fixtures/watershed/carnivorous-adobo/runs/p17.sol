9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-05-09 14:53:22.906805
# Source Data: Surgo
#
# Mukey: 675975
# Major Component: 14268942 (comppct_r = 13.0)
# Texture: sand loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 41067878   H1             13.0 91.7432        0.0         0.0         0.75     7.0    66.8    10.0    12.5
# 41067879   H2             25.0 28.2287        0.0         0.0         0.83     7.0    66.8    10.0     4.0
# 41067880   H3             61.0  9.1743         -         23.0         0.98    10.0    66.9    11.8     2.0
# 41067881   H4            152.0  9.1743         -         25.0         1.05    10.0    46.0    12.9    0.75
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
# 41067878::wilt_pt estimated from rosetta3
# 41067878::field_cap estimated from rosetta3
# 41067879::wilt_pt estimated from rosetta3
# 41067879::field_cap estimated from rosetta3
# 41067880::wilt_pt estimated from wfifteenbar_r and rock
# 41067880::field_cap estimated from wthirdbar_r and rock
# 41067881::wilt_pt estimated from wfifteenbar_r and rock
# 41067881::field_cap estimated from wthirdbar_r and rock
# albedo estimated from om_r (12.5%)
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
# Build Date: 2026-05-09 14:53:23.249318
# Source soil file id: 675975.sol (absolute source path omitted)
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
'Playco-Nimue-Getchell (s8595)'	 'LS'	 5	 0.004	 0.75	 400000	 8e-05	 2
	130.0	 0.75	 60	 10.0	 0.2358	 0.1159	 66.8	 7.0	 12.5	 11.3	 5.0	 0.07719	 0.5517	 0.009022	 1.463	 357.5	 0.1261	 0.245
	200.0	 0.83	 60	 10.0	 0.2211	 0.1081	 66.8	 7.0	 4.0	 11.3	 7.5	 0.07455	 0.5287	 0.009508	 1.481	 311.4	 0.1163	 0.2321
	250.0	 0.83	 101.6233	 10.0	 0.2211	 0.1081	 66.8	 7.0	 4.0	 11.3	 7.5	 0.07455	 0.5287	 0.009508	 1.481	 311.4	 0.1163	 0.2321
	610.0	 0.98	 33.0275	 1.0	 0.14	 0.054	 66.9	 10.0	 2.0	 11.1	 76.9	 0.07602	 0.4975	 0.01104	 1.482	 201.3	 0.1119	 0.2286
	1600.0	 1.05	 33.0275	 1.0	 0.136	 0.034	 46.0	 10.0	 0.75	 8.1	 83.12	 0.07302	 0.4636	 0.005616	 1.523	 119.0	 0.1114	 0.2126
1 10000.0 0.01
