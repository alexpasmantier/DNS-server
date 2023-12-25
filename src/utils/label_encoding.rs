pub fn to_encoded_label(input_string: &String) -> Vec<u8> {
    let mut encoded_label: Vec<u8> = Vec::new();
    for word in input_string.split(".") {
        encoded_label.push(word.chars().count().try_into().expect("Word is too long"));
        encoded_label.extend(word.to_string().as_bytes());
    }
    encoded_label.push(0);
    encoded_label
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_encoded_label() {
        let input_string = "google.com".to_string();
        let expected_vec: Vec<u8> = vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0];
        assert_eq!(to_encoded_label(&input_string), expected_vec);
    }
}
