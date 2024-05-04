fn crc() -> u32 {
    let mut crc_table_computed = false;
    let mut crc_table: [u32; 256] = [0; 256];
    let crc = &update_crc(
        0xffffffffu32,
        "RuStThis is where your secret message will be!".as_bytes(),
        46,
        &mut crc_table,
        &mut crc_table_computed,
    );
    println!("{}", crc);
    return *crc;
}
fn update_crc(
    crc: u32,
    ch: &[u8],
    len: u32,
    crc_table: &mut [u32; 256],
    crc_table_computed: &mut bool,
) -> u32 {
    let mut buf = BufReader::new(ch);
    let mut reading: [u8; 1] = [0];
    let mut c = crc;
    if !*crc_table_computed {
        make_crc_table(crc_table, crc_table_computed)
    }
    for n in 0..len {
        buf.read_exact(&mut reading);
        c = crc_table[((c ^ reading[0] as u32) & 0xff) as usize] ^ (c >> 8);
    }
    return c;
}
fn make_crc_table(crc_table: &mut [u32; 256], crc_table_computed: &mut bool) {
    let mut c: u32;
    for n in 0..256 {
        c = n as u32;
        for _k in 0..8 {
            if (c & 1) == 1 {
                c = 0xedb88320u32 ^ (c >> 1);
            } else {
                c = c >> 1;
            }
            crc_table[n] = c;
        }
        *crc_table_computed = true;
    }
}