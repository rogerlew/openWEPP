9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:08.290992
# Source Data: Surgo
#
# Mukey: 62666
# Major Component: 27032753 (comppct_r = 80.0)
# Texture: clay loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80723936   Oi     X        3.0   373.0        0.0         0.0          0.2    15.0    35.0     5.0    75.0
# 80723937   H1             39.0     9.0        0.0         3.0          1.3    31.0    20.0    12.2     5.0
# 80723938   H2            152.0     3.0        0.0         3.0         1.35    47.5    23.3     5.6    2.05
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
# 80723937::wilt_pt estimated from wfifteenbar_r and rock
# 80723937::field_cap estimated from wthirdbar_r and rock
# 80723938::wilt_pt estimated from wfifteenbar_r and rock
# 80723938::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-04-28 19:18:26.447821
# Source File: soils/62666.sol
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
'Cumley silty clay loam, 2 to 20 percent slopes'	 'SICL'	 3	 0.23	 0.75	 1500000	 6e-05	 0.5
	200.0	 1.3	 14	 10.0	 0.3578	 0.2307	 20.0	 31.0	 5.0	 17.5	 10.27	 0.1061	 0.4535	 0.005098	 1.406	 16.84	 0.1658	 0.2857
	390.0	 1.3	 32.4	 10.0	 0.3578	 0.2307	 20.0	 31.0	 5.0	 17.5	 10.27	 0.1061	 0.4535	 0.005098	 1.406	 16.84	 0.1658	 0.2857
	1600.0	 1.35	 10.8	 1.0	 0.4157	 0.2898	 23.3	 47.5	 2.05	 17.5	 10.27	 0.125	 0.4734	 0.008632	 1.297	 13.9	 0.207	 0.3382
1 10000.0 0.01
