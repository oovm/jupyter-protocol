

### EDSL(Embedded Domain Specific Language)

我们首先使用枚举为我们的语言定义抽象语法树：

```rust
enum Expr {
    /// 整数类型
    Lit(i32),
    /// 取反操作
    Neg(Box<Expr>),
    /// 加法操作
    Add(Box<Expr>, Box<Expr>),
}
```

然后为我们的语言定义一些方便的构造函数, 至少可以少写点 `box`.

```rust
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
```

对于一个表达式, 比如说 `8 + -(1 + 2)`, 我们的抽象语法树可以表达为

```rust
Expr::add(
    Expr::lit(8),
    Expr::neg(Expr::add(Expr::lit(1), Expr::lit(2))),
)
```

我们应该如何来获得结果呢?

这很简单, 写一个 eval 解释器就可以了.

```rust
impl Expr {
    fn eval(&self) -> i32 {
        match self {
            Expr::Lit(i) => *i,
            Expr::Neg(r) => -r.eval(),
            Expr::Add(r1, r2) => r1.eval() + r2.eval(),
        }
    }
}
```

如果我们想从抽象语法树中获取原始的表达式, 又该怎么办?

这也很简单, 写一个 `to_text` 解释器就可以了.

```rust
impl Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Lit(i) => i.to_string(),
            Expr::Neg(r) => format!("(-{})", r.to_text()),
            Expr::Add(r1, r2) => format!("({} + {})", r1.to_text(), r2.to_text()),
        }
    }
}
```

这种风格叫做嵌入式领域特定语言, 也就是 EDSL.

就这? 这不有手就行.

---

好的, 如果你要计算 4294967296 + 4294967296, 会发现直接溢出了.

这是因为超出了 `i32` 的表示范围, 有没有办法可以自由的选择输出类型呢?

啊, 我们可以使用一个关联类型来表示输出, 这样在有需要的时候就可以使用大整数了.


```rust
trait ExprSym {
    type Repr;

    fn lit(i: Self::Repr) -> Self::Repr;
    fn neg(r: Self::Repr) -> Self::Repr;
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;
}
```

```rust
struct MyLang;

impl ExprSym for MyLang {
    type Repr = BigInt;

    fn lit(i: Self::Repr) -> Self::Repr {
        i
    }

    fn neg(r: Self::Repr) -> Self::Repr {
        -r
    }

    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        r1 + r2
    }
}

fn tf1<E: ExprSym>() -> E::Repr {
    E::add(E::lit(8), E::neg(E::add(E::lit(1), E::lit(2))))
}
```