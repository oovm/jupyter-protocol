#![allow(unused, dead_code)]
use calculator::{Evaluator, Printer, SqrtAlgebra};

#[test]
fn ready() {
    println!("it works!")
}

fn parse<L: SqrtAlgebra>(input: &str) -> L::Tag<f64> {
    todo!()
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

#[test]
fn main() {
    let mut l1 = Printer {};
    let mut l2 = Evaluator::default();
    let t1 = program(&mut l1);
    println!("{:?}", t1);
    let t2 = program(&mut l2);
    println!("{:?}", t2);
}
