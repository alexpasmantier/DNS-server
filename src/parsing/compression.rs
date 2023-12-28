pub fn decompress_domain_name<'a>(
    compressed_name: &'a [u8],
    entire_message: &[u8],
) -> (Vec<u8>, &'a [u8]) {
    let mut decompressed_name: Vec<u8> = Vec::new();

    let mut counter = 0;
    let mut reached_null_byte = false;

    while !reached_null_byte {
        // if it's a pointer (i.e. the first two bits are 1 1)
        if (compressed_name[counter] & 0b11000000) == 192 {
            // fetch length and content in `entire_message`
            let byte_offset = ((compressed_name[counter] as u16 & 0b00111111) << 8)
                | compressed_name[counter + 1] as u16;
            let decompressed_content = entire_message[byte_offset as usize..]
                .splitn(2, |b| *b == 0_u8)
                .next()
                .unwrap();
            decompressed_name.extend_from_slice(decompressed_content);
            counter += 1;
            reached_null_byte = true;
        } else {
            // read length and content directly in compressed_name
            let label_length = compressed_name[counter];
            decompressed_name.push(label_length);
            decompressed_name.extend_from_slice(
                &compressed_name[(counter + 1) as usize..counter + 1 + label_length as usize],
            );
            counter += label_length as usize + 1;
            if compressed_name[counter] == 0 {
                reached_null_byte = true;
            }
        }
    }
    decompressed_name.push(0);
    return (decompressed_name, &compressed_name[counter + 1..]);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_decompress_domain_name() {
//         let compressed_name = "google.com".to_string();
//         let expected_vec: Vec<u8> = vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0];
//         assert_eq!(encode_label(&input_string), expected_vec);
//     }
// }
//
