use crate::lexer::{Lexer, Item, Operator};

#[derive(Debug)]
struct Infix {
    value : Operator,
    left  : Box<Expr>,
    right : Box<Expr>,
}

impl Infix {
    fn new(value: Operator, left: Expr, right: Expr) -> Self {
        Infix {
            value ,
            left  : Box::new(left),
            right : Box::new(right)
        }
    }
}

type Number = f64;

#[derive(Debug)]
enum Expr {
    Literal(String),
    Operator(Infix),
    Eval(String, Vec<Box<Expr>>),
}

fn calc(infix: Infix) -> Number {
    use self::Operator::*;

    let left  = infix.left.eval();
    let right = infix.right.eval();

    match infix.value {
        Add => left + right,
        Sub => left - right,
        Mul => left * right,
        Div => left / right,
    }
}

fn method(name: String, mut params: Vec<Box<Expr>>) -> Number {
    if name == "sin" {
        if params.len() > 1 { panic!("sin params invalid") }
        let radians = params.pop().unwrap().eval();
        let value = radians * std::f64::consts::PI / 180.0;
        return value.sin();
    }

    // pass
    params.into_iter().map(|n| n.eval()).sum::<Number>();
    println!("\"{}\" method is undefined, so it's value is 0", name);
    println!("---");
    0.0
}

impl Expr {
    fn eval(self) -> Number {
        use self::Expr::*;
        match self {
            Literal(val)       => val.parse::<Number>().unwrap(),
            Operator(infix)    => calc(infix),
            Eval(name, params) => method(name, params),
        }
    }
}

pub struct Parser(Lexer);

impl Parser {
    pub fn new<'a>(text: &'a str) -> Self {
        Parser(Lexer::new(text))
    }

    #[inline(always)]
    fn block(&mut self) -> Expr {
        self.0.next();
        let expr = self.next().unwrap();
        self.expect(Item::Rparent);
        expr
    }

    #[inline(always)]
    fn attempt(&mut self, mut expr: Expr) -> Expr {
        use self::Operator::*;
        while let Some(Item::Operator(op)) = self.0.current() {
            match op {
                Mul | Div => return self.prior_operator(expr),
                _ => {
                    self.0.next();
                    let value = self.value().unwrap();
                    let right = self.prior_operator(value);
                    expr = Expr::Operator(Infix::new(op, expr, right));
                }
            }
        }
        expr
    }

    #[inline(always)]
    fn prior_operator(&mut self, mut expr: Expr) -> Expr {
        use self::Operator::*;
        while let Some(Item::Operator(op)) = self.0.current() {
            match op {
                Mul | Div => {
                    self.0.next();
                    let right = self.value().unwrap();
                    expr = Expr::Operator(Infix::new(op, expr, right));
                },
                _ => break
            }
        }
        expr
    }

    #[inline(always)]
    fn value(&mut self) -> Option<Expr> {
        use self::Expr::*;
        let item = self.0.current()?;
        let result = match item {
            Item::Number(val) => {
                self.0.next();
                Literal(val)
            },
            Item::Ident(name) => self.eval(name),
            Item::Lparent => self.block(),
            _ => return None,
        };
        Some(result)
    }

    #[inline(always)]
    fn expect(&mut self, target: Item) {
        if self.0.current() != Some(target) {
            panic!("not expect");
        } else {
            self.0.next();
        }
    }

    #[inline(always)]
    fn eval(&mut self, name: String) -> Expr {
        self.0.next();
        let mut params = Vec::new();

        self.expect(Item::Lparent);
        if let Some(item) = self.0.current() {
            if item != Item::Rparent {
                params.push(Box::new(self.next().unwrap()));
                while let Some(Item::Comma) = self.0.current() {
                    self.0.next();
                    params.push(Box::new(self.next().unwrap()));
                }
            }
        }
        self.expect(Item::Rparent);
        self.attempt(Expr::Eval(name, params))
    }

    #[inline(always)]
    fn next(&mut self) -> Option<Expr> {
        use self::Expr::*;
        if let Some(item) = self.0.current() {
            Some(match item {
                Item::Number(val) => {
                    self.0.next();
                    self.attempt(Literal(val))
                },
                Item::Operator(_op) => {
                    panic!("need a expression");
                },
                Item::Ident(name) => self.eval(name),
                Item::Lparent => self.block(),
                Item::Rparent => {
                    panic!("need left parent");
                },
                Item::Comma => {
                    panic!("comma only use in method");
                },
            })
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Number {
        if let Some(expr) = self.next() {
            println!("AST: {:?}", expr);
            println!("---");
            expr.eval()
        } else {
            0.0
        }
    }
}