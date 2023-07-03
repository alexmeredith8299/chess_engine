pub const H1: u64 = 1;
pub const G1: u64 = 2;
pub const F1: u64 = 4;
pub const E1: u64 = 8;
pub const D1: u64 = 16;
pub const C1: u64 = 32;
pub const B1: u64 = 64;
pub const A1: u64 = 128;
pub const H2: u64 = 256;
pub const G2: u64 = 512;
pub const F2: u64 = 1024;
pub const E2: u64 = 2048;
pub const D2: u64 = 4096;
pub const C2: u64 = 8192;
pub const B2: u64 = 16384;
pub const A2: u64 = 32768;
pub const H3: u64 = 65536;
pub const G3: u64 = 131072;
pub const F3: u64 = 262144;
pub const E3: u64 = 524288;
pub const D3: u64 = 1048576;
pub const C3: u64 = 2097152;
pub const B3: u64 = 4194304;
pub const A3: u64 = 8388608;
pub const H4: u64 = 16777216;
pub const G4: u64 = 33554432;
pub const F4: u64 = 67108864;
pub const E4: u64 = 134217728;
pub const D4: u64 = 268435456;
pub const C4: u64 = 536870912;
pub const B4: u64 = 1073741824;
pub const A4: u64 = 2147483648;
pub const H5: u64 = 4294967296;
pub const G5: u64 = 8589934592;
pub const F5: u64 = 17179869184;
pub const E5: u64 = 34359738368;
pub const D5: u64 = 68719476736;
pub const C5: u64 = 137438953472;
pub const B5: u64 = 274877906944;
pub const A5: u64 = 549755813888;
pub const H6: u64 = 1099511627776;
pub const G6: u64 = 2199023255552;
pub const F6: u64 = 4398046511104;
pub const E6: u64 = 8796093022208;
pub const D6: u64 = 17592186044416;
pub const C6: u64 = 35184372088832;
pub const B6: u64 = 70368744177664;
pub const A6: u64 = 140737488355328;
pub const H7: u64 = 281474976710656;
pub const G7: u64 = 562949953421312;
pub const F7: u64 = 1125899906842624;
pub const E7: u64 = 2251799813685248;
pub const D7: u64 = 4503599627370496;
pub const C7: u64 = 9007199254740992;
pub const B7: u64 = 18014398509481984;
pub const A7: u64 = 36028797018963968;
pub const H8: u64 = 72057594037927936;
pub const G8: u64 = 144115188075855872;
pub const F8: u64 = 288230376151711744;
pub const E8: u64 = 576460752303423488;
pub const D8: u64 = 1152921504606846976;
pub const C8: u64 = 2305843009213693952;
pub const B8: u64 = 4611686018427387904;
pub const A8: u64 = 9223372036854775808;

pub const A_file: u64 = A8 ^ A7 ^ A6 ^ A5 ^ A4 ^ A3 ^ A2 ^ A1;
pub const B_file: u64 = B8 ^ B7 ^ B6 ^ B5 ^ B4 ^ B3 ^ B2 ^ B1;
pub const C_file: u64 = C8 ^ C7 ^ C6 ^ C5 ^ C4 ^ C3 ^ C2 ^ C1;
pub const D_file: u64 = D8 ^ D7 ^ D6 ^ D5 ^ D4 ^ D3 ^ D2 ^ D1;
pub const E_file: u64 = E8 ^ E7 ^ E6 ^ E5 ^ E4 ^ E3 ^ E2 ^ E1;
pub const F_file: u64 = F8 ^ F7 ^ F6 ^ F5 ^ F4 ^ F3 ^ F2 ^ F1;
pub const G_file: u64 = G8 ^ G7 ^ G6 ^ G5 ^ G4 ^ G3 ^ G2 ^ G1;
pub const H_file: u64 = H8 ^ H7 ^ H6 ^ H5 ^ H4 ^ H3 ^ H2 ^ H1;

pub const first_rank: u64 = A1 ^ B1 ^ C1 ^ D1 ^ E1 ^ F1 ^ G1 ^ H1;
pub const second_rank: u64 = A2 ^ B2 ^ C2 ^ D2 ^ E2 ^ F2 ^ G2 ^ H2;
pub const third_rank: u64 = A3 ^ B3 ^ C3 ^ D3 ^ E3 ^ F3 ^ G3 ^ H3;
pub const fourth_rank: u64 = A4 ^ B4 ^ C4 ^ D4 ^ E4 ^ F4 ^ G4 ^ H4;
pub const fifth_rank: u64 = A5 ^ B5 ^ C5 ^ D5 ^ E5 ^ F5 ^ G5 ^ H5;
pub const sixth_rank: u64 = A6 ^ B6 ^ C6 ^ D6 ^ E6 ^ F6 ^ G6 ^ H6;
pub const seventh_rank: u64 = A7 ^ B7 ^ C7 ^ D7 ^ E7 ^ F7 ^ G7 ^ H7;
pub const eighth_rank: u64 = A8 ^ B8 ^ C8 ^ D8 ^ E8 ^ F8 ^ G8 ^ H8;

pub const knight_d4_bitstr: &str = "00000000\
                                    00000000\
                                    00101000\
                                    01000100\
                                    00000000\
                                    01000100\
                                    00101000\
                                    00000000";

pub const king_d4_bitstr: &str = "00000000\
                                  00000000\
                                  00000000\
                                  00111000\
                                  00101000\
                                  00111000\
                                  00000000\
                                  00000000";

//Most significant first
pub const squares: [u64; 64] = [
    A8, B8, C8, D8, E8, F8, G8, H8, A7, B7, C7, D7, E7, F7, G7, H7, A6, B6, C6, D6, E6, F6, G6,
    H6, A5, B5, C5, D5, E5, F5, G5, H5, A4, B4, C4, D4, E4, F4, G4, H4, A3, B3, C3, D3, E3, F3,
    G3, H3, A2, B2, C2, D2, E2, F2, G2, H2, A1, B1, C1, D1, E1, F1, G1, H1
];

pub const square_names: [&str; 64] = ["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
                                                                                          "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
                                                                                          "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
                                                                                          "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
                                                                                          "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
                                                                                          "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
                                                                                          "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
                                                                                          "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"];

