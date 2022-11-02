use calculator::{Evaluator, Printer, SqrtAlgebra};

#[test]
fn ready() {
    println!("it works!")
}

fn program<L>(interpreter: &mut L) -> L::Tag<f64>
where
    L: SqrtAlgebra,
{
    let lhs = interpreter.integer(2);
    let rhs = interpreter.integer(3);
    let o1 = interpreter.plus(lhs, rhs);
    interpreter.sqrt(o1)
}

fn parse<L: SqrtAlgebra>(input: &str) -> L::Tag<f64> {
    todo!()
}

#[test]
fn main() {
    let mut l1 = Printer {};
    let mut l2 = Evaluator::default();

    let test1_p = program(&mut l1);

    println!("{:?}", test1_p);
    let test1_b = program(&mut l2);
    println!("{:?}", test1_b);
}
