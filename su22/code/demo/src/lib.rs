use std::i8;

fn demo() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}",top);
    }
    let mut matcher : Option<String> = Some(String::from("none"));
    match matcher {
        Some(some) => some,
        _ => String::from("none"),
    };

    matcher = Some(String::new());
}

#[cfg(test)]
mod test {
    use crate::demo;

    #[test]
    fn test_demo(){
        demo();
    }
}