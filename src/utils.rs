pub fn split_into_pairs(number: i32) -> Vec<i32> {
    let number_str = number.to_string(); // Convertir a string
    let mut pairs = Vec::new();

    // Iterar por cada par de caracteres
    for chunk in number_str.as_bytes().chunks(1) {
        let pair_str = String::from_utf8_lossy(chunk); // Convertir el slice en string
        let pair = pair_str.parse::<i32>().unwrap(); // Convertir el par a i32
        pairs.push(pair);
    }

    pairs
}