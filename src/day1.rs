pub fn stepper(steps :Vec<i64>) -> i64 {
    steps
    .iter()
    .skip(1)
    .zip(steps.iter())
    .map(|(current, prev)| if current > prev { 1 } else { 0 })
    // cramming a reduce here because it sounded fun
    .reduce(|acc, item| { acc + item}).unwrap_or(0)
}

pub fn stepper_window(steps: Vec<i64>) -> i64 {
    stepper(steps
    .iter()
    .skip(2)
    .zip(steps.iter().skip(1))
    .map(|(curr,prev)| curr + prev)
    .zip(steps.iter())
    .map(|(curr, prev)| curr + prev)
    .collect::<Vec<i64>>())
}

#[test]
fn test1() {
    assert_eq!(stepper(vec!{0,1}),1 );
}

#[test]
fn test2() {
    assert_eq!(stepper(vec!{0,1,0,2}),2 );
}
#[test]
fn test3() {
    assert_eq!(stepper(vec!{199, 200, 208, 210, 200, 207, 240, 269, 260, 263}), 7);
}


#[test]
fn test4() {
    assert_eq!(stepper_window(vec!{0,1,3,4}),1 );
}


#[test]
fn test5() {
    assert_eq!(stepper_window(vec!{199, 200, 208, 210, 200, 207, 240, 269, 260, 263}), 5);
}