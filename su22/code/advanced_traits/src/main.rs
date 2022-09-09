mod fly {
    pub trait Pilot {
        fn fly(&self);
    }

    pub trait Wizard {
        fn fly(&self);
    }

    pub struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        pub fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
}
mod baby_name {
    pub trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
}

use crate::{fly::{Wizard,Pilot}};
fn main() {
    let person = fly::Human;
    fly::Pilot::fly(&person);
    fly::Wizard::fly(&person);
    fly::Human::fly(&person);
    person.fly();
}
