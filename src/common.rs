pub(super) fn decoded_len(input: usize) -> usize {
    let mod4 = input % 4;
    input / 4 * 3 + (mod4 - mod4 / 2)
}

fn encoded_len(input: usize) -> usize {
    let mod3 = input % 3;
    input / 3 * 4 + (mod3 + (mod3 + 1) / 2)
}
