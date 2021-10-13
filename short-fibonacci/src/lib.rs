/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut result = create_buffer(5);
    for i in 0..5 {
        result[i] = if let Some(n) = i.checked_sub(1) {
            result[n]
        } else {
            1
        } + if let Some(n) = i.checked_sub(2) {
            result[n]
        } else {
            0
        }
    }
    result
}
