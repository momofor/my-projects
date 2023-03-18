fn main() {
    let data: Vec<_> = include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .map(|b| (b[0] - b'A', b[2] - b'X'))
        .map(|(opponent_hand, my_hand)| {
            //  score of shape   it loops around
            println!(
                "{}",
                (1 + my_hand) + 3 * ((my_hand - opponent_hand) + 1).rem_euclid(3)
            )
        })
        .collect();
}
