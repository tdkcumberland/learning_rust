/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut empty = create_empty();
    for n in 0..count {
        empty.push(0);
    }
    empty
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut buffer = create_buffer(5);
    for n in 0..5 {
        if n == 0 || n == 1 {
            buffer[n] = 1;
        } else {
            buffer[n] = buffer[n-2] + buffer[n-1];
        }
    }
    buffer
}
