use std::rc::Rc;
use std::cell::RefCell;


pub struct Tracker {
    pub messages: Rc<RefCell<Vec<String>>>,
    pub value: RefCell<usize>,
    pub max:usize,
}

impl Tracker {
    pub fn new(max:usize) -> Self {
        Tracker{
            messages: Rc::new(RefCell::new(Vec::new())),
            value:RefCell::new(0),
            max,
        }
    }
    pub fn set_value(&self, new_val:&Rc<usize>) {
        // let count = Rc::strong_count(new_val);
        // if count > self.max {
            // xxxx & peek
        // }

    }
    // pub fn peek(&self) {
    // }
}