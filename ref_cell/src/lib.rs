use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: Rc<RefCell<Vec<String>>>,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: Rc::new(RefCell::new(Vec::new())),
            value: RefCell::new(0),
            max,
        }
    }
    pub fn set_value<T>(&self, new_val: &Rc<T>) {
        let count = Rc::strong_count(new_val);
        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
            return;
        }
        *self.value.borrow_mut() = count;
        let per = count * 100 / self.max;
        if per > 70 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                per
            ));
        }
    }
    pub fn peek<T>(&self, new_val: &Rc<T>) {
        let count = Rc::strong_count(new_val);
        let per = count * 100 / self.max;
        self.messages
            .borrow_mut()
            .push(format!("Info: This value would use {}% of your quota", per));
    }
}
