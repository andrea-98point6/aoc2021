// process one card at a time?

// represent each card with a struct
// have a method that calls a number and returns Option<i32>

pub fn find_bingo(mut numbers: Vec<i32>, called: Vec<i32>) -> Option<(i32, i32)> {
    let mut result = -1;
    let mut i = 0;

    for c in called {
        i += 1;
        let (res, next) = call_number(numbers.clone(), c);
        if let Some(r) = res {
            result = r;
            break;
        } else {
            numbers = next;
        }
    }
    if result != -1 {
        Some((i, result))
    } else {
        None
    }
}

pub fn call_number(mut numbers: Vec<i32>, called: i32) -> (Option<i32>, Vec<i32>) {
    // mark called as -1
    for i in 0..numbers.len() {
        if numbers[i] == called {
            numbers[i] = -1;
        }
    }

    let hor_win = vec![0, 5, 10, 15, 20].iter().any(|start| {
        for i in 0..5 {
            if numbers[start + i] != -1 {
                return false;
            };
        }
        return true;
    });

    let ver_win = vec![0, 1, 2, 3, 4].iter().any(|start| {
        for i in 0..5 {
            if numbers[start + i * 5] != -1 {
                return false;
            }
        }
        return true;
    });

    if ver_win || hor_win {
        (
            Some(calculate_win(numbers.clone(), called)),
            numbers.clone(),
        )
    } else {
        (None, numbers.clone())
    }
}

fn calculate_win(numbers: Vec<i32>, called: i32) -> i32 {
    let sum = numbers
        .iter()
        .filter(|s| **s != -1)
        .fold(0, |acc, n| acc + n);

    sum * called
}

// restart from checking the code can find a winning card (returns 23 for the time being =)
#[test]
fn test1() {
    let numbers = vec![
        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19,
    ];

    assert_eq!(call_number(numbers, 7).0, None);
}

#[test]
fn test2() {
    // let mut numbers = vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16,  7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19 ];
    let mut numbers = vec![
        14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
    ];

    let called = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    let mut result = -1;

    for c in called {
        let (res, next) = call_number(numbers.clone(), c);
        if let Some(r) = res {
            result = r;
            break;
        } else {
            numbers = next;
        }
    }

    assert_eq!(result, 4512);
}

#[test]
fn test3() {
    let other_numbers = vec![
        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19,
    ];
    let numbers = vec![
        14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
    ];
    let more_numbers = vec![
        14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
    ];

    let called = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    let cards = vec![other_numbers, numbers, more_numbers];
    let mut result = 0;
    let mut min = 700;

    // sad way to find the first winner (if it is the last one, you end up doing a lot)
    for card in cards {
        if let Some((moves, result_t)) = find_bingo(card.clone(), called.clone()) {
            if moves < min {
                min = moves;
                result = result_t;
            }
        }
    }

    assert_eq!(result, 4512);
}
