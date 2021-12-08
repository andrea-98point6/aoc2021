use std::collections::HashMap;

// represent board as a HashMap<String, i32>
// the key is x_y
// the value is the number of roads
//when done need to traverse all keys and cound where i32 >=2

pub struct Board {
    pub state: HashMap<String,i32>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn mark_this_route(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32) {
        if x_start == x_end {
            let start = std::cmp::min(y_end, y_start);
            let end = std::cmp::max(y_end, y_start);
            (start..=end).into_iter().for_each(|y| self.mark_this_point(x_start, y));
        } else if y_start == y_end {
            let start = std::cmp::min(x_end, x_start);
            let end = std::cmp::max(x_start, x_end);
            (start..=end).into_iter().for_each(|x| self.mark_this_point(x, y_start))
        } else {
            // panic!("something is wrong with input found {}, {}, {}, {}", x_start, y_start, x_end, y_end);
        }
    }

    pub fn mark_this_route_diag(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32) {
        let y_min = std::cmp::min(y_end, y_start);
        let y_max = std::cmp::max(y_end, y_start);
        let x_min = std::cmp::min(x_end, x_start);
        let x_max = std::cmp::max(x_start, x_end);
        if x_start == x_end {
            (y_min..=y_max).into_iter().for_each(|y| self.mark_this_point(x_start, y));
        } else if y_start == y_end {
            (x_min..=x_max).into_iter().for_each(|x| self.mark_this_point(x, y_start))
        } else {
            let mut y_increments = 1;
            if y_end < y_start {
                y_increments = -1;
            }
            let mut x_increment = 1;
            if x_end < x_start {
                x_increment = -1;
            }
            let mut x_t = x_start;
            let mut y_t = y_start;
            while (x_min..=x_max).contains(&x_t) && (y_min..=y_max).contains(&y_t) {
                self.mark_this_point(x_t, y_t);
                x_t += x_increment;
                y_t += y_increments;
            }

            // panic!("something is wrong with input found {}, {}, {}, {}", x_start, y_start, x_end, y_end);
        }
    }

    fn mark_this_point(&mut self, x: i32, y: i32) {
        let key = Board::coordinate_to_string(x, y);
        if self.state.contains_key(&key) {
            self.state.insert(key.clone(), self.state[&key] +1);
        } else  {
            self.state.insert(key, 1);
        }
    }

    fn coordinate_to_string(x: i32, y: i32) -> String {
        format!("{}_{}", x, y)
    }

    pub fn count(self) -> i32 {
        let mut c = 0;
        for (_, value) in self.state {
            if value > 1 {
                c +=1;
            }
        }
        c
    }

}


#[test]
fn test1() {
    let mut sut = Board::new();
    sut.mark_this_point(1, 1);
    sut.mark_this_point(1, 1);
    assert!(sut.state.contains_key("1_1"));
    assert_eq!(sut.state["1_1"], 2);
}

#[test]
fn test2() {
    let mut sut = Board::new();
    sut.mark_this_route(3,3,3,3);
    sut.mark_this_route(3, 3, 3, 0);
    assert_eq!(sut.state["3_3"], 2);
    assert_eq!(sut.state["3_2"],1);
    assert_eq!(sut.state["3_1"],1);
    assert_eq!(sut.state["3_0"],1);
    assert_eq!(sut.state.keys().len(), 4);
}

#[test]
fn test3() {
    let mut sut = Board::new();
    sut.mark_this_route(0,9,5,9);
    sut.mark_this_route(8,0,0,8);
    sut.mark_this_route(9,4,3,4);
    sut.mark_this_route(2,2,2,1);
    sut.mark_this_route(7,0,7,4);
    sut.mark_this_route(6,4,2,0);
    sut.mark_this_route(0,9,2,9);
    sut.mark_this_route(3,4,1,4);
    sut.mark_this_route(0,0,8,8);
    sut.mark_this_route(5,5,8,2);
    let output = sut.count();
    assert_eq!(output, 5);

}

#[test]
fn test4() {
    let mut sut = Board::new();
    sut.mark_this_route_diag(0,9,5,9);
    sut.mark_this_route_diag(8,0,0,8);
    sut.mark_this_route_diag(9,4,3,4);
    sut.mark_this_route_diag(2,2,2,1);
    sut.mark_this_route_diag(7,0,7,4);
    sut.mark_this_route_diag(6,4,2,0);
    sut.mark_this_route_diag(0,9,2,9);
    sut.mark_this_route_diag(3,4,1,4);
    sut.mark_this_route_diag(0,0,8,8);
    sut.mark_this_route_diag(5,5,8,2);
    let output = sut.count();
    assert_eq!(output, 12);

}
