pub fn get_consumption(input: Vec<String>) -> i32 {
    let size = input.first().unwrap().len().try_into().unwrap();

    let mut zero_counter: Vec<u32> = vec![0; size];
    let mut one_counter: Vec<u32> = vec![0; size];

    // count zeros and one for each location
    input.into_iter().for_each(|string| {
        string.chars().fold(0, |counter, this_bit| {
            if this_bit == '0' {
                zero_counter[counter] += 1;
            };
            if this_bit == '1' {
                one_counter[counter] += 1;
            };
            counter + 1
        });
    });

    // reduce
    let (_, epsi, gamma) = zero_counter
        .into_iter()
        .rev()
        .zip(one_counter.into_iter().rev())
        .fold((0, 0, 0), |(pow_2, epsilon, gamma), (zeros, ones)| {
            // the most frequent goes in gamma
            let base: i32 = 2;
            if ones > zeros {
                (pow_2 + 1, epsilon, gamma + base.pow(pow_2))
            } else {
                (pow_2 + 1, epsilon + base.pow(pow_2), gamma)
            }
        });
    epsi * gamma
}

#[test]
fn test1() {
    let input: Vec<String> = vec![
        "00100".into(),
        "11110".into(),
        "10110".into(),
        "10111".into(),
        "10101".into(),
        "01111".into(),
        "00111".into(),
        "11100".into(),
        "10000".into(),
        "11001".into(),
        "00010".into(),
        "01010".into(),
    ];

    assert_eq!(get_consumption(input), 198);
}
