pub fn recite(start_bottles: u32, take_down: u32) -> String {
    const BOTTLE_NAMES: [&str; 11] = [
        "No green bottles",
        "One green bottle",
        "Two green bottles",
        "Three green bottles",
        "Four green bottles",
        "Five green bottles",
        "Six green bottles",
        "Seven green bottles",
        "Eight green bottles",
        "Nine green bottles",
        "Ten green bottles",
    ];
    (0..take_down)
        .map(|i| {
            format!(
                "{0} hanging on the wall,\n\
            {0} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {1} hanging on the wall.",
                BOTTLE_NAMES[(start_bottles - i) as usize],
                BOTTLE_NAMES[(start_bottles - i - 1) as usize].to_lowercase(),
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}
