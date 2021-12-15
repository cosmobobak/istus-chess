#![allow(clippy::unreadable_literal)]

use crate::bitmethods::Bithackable;

pub const BB_KNIGHT_ATTACKS: [u64; 64] = [
    132096, 329728, 659712, 1319424, 2638848, 5277696, 10489856, 4202496, 33816580, 84410376, 168886289, 337772578, 675545156, 1351090312, 2685403152, 1075839008, 8657044482, 21609056261, 43234889994, 86469779988, 172939559976, 345879119952, 687463207072, 275414786112, 2216203387392, 5531918402816, 11068131838464, 22136263676928, 44272527353856, 88545054707712, 175990581010432, 70506185244672, 567348067172352, 1416171111120896, 2833441750646784, 5666883501293568, 11333767002587136, 22667534005174272, 45053588738670592, 18049583422636032, 145241105196122112, 362539804446949376, 725361088165576704, 1450722176331153408, 2901444352662306816, 5802888705324613632, 11533718717099671552, 4620693356194824192, 288234782788157440, 576469569871282176, 1224997833292120064, 2449995666584240128, 4899991333168480256, 9799982666336960512, 1152939783987658752, 2305878468463689728, 1128098930098176, 2257297371824128, 4796069720358912, 9592139440717824, 19184278881435648, 38368557762871296, 4679521487814656, 9077567998918656];

pub const BB_KING_ATTACKS: [u64; 64] = [
    770, 1797, 3594, 7188, 14376, 28752, 57504, 49216, 197123, 460039, 920078, 1840156, 3680312, 7360624, 14721248, 12599488, 50463488, 117769984, 235539968, 471079936, 942159872, 1884319744, 3768639488, 3225468928, 12918652928, 30149115904, 60298231808, 120596463616, 241192927232, 482385854464, 964771708928, 825720045568, 3307175149568, 7718173671424, 15436347342848, 30872694685696, 61745389371392, 123490778742784, 246981557485568, 211384331665408, 846636838289408, 1975852459884544, 3951704919769088, 7903409839538176, 15806819679076352, 31613639358152704, 63227278716305408, 54114388906344448, 216739030602088448, 505818229730443264, 1011636459460886528, 2023272918921773056, 4046545837843546112, 8093091675687092224, 16186183351374184448, 13853283560024178688, 144959613005987840, 362258295026614272, 724516590053228544, 1449033180106457088, 2898066360212914176, 5796132720425828352, 11592265440851656704, 4665729213955833856];

pub const BB_PAWN_ATTACKS: [[u64; 64]; 2] = [
    [512, 1280, 2560, 5120, 10240, 20480, 40960, 16384, 131072, 327680, 655360, 1310720, 2621440, 5242880, 10485760, 4194304, 33554432, 83886080, 167772160, 335544320, 671088640, 1342177280, 2684354560, 1073741824, 8589934592, 21474836480, 42949672960, 85899345920, 171798691840, 343597383680, 687194767360, 274877906944, 2199023255552, 5497558138880, 10995116277760, 21990232555520, 43980465111040, 87960930222080, 175921860444160, 70368744177664, 562949953421312, 1407374883553280, 2814749767106560, 5629499534213120, 11258999068426240, 22517998136852480, 45035996273704960, 18014398509481984, 144115188075855872, 360287970189639680, 720575940379279360, 1441151880758558720, 2882303761517117440, 5764607523034234880, 11529215046068469760, 4611686018427387904, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 2, 5, 10, 20, 40, 80, 160, 64, 512, 1280, 2560, 5120, 10240, 20480, 40960, 16384, 131072, 327680, 655360, 1310720, 2621440, 5242880, 10485760, 4194304, 33554432, 83886080, 167772160, 335544320, 671088640, 1342177280, 2684354560, 1073741824, 8589934592, 21474836480, 42949672960, 85899345920, 171798691840, 343597383680, 687194767360, 274877906944, 2199023255552, 5497558138880, 10995116277760, 21990232555520, 43980465111040, 87960930222080, 175921860444160, 70368744177664, 562949953421312, 1407374883553280, 2814749767106560, 5629499534213120, 11258999068426240, 22517998136852480, 45035996273704960, 18014398509481984]];

pub const BB_RANK_ATTACKS: [u64; 64] = [
    254,253,251,247,239,223,191,127, 65024, 64768,64256, 63232, 61184, 57088, 48896, 32512, 16646144, 16580608, 16449536, 16187392, 15663104, 14614528, 12517376, 8323072, 4261412864, 4244635648, 4211081216, 4143972352, 4009754624, 3741319168, 3204448256, 2130706432, 1090921693184, 1086626725888, 1078036791296, 1060856922112, 1026497183744, 957777707008, 820338753536, 545460846592, 279275953455104, 278176441827328, 275977418571776, 271579372060672, 262783279038464, 245191092994048, 210006720905216, 139637976727552, 71494644084506624, 71213169107795968, 70650219154374656, 69524319247532032, 67272519433846784, 62768919806476288, 53761720551735296, 35747322042253312, 18302628885633695744, 18230571291595767808, 18086456103519911936, 17798225727368200192, 17221764975064776704, 16068843470457929728, 13763000461244235776, 9151314442816847872];

pub const BB_FILE_ATTACKS: [u64; 64] = [
    72340172838076672, 144680345676153344, 289360691352306688, 578721382704613376, 1157442765409226752, 2314885530818453504, 4629771061636907008, 9259542123273814016, 72340172838076417, 144680345676152834, 289360691352305668, 578721382704611336, 1157442765409222672, 2314885530818445344, 4629771061636890688, 9259542123273781376, 72340172838011137, 144680345676022274, 289360691352044548, 578721382704089096, 1157442765408178192, 2314885530816356384, 4629771061632712768, 9259542123265425536, 72340172821299457, 144680345642598914, 289360691285197828, 578721382570395656, 1157442765140791312, 2314885530281582624, 4629771060563165248, 9259542121126330496, 72340168543109377, 144680337086218754, 289360674172437508, 578721348344875016, 1157442696689750032, 2314885393379500064, 4629770786759000128, 9259541573518000256, 72339073326448897, 144678146652897794, 289356293305795588, 578712586611591176, 1157425173223182352, 2314850346446364704, 4629700692892729408, 9259401385785458816, 72058697861366017, 144117395722732034, 288234791445464068, 576469582890928136, 1152939165781856272, 2305878331563712544, 4611756663127425088, 9223513326254850176, 282578800148737, 565157600297474, 1130315200594948, 2260630401189896, 4521260802379792, 9042521604759584, 18085043209519168, 36170086419038336];

pub const BB_DIAG_ATTACKS: [u64; 64] = [
    9241421688590303744, 36099303471056128, 141012904249856, 550848566272, 6480472064, 1108177604608, 283691315142656, 72624976668147712, 4620710844295151618, 9241421688590368773, 36099303487963146, 141017232965652, 1659000848424, 83693466779728,72624976676520096, 145249953336262720, 2310355422147510788, 4620710844311799048, 9241421692918565393, 36100411639206946, 424704217196612, 72625527495610504, 145249955479592976, 290499906664153120, 1155177711057110024, 2310355426409252880, 4620711952330133792, 9241705379636978241, 108724279602332802, 145390965166737412, 290500455356698632, 580999811184992272, 577588851267340304, 1155178802063085600, 2310639079102947392, 4693335752243822976, 9386671504487645697, 326598935265674242, 581140276476643332, 1161999073681608712, 288793334762704928, 577868148797087808, 1227793891648880768, 2455587783297826816, 4911175566595588352, 9822351133174399489, 1197958188344280066, 2323857683139004420, 144117404414255168, 360293502378066048, 720587009051099136, 1441174018118909952, 2882348036221108224, 5764696068147249408, 11529391036782871041, 4611756524879479810, 567382630219904, 1416240237150208, 2833579985862656, 5667164249915392, 11334324221640704, 22667548931719168, 45053622886727936, 18049651735527937];

lazy_static! {
    pub static ref BB_RAYS: [[u64; 64]; 64] = {
        let mut rays = [[0; 64]; 64];
        for (a, bb_a) in (0..64).map(|i| (i, 1 << i)) {
            for (b, bb_b) in (0..64).map(|i| (i, 1 << i)) {
                if (BB_DIAG_ATTACKS[a] & bb_b).has_any_set() {
                    rays[a][b] = (BB_DIAG_ATTACKS[a] & BB_DIAG_ATTACKS[b]) | bb_a | bb_b;
                } else if (BB_RANK_ATTACKS[a] & bb_b).has_any_set() {
                    rays[a][b] = BB_RANK_ATTACKS[a] | bb_a;
                } else {
                    rays[a][b] = 0;
                }
            }
        }
        rays
    };
}