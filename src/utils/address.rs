pub fn process_address(address_extract: &String) -> Option<String> {
    let address = address_extract.to_string();
    if address.len() == 42 && address.starts_with("sif") {
        return Some(address);
    }
    None
}

#[test]
fn process_address_valid() {
    let valid_address = "sif1nn5fwthfw2gdhvvyk3zynk0gjda8hsf5zrsqv7".to_string();
    let results = process_address(&valid_address);
    assert_eq!(results, Some(valid_address));
}

#[test]
fn process_address_invalid() {
    let invalid_address = "0x5CD948D2a94B25b03D5008074450ca24DCb4166F".to_string();
    let results = process_address(&invalid_address);
    assert_eq!(results, None)
}
