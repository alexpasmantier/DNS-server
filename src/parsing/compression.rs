pub fn decompress_domain_name(compressed_name: &[u8], entire_message: &[u8]) -> Vec<u8> {
    let decompressed_name: Vec<u8> = Vec::new();

    let compressed_length = compressed_name.len();
    let mut counter = 0;

    while counter + 1 < compressed_length {
        // if it's a pointer (i.e. the first two bits are 1 1)
        if (compressed_name[counter] & 0b11000000) == 192 {
            // fetch length and content in `entire_message`
            let byte_offset = ((compressed_name[counter] & 0b00111111) << 8) as u16
                | compressed_name[counter + 1] as u16;
            let decompressed_length = entire_message[byte_offset as usize];
            let decompressed_content = entire_message[(byte_offset + 1) as usize
                ..(byte_offset + 1 + decompressed_length as u16) as usize];
        } else {
            // read length and content directly in compressed_name
        }
        // push to decompressed_name
    }

    decompressed_name
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
