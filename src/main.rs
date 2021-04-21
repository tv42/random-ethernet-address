fn main() {
    const ETH_ALEN: usize = 6;
    let mut addr = [0u8; ETH_ALEN];
    getrandom::getrandom(&mut addr).expect("getrandom");
    // clear multicast bit
    addr[0] &= 0xfe;
    // set local assignment bit (IEEE802)
    addr[0] |= 0x02;
    println!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        addr[0], addr[1], addr[2], addr[3], addr[4], addr[5],
    );
}
