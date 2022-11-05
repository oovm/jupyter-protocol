use num::traits::FloatErrorKind;

pub trait ElementaryAlgebra {
    type Tag<T>;
    fn integer(&mut self, v: i64) -> Self::Tag<f64>;
    fn plus(&mut self, lhs: Self::Tag<f64>, rhs: Self::Tag<f64>) -> Self::Tag<f64>;
    fn minus(&mut self, lhs: Self::Tag<f64>, rhs: Self::Tag<f64>) -> Self::Tag<f64>;
    fn times(&mut self, lhs: Self::Tag<f64>, rhs: Self::Tag<f64>) -> Self::Tag<f64>;
    fn divide(&mut self, lhs: Self::Tag<f64>, rhs: Self::Tag<f64>) -> Self::Tag<f64>;
}

#[derive(Clone, Debug, Default)]
pub struct Evaluator {
    count: usize,
}

impl ElementaryAlgebra for Evaluator {
    type Tag<T> = Result<f64, FloatErrorKind>;

    fn integer(&mut self, v: i64) -> Self::Tag<f64> {
        Ok(v as f64)
    }

    fn plus(&mut self, b1: Self::Tag<f64>, b2: Self::Tag<f64>) -> Self::Tag<f64> {
        Ok(b1? + b2?)
    }

    fn minus(&mut self, b1: Self::Tag<f64>, b2: Self::Tag<f64>) -> Self::Tag<f64> {
        Ok(b1? - b2?)
    }

    fn times(&mut self, b1: Self::Tag<f64>, b2: Self::Tag<f64>) -> Self::Tag<f64> {
        Ok(b1? * b2?)
    }

    fn divide(&mut self, b1: Self::Tag<f64>, b2: Self::Tag<f64>) -> Self::Tag<f64> {
        Ok(b1? / b2?)
    }
}

pub struct Printer;

impl ElementaryAlgebra for Printer {
    type Tag<T> = String;

    fn integer(&mut self, v: i64) -> Self::Tag<f64> {
        format!("{}", v)
    }

    fn plus(&mut self, b1: Self::Tag<f64>, b2: Self::Tag<f64>) -> Self::Tag<f64> {
        format!("({} + {})", b1, b2)
    }

    fn minus(&mut self, b1: Self::Tag<f64>, b2: Self::Tag<f64>) -> Self::Tag<f64> {
        format!("({} - {})", b1, b2)
    }

    fn times(&mut self, b1: Self::Tag<f64>, b2: Self::Tag<f64>) -> Self::Tag<f64> {
        format!("({} * {})", b1, b2)
    }

    fn divide(&mut self, b1: Self::Tag<f64>, b2: Self::Tag<f64>) -> Self::Tag<f64> {
        format!("({} / {})", b1, b2)
    }
}

pub trait SqrtAlgebra: ElementaryAlgebra {
    fn sqrt(&mut self, b: Self::Tag<f64>) -> Self::Tag<f64>;
}

impl SqrtAlgebra for Evaluator {
    fn sqrt(&mut self, b: Self::Tag<f64>) -> Self::Tag<f64> {
        Ok(b?.sqrt())
    }
}

impl SqrtAlgebra for Printer {
    fn sqrt(&mut self, b: Self::Tag<f64>) -> Self::Tag<f64> {
        format!("sqrt({})", b)
    }
}
