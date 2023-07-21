use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
    sync::Mutex,
};

pub type Value = i16;
type Stack = Vec<Value>;
type Result = std::result::Result<(), Error>;
type Operation = Rc<dyn Fn(&mut Forth, Option<&str>) -> Result>;
type Operations = HashMap<String, Operation>;

pub struct Forth {
    eval_stack: Stack,
    dict: Operations,
}

macro_rules! call {
    ($forth:expr, $name:literal) => {
        $forth.call($name, None)?
    };
    ($forth:expr, $($name:literal),+) => {
        $($forth.call($name, None)?;)+
    };
    ($forth:expr, ($name:literal, $val:literal)) => {
        $forth.call($name, Some($val))?
    };
}

macro_rules! install {
    ($forth:expr, $($name:expr => $op:expr$(;)?)+) => {
        $(
            $forth.add_operation($name, Rc::new(move |forth, val| {
                $op(forth, val)
            }));
        )+
    };
}

macro_rules! replace {
    ($forth:expr, $($name:literal -> $(($new:literal, $val:expr))+$(;)?)+) => {{
        $(
            let maybe_existing = Rc::new(Mutex::new($forth.dict.remove($name))).clone();
            let a = Rc::new(move |forth: &mut Forth, _: Option<&str>| {
                if let Some(existing) = maybe_existing.lock().unwrap().as_ref() {
                    let existing = existing.clone();
                    forth.add_operation(format!("{}_old",$name).as_str(), Rc::new(move |forth, val| existing(forth, val)));
                }
                $(forth.call({if $new == $name {format!("{}_old",$name)} else {$new.to_string()}}.as_str(), Some($val))?;)+
                Ok(())
            });

            $forth.dict
            .entry($name.into())
            .and_modify(|e| *e = a.clone())
            .or_insert(a)
        )+
    }};
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    fn init() -> Self {
        Self {
            eval_stack: Vec::new(),
            dict: Operations::new(),
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.eval_stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let n = self;

        let x = "4";
        install!(n, "foo" => |forth: &mut Forth, _| {
            forth.call("_VAL", Some(x))
        });
        call!(n, "foo");

        replace!(n, "foo" -> ("DUP", "")("_VAL", "1")("+", ""));
        call!(n, "foo");

        call!(n, "foo");

        replace!(n, "foo" -> ("foo", "")("_VAL", "1")("+", ""));
        call!(n, "foo");

        dbg!(&n.eval_stack);
        Ok(())
    }

    fn add_operation(&mut self, name: &str, op: Operation) {
        self.dict.insert(name.to_string(), op);
    }

    fn call(&mut self, name: &str, val: Option<&str>) -> Result {
        let op = self.dict.remove(name);
        match op {
            Some(op) => {
                let result = op(self, val);
                self.dict.insert(name.to_string(), op);
                result
            }
            None => Err(Error::UnknownWord),
        }
    }
}

impl Default for Forth {
    fn default() -> Self {
        let mut new = Self::init();

        install!(new,
            "DROP" => |forth: &mut Forth, _| {
                let stack = &mut forth.eval_stack;
                match stack.pop() {
                    Some(_) => Ok(()),
                    None => Err(Error::StackUnderflow),
                }
            };

            "DUP" => |forth: &mut Forth, _| {
                let stack = &mut forth.eval_stack;
                match stack.last() {
                    Some(last) => {stack.push(*last); Ok(())},
                    None => Err(Error::StackUnderflow),
                }
            };

            "SWAP" => |forth: &mut Forth, _| {
                let stack = &mut forth.eval_stack;
                let a = stack.pop();
                let b = stack.pop();
                match (a, b) {
                    (Some(first), Some(last)) => {stack.extend([last, first]); Ok(())},
                    _ => Err(Error::StackUnderflow)
                }
            };

            "OVER" => |forth: &mut Forth, _| {
                let stack = &mut forth.eval_stack;
                if stack.len() < 2 {
                    return Err(Error::StackUnderflow)
                }
                match stack.get(stack.len() - 2) {
                    Some(over) => {stack.push(*over); Ok(())},
                    None => Err(Error::StackUnderflow),
                }
            };

            "_ARITH" => |forth: &mut Forth, op: Option<&str>| {
                let stack = &mut forth.eval_stack;
                let a = stack                .pop();
                let b = stack.pop();
                match (a, b) {
                    (Some(first), Some(last)) => {
                        match op.unwrap() {
                            "/" if last == 0 => {return Err(Error::DivisionByZero)},
                            "/" => stack.push(last.div(first)),
                            "+" => stack.push(last.add(first)),
                            "-" => stack.push(last.sub(first)),
                            "*" => stack.push(last.mul(first)),
                            _ => {return Err(Error::UnknownWord)}
                        };
                        Ok(())
                    },
                    _ => Err(Error::StackUnderflow)
                }
            };

            "_VAL" => |forth: &mut Forth, val: Option<&str>| {
                let stack = &mut forth.eval_stack;
                if let Some(val) = val {
                    match val.parse::<Value>() {
                        Ok(val) => {stack.push(val); Ok(())},
                        Err(_) => Err(Error::InvalidWord),
                    }
                } else {
                    Err(Error::InvalidWord)
                }
            };

            "+" => |forth: &mut Forth, _| {
                forth.call("_ARITH", Some("+"))
            };

            "-" => |forth: &mut Forth, _| {
                forth.call("_ARITH", Some("-"))
            };

            "/" => |forth: &mut Forth, _| {
                forth.call("_ARITH", Some("/"))
            };

            "*" => |forth: &mut Forth, _| {
                forth.call("_ARITH", Some("*"))
            };
        );

        new
    }
}
