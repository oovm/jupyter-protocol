enum Expr {
    /// 整数类型
    Lit(i32),
    /// 取反操作
    Neg(Box<Expr>),
    /// 加法操作
    Add(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn lit(i: i32) -> Expr {
        Expr::Lit(i)
    }

    fn neg(r: Expr) -> Expr {
        Expr::Neg(Box::new(r))
    }

    fn add(r1: Expr, r2: Expr) -> Expr {
        Expr::Add(Box::new(r1), Box::new(r2))
    }
}
