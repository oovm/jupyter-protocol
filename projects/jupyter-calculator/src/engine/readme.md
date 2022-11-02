

用 Visitor Pattern 来模拟 ADT.

## Embedded Domain Specific Language

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

---

## Final Expression Style

好的, 现在来计算 `4294967296 + 4294967296`, 你会发现直接就溢出了.

这是这因为超出了 `i32` 的表示范围, 那有没有办法可以自由的选择输出类型呢?

啊, 我们可以使用一个关联类型来表示输出, 这样在有需要的时候就可以使用大整数了.

```rust
trait ExprAlgebra {
    type Repr;

    fn lit(i: Self::Repr) -> Self::Repr;
    fn neg(r: Self::Repr) -> Self::Repr;
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;
}
```

接着我们可以这样定义 MyLang 的语法树:

```rust
struct MyLang;

impl ExprAlgebra for MyLang {
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
```

此时我们可以将 `8 + -(1 + 2)` 表达为

```rust
// E: ExprAlgebra
E::add(E::lit(8), E::neg(E::add(E::lit(1), E::lit(2))))
```

这样就可以自由的根据需要选择不同的输出类型了.

这被称为最终表达语言, 也就是 FES.

---

## Expression Problem

FP 与 OOP 之争的一个核心分歧就是如何处理表达难题(Expression Problem).

EP 问的是如何在不修改已有源代码的情况下, 同时增加新的数据类型和新的操作.

(因为定义的代码是别的人, 你可能改不了源代码, 或者许可证不允许.)

OOP 中新增数据类型很容易, 直接定义一个类继承接口就行了, 但是你没法改接口里的方法.

FP 则相反, 新增操作很容易, 直接定义一个新函数就行了, 但是你没法改已有的数据类型.

我们看到 FES 可以避免这个问题.

增加数据类型很简单

```rust
struct MyLangView;
impl ExprAlgebra for MyLangView {
    type Repr = String;

    fn lit(i: Self::Repr) -> Self::Repr {
        i.to_string()
    }

    fn neg(r: Self::Repr) -> Self::Repr {
        format!("(-{})", r)
    }

    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        format!("({} + {})", r1, r2)
    }
}
```

新增操作也很简单, 这里增加一个新操作比如乘法:

```rust
trait MulExprAlgebra: ExprAlgebra {
    fn mul(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;
}

impl ExprAlgebra for MyLang {
    fn mul(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        r1 * r2
    }
}
```

此时我们可以将 `8 + -(1 * 2)` 表达为

```rust
// E: MulExprAlgebra
E::add(E::lit(8), E::neg(E::mul(E::lit(1), E::lit(2))))
```

到此为止都很简单, 不涉及上下文的解释器到这里就足够了.

---

## Contextual Expression Style

对于一个正常的编程语言, 变量是必不可少的, 我们先以 EDSL 形式定义一个语言:

```rust
use std::rc::Rc;
type VarId = &'static str;

enum Expr {
    /// 变量类型
    Var(VarId),
    /// 凉凉的值
    Int(i32),
    // 函数类型
    Lam(Rc<Expr>),
    // 函数调用
    App(Rc<Expr>, Rc<Expr>),
}
```

理解一下这个语法树, 比如说表达式 `(\x -> x)(1)` 可以解析为

```rust
Expr::app(
    Expr::lam(Expr::var("x")),
    Expr::int(1)
)
```

现在，如果为我们的语言定义一个解释器, 就会遇到一些上一节中的简单语言中没有遇到的复杂情况。

首先，可以编写无意义的表达式, 此时结果类型应该是个什么?

```rust
// 1(2)
Expr::app(
    Expr::int(1),
    Expr::int(2),
)

第二, 我们怎么知道变量或者函数的值是多少?

那我们首先要写一个上下文环境

```rust
type Env = Vec<Val>;

enum Val {
    Int(i32),
    Fun(Rc<dyn Fn(Val) -> Val>),
}

impl Expr {
    fn eval(expr: Expr, env: Env) -> Val {
        todo!()
    }
}
```

当然如果我们能绑定新的变量应该是 `&mut Env`, 简单起见先不考虑这个.

然后我们的 eval 解释器应该写成这个形式.

```rust
fn eval(expr: Expr, env: Env) -> Val {
    match expr {
        Expr::Var(id) => env[id].clone(),
        Expr::Int(i) => Val::Int(i),
        Expr::Lam(e) => Val::Fun(Rc::new(move |x| {
            let mut envr = env.clone();
            envr.push(x);
            Expr::eval(envr, e.as_ref().clone())
        })),

        Expr::App(e1, e2) => {
            let eval_e1 = Expr::eval(env.clone(), e1.as_ref().clone());
            let eval_e2 = Expr::eval(env, e2.as_ref().clone());
            match eval_e1 {
                Val::Fun(f) => f(eval_e2),
                _ => panic!("Expected function"),
            }
        }
    }
}
```

这里有两个地方会 panic, 找不到变量或者函数会 panic, 非法 apply 会 panic.

如果我们想让我们的语言类型安全，我们似乎必须自己实现一个类型检查器，将我们的语言变成简单类型的 lambda 演算。

然后重新发明一整套轮子.

---


# Finally Higher Order Style

这也太傻了, 考虑到 Rust 本身已经有一个强大的类型系统, 可以在编译期为 Rust 预防这些错误。

有没有办法来直接把 Rust 类型系统嵌入我们的语言和解释器?

有, 这就是最终高阶表达形式 FHOS.

何为高阶?

简单的说, 如果是用 ADT 表示, 那么我们说这是 0 阶表达.

如果是用 FES(trait) 表示, 那么我们说这是 1 阶表达.

更高阶的表达, 在 Rust 中我们要用到 GAT, 在其他语言中则可以使用 HKT 或者 GADT.

我们直接给 Repr 增加一个泛型参数 T, 得到一个 2 阶表达

```rust
type Func<A, B> = Box<dyn Fn(A) -> B>;

trait ExprAlgebra {
    type Repr<T>;

    fn int(i: i32) -> Self::Repr<i32>;
    fn add(a: &Self::Repr<i32>, b: &Self::Repr<i32>) -> Self::Repr<i32>;
    fn lam<A, B, F: Fn(Self::Repr<A>) -> Self::Repr<B>>(f: F) -> Self::Repr<Func<A, B>>
    where
        for<'a> F: 'a;
    fn app<F: Fn(A) -> B, A, B>(f: Self::Repr<F>, arg: Self::Repr<A>) -> Self::Repr<B>;
}
```

此时你再写 `1(2)` 会发现压根无法编译.

```rust
fn test<T, E: ExprAlgebra>() -> E::Repr<T> {
    E::app(E::int(2), E::int(3))
}
```

![img.png](img.png)

到这里我们就成功的把 rust 类型系统嵌入到了我们的语言中.

接着我们可以这样写一个简单的 eval 解释器.

```rust
struct Eval;
impl ExprAlgebra for Eval {
    type Repr<T> = T;

    fn int(i: i32) -> Self::Repr<i32> {
        i
    }

    fn add(a: &Self::Repr<i32>, b: &Self::Repr<i32>) -> Self::Repr<i32> {
        a + b
    }

    fn lam<A, B, F: Fn(Self::Repr<A>) -> Self::Repr<B>>(f: F) -> Self::Repr<Box<dyn Fn(A) -> B>>
    where
        for<'a> F: 'a,
    {
        Box::new(f)
    }

    fn app<F: Fn(A) -> B, A, B>(f: Self::Repr<F>, arg: Self::Repr<A>) -> Self::Repr<B> {
        f(arg)
    }
}
```

