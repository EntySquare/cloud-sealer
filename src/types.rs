#[inline]
pub fn varuint(number: u64, buf: &mut [u8; 32]) -> &[u8] {
    let mut n = number;
    let mut i = 0;
    for b in buf.iter_mut() {
        *b = n as u8 | 0x80;
        n >>= 7;
        if n == 0 {
            *b &= 0x7f;
            break;
        }
        i += 1
    }
    debug_assert_eq!(n, 0);
    &buf[0..=i]
}

pub fn miner_id_to_prover_id(miner_id: u64) -> [u8; 32] {
    let mut buf = [0; 32];
    let mut buf2 =  [0; 32];
    let  prover_id:&[u8] = varuint(miner_id, &mut buf);
    for i in 0..32 {
        if i < prover_id.len() {
            buf2[i] = prover_id[i];
        }
    }
    buf2
}

#[test]
pub fn test_unsigned_varint() {
    let buf2 =  miner_id_to_prover_id(1000);
    println!("C2 — prover_id: {:?}", buf2);
}