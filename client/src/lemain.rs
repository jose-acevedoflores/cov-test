pub fn entry_point() {
    println!("Hello, client!");
    let res = internal(34);
    println!("result {res}");
}

fn internal(i: i32) -> String {
    if i < 100 {
        format!("low end {i}")
    } else if i == 1010 {
        format!("exact end")
    } else {
        format!("above and beyond {i}")
    }
}

#[cfg(test)]
mod tests {
    use crate::lemain::{entry_point, internal};

    #[test]
    fn run_entry_point() {
        entry_point()
    }

    #[test]
    fn run_internal() {
        let res = internal(214);
        assert_eq!(res, "above and beyond 214");
    }
}