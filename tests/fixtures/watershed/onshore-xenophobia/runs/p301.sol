9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:09.333091
# Source Data: Surgo
#
# Mukey: 62724
# Major Component: 27032853 (comppct_r = 90.0)
# Texture: loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80724187   Oe     X        3.0   373.0        0.0         0.0          0.2    15.0    35.0     5.0    75.0
# 80724188   H1             33.0     9.0        0.0        20.0          0.9    22.5    39.8    11.9     6.0
# 80724189   H2            110.0     9.0        0.0        25.0         1.05    28.0    34.4    10.3    2.25
# 80724190   H3            130.0     9.0        0.0        33.0         1.05    21.0    41.6    12.0    0.25
# 80724191   H4            155.016.631501366310225         -           -        1.5196     7.0    66.8    10.0     7.0
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
# 80724188::wilt_pt estimated from wfifteenbar_r and rock
# 80724188::field_cap estimated from wthirdbar_r and rock
# 80724189::wilt_pt estimated from wfifteenbar_r and rock
# 80724189::field_cap estimated from wthirdbar_r and rock
# 80724190::wilt_pt estimated from wfifteenbar_r and rock
# 80724190::field_cap estimated from wthirdbar_r and rock
# 80724191::using default rock content of 55.5%
# 80724191::ksat_r estimated from rosetta2
# 80724191::wilt_pt estimated from rosetta2
# 80724191::field_cap estimated from rosetta2
# 80724191::bd estimated from sand, vfs, and clay
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
# Source soil utility: 9002.0migration
# Build Date: 2026-04-28 19:18:17.916776
# Source File: soils/62724.sol
#
# Replacements
# --------------------------
# luse -> forest moderate sev fire
# stext -> loam
# ki -> 1000000
# kr -> 8.00E-05
# shcrit -> 1
# avke -> 20
# bd ->
# ksflag -> 0
# ksatadj -> 0
# ksatfac -> 1.3
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.3
# xmxlai -> 4
# keffflag -> 1
# lkeff -> 1
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
#
# h0_min_depth = None
# h0_max_om = None
#
# Source soil utility: modify_initial_sat(initial_sat=0.75)
Any comments:
1 0
0	 'forest moderate sev fire'	 'loam'	 1.3 	 0.3
'Kinney cobbly loam, 20 to 50 percent south slopes'	 'CB-L'	 5	 0.23	 0.75	 1000000	 8e-05	 1
	200.0	 0.9	 20	 10.0	 0.3588	 0.1647	 39.8	 22.5	 6.0	 22.5	 32.0	 0.09949	 0.5372	 0.005576	 1.435	 142.7	 0.1633	 0.2774
	330.0	 0.9	 32.4	 10.0	 0.3588	 0.1647	 39.8	 22.5	 6.0	 22.5	 32.0	 0.09949	 0.5372	 0.005576	 1.435	 142.7	 0.1633	 0.2774
	1100.0	 1.05	 32.4	 1.0	 0.3703	 0.1829	 34.4	 28.0	 2.25	 15.0	 34.38	 0.1046	 0.5103	 0.006051	 1.409	 66.22	 0.1687	 0.2894
	1300.0	 1.05	 32.4	 1.0	 0.3371	 0.1229	 41.6	 21.0	 0.25	 12.5	 43.05	 0.09346	 0.4923	 0.006136	 1.44	 76.9	 0.1479	 0.2638
	1600.0	 1.52	 59.8734	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
1 10000.0 0.01
