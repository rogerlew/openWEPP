9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:07.733100
# Source Data: Surgo
#
# Mukey: 62704
# Major Component: 27032828 (comppct_r = 85.0)
# Texture: clay loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80725054   H1             23.0     3.0        0.0         0.0          1.3    35.0    17.3    10.6     6.5
# 80725055   H2            152.0     3.0        0.0         8.0          1.3    55.0    17.1     5.3    2.75
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
# 80725054::wilt_pt estimated from wfifteenbar_r and rock
# 80725054::field_cap estimated from wthirdbar_r and rock
# 80725055::wilt_pt estimated from wfifteenbar_r and rock
# 80725055::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-04-28 19:18:29.034388
# Source File: soils/62704.sol
#
# Replacements
# --------------------------
# luse -> forest high sev fire
# stext -> clay loam
# ki -> 1500000
# kr -> 6.00E-05
# shcrit -> 0.5
# avke -> 14
# bd ->
# ksflag -> 0
# ksatadj -> 1
# ksatfac -> 100
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.3
# xmxlai -> 2
# keffflag -> 1
# lkeff -> 0.1
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
#
# h0_min_depth = None
# h0_max_om = None
#
# Source soil utility: modify_initial_sat(initial_sat=0.75)
Any comments:
1 0
1	 'forest high sev fire'	 'clay loam'	 100 	 0.3
'Honeygrove silty clay loam, 25 to 50 percent slopes'	 'SICL'	 3	 0.16	 0.75	 1500000	 6e-05	 0.5
	200.0	 1.3	 14	 10.0	 0.359	 0.255	 17.3	 35.0	 6.5	 22.5	 0.0	 0.1119	 0.4628	 0.005341	 1.386	 15.85	 0.1765	 0.2994
	230.0	 1.3	 10.8	 10.0	 0.359	 0.255	 17.3	 35.0	 6.5	 22.5	 0.0	 0.1119	 0.4628	 0.005341	 1.386	 15.85	 0.1765	 0.2994
	1600.0	 1.3	 10.8	 1.0	 0.4658	 0.3516	 17.1	 55.0	 2.75	 20.0	 19.5	 0.1334	 0.5012	 0.008962	 1.283	 17.11	 0.2252	 0.3602
1 10000.0 0.01
