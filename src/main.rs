fn main() {
    println!("Hello, world!");
}

fn two_plus_two(he: u32) -> u32 {
    let a = 32 + he;
    println!("jrjr {a}");
    a
}

#[cfg(test)]
mod te {
    use crate::two_plus_two;

    #[test]
    fn test_somewthing() {
        let res = two_plus_two(24);
        assert_eq!(res, 56);
    }
}
