pub struct School {
    fishes: Vec<i32>,
    days_passed: i32,
}

impl School {
    pub fn new(fishes: Vec<i32>) -> Self {
        Self {
            fishes,
            days_passed: 0,
        }
    }

    pub fn add_days(mut self, d: i32) -> Self {
        for _ in 0..d {
            self = self.add_a_day();
        }
        self
    }

    pub fn add_a_day(mut self) -> Self {
        self.fishes = self
            .fishes
            .into_iter()
            .map(|f| School::next(f))
            .flatten()
            .collect::<Vec<i32>>();
        self.days_passed += 1;
        self
    }

    pub fn next(fish_status: i32) -> Vec<i32> {
        if fish_status == 0 {
            vec![6, 8]
        } else {
            vec![fish_status - 1]
        }
    }

    pub fn fishes(self) -> usize {
        self.fishes.len()
    }
}

#[test]
fn test1() {
    let init = vec![3, 4, 3, 1, 2];
    let mut sut = School::new(init);
    sut = sut.add_days(18);
    assert_eq!(sut.fishes.len(), 26)
}

#[test]
fn test2() {
    let init = vec![3, 4, 3, 1, 2];
    let mut sut = School::new(init);
    sut = sut.add_days(80);
    assert_eq!(sut.fishes.len(), 5934)
}
