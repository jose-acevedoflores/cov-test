
pub fn serve() {
    println!("Hello, server!");
    middleware(Type::Multi);
}

pub enum Type {
    Single,
    Multi,
    Extra,
    Another,
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
        Type::Another => {
            unreachable!();
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
}
