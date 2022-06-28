use std::{fmt, io::prelude::*};


#[derive(Debug)]
pub struct Counter {
    count: u32
}


impl Counter {
    pub fn new() -> Self {
        Self{
            count: 0
        }
    }
}


//then implement Iterator trait for the counter struct
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        //if the user count is less than 5 calling next should increase it by 1
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }else {
            None
        }
    }
}


// impl fmt::Display for Counter {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }



#[cfg(test)]
mod checkers {
    use super::*;


    #[test]
    fn counter_iterates() {
        //create a counter
        let mut counter = Counter::new();
        println!("{:#?}", counter);
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}