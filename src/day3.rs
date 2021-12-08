pub fn get_consumption(input: Vec<String>) -> i32 {
    let size = input.first().unwrap().len().try_into().unwrap();

    let mut one_counter: Vec<i32> = vec![0; size];

    // count zeros and one for each location
    input.into_iter().for_each(|string| {
        string.chars().fold(0, |counter, this_bit| {
            if this_bit == '0' {
                one_counter[counter] -= 1;
            };
            if this_bit == '1' {
                one_counter[counter] += 1;
            };
            counter + 1
        });
    });

    // reduce
    let (_, epsi, gamma) =
        one_counter
            .into_iter()
            .rev()
            .fold((0, 0, 0), |(pow_2, epsilon, gamma), ones| {
                // the most frequent goes in gamma
                let base: i32 = 2;
                if ones > 0 {
                    (pow_2 + 1, epsilon, gamma + base.pow(pow_2))
                } else {
                    (pow_2 + 1, epsilon + base.pow(pow_2), gamma)
                }
            });
    epsi * gamma
}

// appreciate this code is not great as there is a lot of repetition, but my kid is sick and this is Xmas...
// and this is not going to go to production any time soon so... yeah
pub fn get_oxigen(input: Vec<String>, relevant_bit: usize) -> String {
    let zero_counts = input
        .clone()
        .into_iter()
        .map(|s| s.chars().nth(relevant_bit))
        .filter(|c| c == &Some('0'))
        .count();
    let mut wanted: char = '1';
    if zero_counts > input.len() / 2 {
        wanted = '0';
    };
    let still_in = input
        .into_iter()
        .filter(|dis| dis.chars().nth(relevant_bit) == Some(wanted))
        .collect::<Vec<String>>();
    if still_in.len() == 1 {
        return still_in.first().unwrap().to_string();
    };
    get_oxigen(still_in, relevant_bit + 1)
}

pub fn get_scrub(input: Vec<String>, relevant_bit: usize) -> String {
    let zero_counts = input
        .clone()
        .into_iter()
        .map(|s| s.chars().nth(relevant_bit))
        .filter(|c| c == &Some('0'))
        .count();
    let mut wanted: char = '0';
    if zero_counts > input.len() / 2 {
        wanted = '1';
    };
    let still_in = input
        .into_iter()
        .filter(|dis| dis.chars().nth(relevant_bit) == Some(wanted))
        .collect::<Vec<String>>();
    if still_in.len() == 1 {
        return still_in.first().unwrap().to_string();
    };
    get_scrub(still_in, relevant_bit + 1)
}

pub fn get_live(input: Vec<String>) -> i32 {
    let oxigen = get_oxigen(input.clone(), 0)
        .chars()
        .rev()
        .fold((0, 0), |(coll, pow_2), n| {
            if n == '1' {
                let base: i32 = 2;
                (coll + base.pow(pow_2), pow_2 + 1)
            } else {
                (coll, pow_2 + 1)
            }
        })
        .0;

    let scrub = get_scrub(input.clone(), 0)
        .chars()
        .rev()
        .fold((0, 0), |(coll, pow_2), n| {
            if n == '1' {
                let base: i32 = 2;
                (coll + base.pow(pow_2), pow_2 + 1)
            } else {
                (coll, pow_2 + 1)
            }
        })
        .0;

    scrub * oxigen
}

#[test]
fn test2() {
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
    assert_eq!(get_oxigen(input.clone(), 0), "10111");

    assert_eq!(get_scrub(input.clone(), 0), "01010");

    assert_eq!(get_live(input), 230);
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
