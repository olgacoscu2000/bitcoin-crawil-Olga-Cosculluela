// Test blocks checksum V0
// let header = [
//   0x00, 0x00, 0x80, 0x20,
//   0x19, 0x61, 0x47, 0x2a, 0x82, 0x73, 0x1b, 0xeb, 0xe2, 0x40, 0x36, 0x53, 0xed, 0x4f, 0xc7, 0xd8, 0x87, 0x42, 0xd9, 0x4a, 0x69, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//   0x24, 0x0b, 0x12, 0x0b, 0x83, 0xc7, 0x8c, 0xd4, 0x96, 0x7d, 0x01, 0x77, 0xfc, 0x80, 0x38, 0xeb, 0xaf, 0xba, 0x49, 0xa2, 0xdf, 0x34, 0xbb, 0xbd, 0xba, 0x12, 0x70, 0xbd, 0xfb, 0x22, 0x0b, 0xa1,
//   0x69, 0x08, 0x99, 0x61,
//   0xea, 0x69, 0x0c, 0x17,
//   0x2c, 0x79, 0x25, 0x19,
//   0x22];
//
// eprintln!("{:02x?}", &header[..80]);
// std::process::exit(1);

// let mut hasher = Sha256::new();
// hasher.input(&header);
// let sum = hasher.result();
// let mut hasher2 = Sha256::new();
// hasher2.input(sum);
// let mut result = hasher2.result();
// result.reverse();
// let format = hex::encode(result);
// eprintln!("{:?}", format);

// Test blocks checksum V1
// let hash = sha256d::Hash::hash(&header);
// eprintln!("{:?}", hash.to_string());


// let gen:Vec<u8> = vec![0x00, 0x01, 0x02];
// let gen2 = Vec::from_hex("000102").unwrap();
// // block_message.extend((Vec::from_hex(search_block).unwrap()).to_be_bytes());
// let mut tmp = Vec::hex::from_hex("0000000000000000000c1c499d6f1e87e199633a8c811f18a8e5a86f40b3fb50").unwrap();
// tmp.reverse();
// eprintln!("{:02x?}", tmp);
// std::process::exit(1);

// let toto = hex::encode([0x00, 0x02, 0xFF]);
//
// eprintln!("{:02X?}",gen);
// eprintln!("{:02X?}",gen2);
// dbg!("coucou");
// eprintln!("{}", toto);
// std::process::exit(1);
