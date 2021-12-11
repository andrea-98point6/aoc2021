use std::collections::HashMap;
use std::thread;

pub fn spawns(start: i64, tot_days: i64) -> i64 {
    let mut tot = 0; 
    let offset_up = 6 - start;

    for gen in 0..=(tot_days / 7) {
        if gen == 0 {
            // how many will he s
            tot += (tot_days + offset_up - 7 * gen) / 7
        } else {
            tot += spawns(8, tot_days - 7 * gen + offset_up);
        }
    }
    tot
}

// to be honest, it is pretty late and I don't think there is much value in me learning
// how to solve this with math - thought I would liek it, BUT I do like the idea of caching
// and sharing that through various threads, so I will do that instead if that is ok with
// everyone
pub fn lanterns_with_cache(input: Vec<i64>, tot_days: i64) -> i64 {
    // prepare cache for this days
    let mut children = vec![];
    // there is probably also a better way to do this reusing the caches from other runs, but I am done noe
    // so really no incentive to keep going at this.
    for dis in 0..=6 {
        children.push(thread::spawn(move || -> (i64, i64) {
            // let dis = 6;
            let calculated = lanterns(vec![dis], tot_days);
            println!("for {:?} is {:?}", dis, calculated);
            (dis, calculated)
        }));
    }

    let mut cache = HashMap::new();
    children
        .into_iter()
        .map(|t| t.join().unwrap())
        .for_each(|(k, v)| {let _ = cache.insert(k, v);});
    input.into_iter().fold(0, |acc, r| acc + cache[&r])
    // for 6 is 3989468462
    // for 5 is 4368232009
    // for 4 is 4726100874
    // for 3 is 5217223242
    // for 2 is 5617089148
    // for 1 is 6206821033
    // for 0 is 6703087164
}

pub fn lanterns(input: Vec<i64>, tot_days: i64) -> i64 {

    let starter: i64 = input.clone().len().try_into().unwrap();
    input
        .clone()
        .into_iter()
        .fold(starter, |acc, fish| acc + spawns(fish, tot_days))
}

#[test]
pub fn test2() {
    let input = vec![3, 4, 3, 1, 2];
    assert_eq!(lanterns(input, 18), 26);
}

#[test]
pub fn test1() {
    let input = vec![3, 4, 3, 1, 2];
    assert_eq!(lanterns(input, 1), 5);
}
