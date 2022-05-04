
pub fn serve() {
    println!("Hello, server!");
    middleware(Type::Multi);
}

pub enum Type {
    Single,
    Multi,
    Extra,
    Another(u32),
}

fn another(u: u32) -> Result<String, String> {
    if u == 0 {
        Err("bad".to_string())
    } else if u == 23 {
        Ok("good".to_string())
    } else {
        Ok("unk".to_string())
    }
}

fn middleware(x: Type) {

    match x {
        Type::Single => {
            println!("data single");
        }
        Type::Multi => {
            println!("data multi");
        }
        Type::Extra => {
            panic!("dead jim");
        }
        Type::Another(u) => {
            another(u).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iserve::{middleware, serve, Type};

    #[test]
    fn run_serve() {
        serve();
    }

    #[test]
    #[should_panic(expected = "dead jim")]
    fn run_internal() {
        middleware(Type::Extra);
    }

    #[test]
    #[should_panic]
    fn run_internal2() {
        middleware(Type::Another(0));
    }
}
