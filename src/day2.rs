pub fn move_sub(moves: Vec<(String, i32)>) -> i32 {
    let tot_down: i32 = sum_for_move(&moves, "down");
    let tot_up: i32 = sum_for_move(&moves, "up");
    let tot_for = sum_for_move(&moves, "forward");
    (tot_down - tot_up) * tot_for
}

fn sum_for_move(moves: &Vec<(String, i32)>, target: &str) -> i32 {
    moves
        .into_iter()
        .filter(|(s, _m)| s.contains(target))
        .map(|(_s, m)| m)
        .sum()
}

#[test]
fn test1() {
    let input = vec![
        ("forward".into(), 5),
        ("down".into(), 5),
        ("forward".into(), 8),
        ("up".into(), 3),
        ("down".into(), 8),
        ("forward".into(), 2),
    ];
    assert_eq!(move_sub(input), 150)
}

pub fn move_sub_better(moves: Vec<(String, i32)>) -> i32 {
    // 0 is depth
    // 1 is horizontal
    // 2 is aim
    let res = moves.into_iter().fold((0, 0, 0), |mut acc, cur_move| {
        if cur_move.0.contains("up") {
            acc.2 -= cur_move.1;
        } else if cur_move.0.contains("down") {
            acc.2 += cur_move.1;
        } else if cur_move.0.contains("forward") {
            acc.1 += cur_move.1;
            acc.0 += acc.2 * cur_move.1;
        };
        acc
    });
    res.0 * res.1
}

#[test]
fn test2() {
    let input = vec![
        ("forward".into(), 5),
        ("down".into(), 5),
        ("forward".into(), 8),
        ("up".into(), 3),
        ("down".into(), 8),
        ("forward".into(), 2),
    ];
    assert_eq!(move_sub_better(input), 900)
}
