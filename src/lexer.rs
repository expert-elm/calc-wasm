#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Number(String),
    Operator(Operator),
    Ident(String),
    Lparent,
    Rparent,
    Comma,
}

pub struct Lexer {
    source : Vec<char>,
    index  : usize,
    cache  : Option<Item>,
}

use self::Operator::*;

impl Lexer {
    pub fn new<'a>(text: &'a str) -> Self {
        let source: Vec<_> = text.chars().collect();
        let mut lexer = Lexer {
            source : source,
            index  : 0,
            cache  : None
        };
        lexer.next();
        lexer
    }

    #[inline(always)]
    fn now(&self) -> char {
        self.source[self.index]
    }

    #[inline(always)]
    fn end(&self) -> bool {
        self.index >= self.source.len()
    }

    #[inline(always)]
    fn inc(&mut self) {
        self.index += 1;
    }

    #[inline(always)]
    fn number(&mut self) -> Item {
        let mut result = format!("{}", self.now());
        self.inc();

        while !self.end() {
            let ch = self.now();
            match ch {
                '0'...'9' => result.push(ch),
                _ => break,
            }
            self.inc();
        }

        Item::Number(result)
    }

    #[inline(always)]
    fn blank(&mut self) -> Option<Item> {
        self.inc();
        while !self.end() {
            match self.now() {
                ' ' | '\r' | '\n' => {
                    self.inc();
                },
                _ => break
            }
        }
        self.next()
    }

    #[inline(always)]
    fn add(&mut self) -> Item {
        self.inc();
        Item::Operator(Add)
    }

    #[inline(always)]
    fn sub(&mut self) -> Item {
        self.inc();
        Item::Operator(Sub)
    }
    
    #[inline(always)]
    fn mul(&mut self) -> Item {
        self.inc();
        Item::Operator(Mul)
    }

    #[inline(always)]
    fn div(&mut self) -> Item {
        self.inc();
        Item::Operator(Div)
    }

    #[inline(always)]
    fn comma(&mut self) -> Item {
        self.inc();
        Item::Comma
    }

    #[inline(always)]
    fn lparent(&mut self) -> Item {
        self.inc();
        Item::Lparent
    }

    #[inline(always)]
    fn rparent(&mut self) -> Item {
        self.inc();
        Item::Rparent
    }

    #[inline(always)]
    fn ident(&mut self) -> Item {
        let mut result = format!("{}", self.now());
        self.inc();

        while !self.end() {
            let ch = self.now();
            match ch {
                'a'...'z' | 'A'...'Z' => result.push(ch),
                _ => break,
            }
            self.inc();
        }

        Item::Ident(result)
    }

    pub fn next(&mut self) -> Option<Item> {
        if !self.end() {
            let ch = self.now();
            let item = match ch {
                ' ' | '\r' | '\n' => return self.blank(),
                'a'...'z'  | 'A'...'Z' => self.ident(),
                '0'...'9' => self.number(),
                '+' => self.add(),
                '-' => self.sub(),
                '*' => self.mul(),
                '/' => self.div(),
                ',' => self.comma(),
                '(' => self.lparent(),
                ')' => self.rparent(),
                 _  => {
                    println!("{}", ch);
                    panic!("lexer");
                    // self.inc();
                    // return None;
                },
            };
            self.cache = Some(item.clone());
            Some(item)
        } else {
            self.cache = None;
            None
        }
    }

    #[inline(always)]
    pub fn current(&self) -> Option<Item> {
        self.cache.clone()
    }
}