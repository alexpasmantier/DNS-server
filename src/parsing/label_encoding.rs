pub fn encode_label(input_string: &String) -> Vec<u8> {
    let mut encoded_label: Vec<u8> = Vec::new();
    for word in input_string.split(".") {
        encoded_label.push(word.chars().count().try_into().expect("Word is too long"));
        encoded_label.extend(word.to_string().as_bytes());
    }
    encoded_label.push(0);
    encoded_label
}

pub fn decode_label(input: &[u8]) -> String {
    let mut decoded_string = String::new();
    let input_length = input.len();
    let mut counter = 0;
    // println!("input length {}", input_length);

    while counter + 1 < input_length {
        let n_to_get = input[counter];
        // println!("n_to_get {}", n_to_get);
        input[(counter + 1)..(counter + 1 + n_to_get as usize)]
            .iter()
            .for_each(|b| decoded_string.push(*b as char));
        counter += n_to_get as usize + 1;
        if counter + 1 < input_length {
            decoded_string.push('.');
        };
        // println!("counter {}", counter);
    }

    decoded_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_label() {
        let input_string = "google.com".to_string();
        let expected_vec: Vec<u8> = vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0];
        assert_eq!(encode_label(&input_string), expected_vec);
    }

    #[test]
    fn test_decode_label() {
        let input_bytes = vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0];
        let expected_string = "google.com".to_string();
        assert_eq!(decode_label(&input_bytes), expected_string);
    }
}
