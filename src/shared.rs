// sequence of labels
// each label is a length octet & that number of octets
// terminates with 0-length octet
fn name(
    input: &[u8],
    offset: usize,
    compression_map: &mut std::collections::HashMap<usize, usize>,
    tr: &mut std::collections::BTreeMap<usize, String>,
) -> Vec<String> {
    let mut s = Vec::new();
    let mut length = input[0] as usize;
    let mut idx = 1;
    let mut last = 0;
    while length != 0x00 {
        let next = std::str::from_utf8(input.get(idx..(idx + length)).unwrap()).unwrap();
        s.push(next.into());

        //compression_map.insert(offset + idx - 1, next.into());

        tr.insert(offset + idx - 1, next.into());
        last = offset + idx - 1;

        idx += length;
        //dbg!(length);
        //dbg!(idx);
        //dbg!(&input);
        length = *input.get(idx).unwrap() as usize;
        idx += 1;
    }
    compression_map.insert(offset, last + 1);
    s
}

#[test]
fn name_test() {
    let mut compression_map = std::collections::HashMap::new();
    let mut tr = std::collections::BTreeMap::new();
    let case = [
        0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00,
    ];
    //dbg!(name(&case, 0, &mut compression_map, &mut tr));
    //dbg!(&compression_map);
    //dbg!(&tr);
    //dbg!(&tr.range(0..*compression_map.get(&0).unwrap()));

    let one = [
        0x01, 0x46, 0x03, 0x49, 0x53, 0x49, 0x04, 0x41, 0x52, 0x50, 0x41, 0x03, 0x43, 0x4f, 0x4d,
        0x00,
    ];
    let two = [0x03, 0x46, 0x4f, 0x4f, 0x00];
    dbg!(name(&one, 0, &mut compression_map, &mut tr));
    dbg!(name(&two, 16, &mut compression_map, &mut tr));

    dbg!(&tr.range(0..*compression_map.get(&0).unwrap()));
    dbg!();
    dbg!(&tr.get(&16).unwrap());
    dbg!(&tr.range(0..*compression_map.get(&0).unwrap()));
    dbg!();
    dbg!(&tr.range(11..*compression_map.get(&0).unwrap()));
}
