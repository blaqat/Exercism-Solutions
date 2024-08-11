use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
type Stack = Vec<Value>;
type Tokens = Vec<TokenTypes>;
type Identifiers = HashMap<String, Tokens>;

#[derive(Debug, Clone)]
enum TokenTypes {
    Val(Value),
    Ident(String),
    OpenDef,
    CloseDef,
    Op(String), // Default Forth Operations
}

impl TokenTypes {
    // Converts an input string into a Vector of TokenTypes
    fn from_str(input: &str) -> Tokens {
        input
            .replace('\n', " ")
            .split_whitespace()
            .map(|token| {
                if let Ok(num) = token.parse() {
                    TokenTypes::Val(num)
                } else {
                    match token {
                        ":" => TokenTypes::OpenDef,
                        ";" => TokenTypes::CloseDef,
                        t => TokenTypes::Ident(t.to_lowercase()),
                    }
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub struct Forth {
    stack: Stack,
    idents: Identifiers,
}

impl Forth {
    pub fn new() -> Self {
        let mut idents = HashMap::new();

        idents.insert("+".into(), vec![TokenTypes::Op("_add".into())]);
        idents.insert("-".into(), vec![TokenTypes::Op("_sub".into())]);
        idents.insert("/".into(), vec![TokenTypes::Op("_div".into())]);
        idents.insert("*".into(), vec![TokenTypes::Op("_mult".into())]);
        idents.insert("dup".into(), vec![TokenTypes::Op("_dup".into())]);
        idents.insert("drop".into(), vec![TokenTypes::Op("_drop".into())]);
        idents.insert("swap".into(), vec![TokenTypes::Op("_swap".into())]);
        idents.insert("over".into(), vec![TokenTypes::Op("_over".into())]);

        Self {
            stack: Stack::new(),
            idents,
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    #[allow(unused_variables)]
    pub fn eval(&mut self, input: &str) -> Result {
        TokenTypes::from_str(input) // Converts input to tokens
            .validate(&mut self.idents)? // Validates tokens as a valid command and/or deifnition
            .run(self.stack.as_mut(), &self.idents) // Tries to run series of token operation/values
    }
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}

// Forth Stack Operations
trait StackOperations {
    fn _dupl(&mut self) -> Result;
    fn _drop(&mut self) -> Result;
    fn _swap(&mut self) -> Result;
    fn _over(&mut self) -> Result;
    fn _add(&mut self) -> Result;
    fn _sub(&mut self) -> Result;
    fn _div(&mut self) -> Result;
    fn _mult(&mut self) -> Result;
    fn _ins(&mut self, x: Value) -> Result;
}

impl StackOperations for Stack {
    fn _dupl(&mut self) -> Result {
        if let Some(&num) = self.iter().last() {
            self.push(num);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn _drop(&mut self) -> Result {
        self.pop().map_or(Err(Error::StackUnderflow), |_| Ok(()))
    }

    fn _swap(&mut self) -> Result {
        let stack_len = self.len();

        if stack_len < 2 {
            Err(Error::StackUnderflow)
        } else {
            self.swap(stack_len - 2, stack_len - 1);
            Ok(())
        }
    }

    fn _over(&mut self) -> Result {
        let stack_len = self.len();
        if stack_len < 2 {
            Err(Error::StackUnderflow)
        } else {
            self.push(self[stack_len - 2]);
            Ok(())
        }
    }

    fn _add(&mut self) -> Result {
        match (self.pop(), self.pop()) {
            (Some(a), Some(b)) => {
                self.push(a + b);
                Ok(())
            }
            _ => Err(Error::StackUnderflow),
        }
    }

    fn _div(&mut self) -> Result {
        match (self.pop(), self.pop()) {
            (Some(0), Some(_)) => Err(Error::DivisionByZero),
            (Some(b), Some(a)) => {
                self.push(a / b);
                Ok(())
            }
            _ => Err(Error::StackUnderflow),
        }
    }

    fn _sub(&mut self) -> Result {
        match (self.pop(), self.pop()) {
            (Some(b), Some(a)) => {
                self.push(a - b);
                Ok(())
            }
            _ => Err(Error::StackUnderflow),
        }
    }

    fn _mult(&mut self) -> Result {
        match (self.pop(), self.pop()) {
            (Some(a), Some(b)) => {
                self.push(a * b);
                Ok(())
            }
            _ => Err(Error::StackUnderflow),
        }
    }

    fn _ins(&mut self, x: Value) -> Result {
        self.push(x);
        Ok(())
    }
}

trait Command {
    fn is_flat(&self) -> bool;
    fn flatten_with(&mut self, command: &str, flattened: &Tokens) -> Tokens;
    fn flatten_all(&self, idents: &Identifiers) -> Tokens;
    fn validate(&self, idents: &mut Identifiers) -> std::result::Result<Tokens, Error>;
    fn run(&self, stack: &mut Stack, idents: &Identifiers) -> Result;
}

impl Command for Tokens {
    // Checks whether command has any unflattened Identifier tokens
    fn is_flat(&self) -> bool {
        !self
            .iter()
            .any(|token| matches!(token, TokenTypes::Ident(_)))
    }

    // Flattens Identifier to a set of tokens
    fn flatten_with(&mut self, identifier_name: &str, flattened: &Tokens) -> Tokens {
        self.iter()
            .flat_map(|token| {
                if matches!(token, TokenTypes::Ident(a) if a == identifier_name) {
                    flattened.clone()
                } else {
                    vec![token.clone()]
                }
            })
            .collect()
    }

    // Flattens a Command to only Forth Operations and/or Value insertions
    fn flatten_all(&self, idents: &Identifiers) -> Tokens {
        if self.is_flat() {
            return self.to_owned();
        }

        self.iter()
            .flat_map(|token| match token {
                TokenTypes::Ident(i) => {
                    let mut cmd = idents.get(i).unwrap().to_owned();
                    if !cmd.is_flat() {
                        cmd = cmd.flatten_all(idents);
                    }
                    cmd.into_iter()
                }
                t => vec![t.clone()].into_iter(),
            })
            .collect()
    }

    // Validates Commands and Definitions; Returns a Valid Command or an Error
    fn validate(&self, idents: &mut Identifiers) -> std::result::Result<Tokens, Error> {
        let mut new_command = vec![];
        let mut new_def = vec![];
        let mut building_def = false;
        let mut def_name = "".to_string();

        for token in self {
            match token {
                TokenTypes::Ident(n) => {
                    if building_def && def_name.is_empty() {
                        def_name = n.clone();
                    } else if idents.get(n).is_some() {
                        if building_def {
                            new_def.push(token.clone());
                        } else {
                            new_command.push(token.clone());
                        }
                    } else {
                        return Err(Error::UnknownWord);
                    }
                }
                TokenTypes::Val(_) | TokenTypes::Op(_) => {
                    if building_def {
                        new_def.push(token.clone());
                    } else {
                        new_command.push(token.clone());
                    }
                }

                // Definition Building
                TokenTypes::OpenDef => {
                    if building_def {
                        return Err(Error::InvalidWord);
                    }
                    building_def = true;
                    def_name.clear();
                    new_def.clear();
                }

                TokenTypes::CloseDef => {
                    if !building_def || new_def.is_empty() || def_name.is_empty() {
                        return Err(Error::InvalidWord);
                    }
                    building_def = false;

                    // Validates old definitions being replaced (same names) in Identifier Dict and new definition
                    if let Some(old_def) = idents.get(&def_name).cloned() {
                        idents
                            .iter_mut()
                            .for_each(|(_, cmd)| *cmd = cmd.flatten_with(&def_name, &old_def));

                        new_def = new_def.flatten_with(&def_name, &old_def);
                    }

                    idents.insert(def_name.clone(), new_def.clone());
                }
            }
        }

        if building_def {
            Err(Error::InvalidWord)
        } else {
            Ok(new_command)
        }
    }

    // Tries to Run a series of Value insertions or Stack operations (Command)
    fn run(&self, stack: &mut Stack, idents: &Identifiers) -> Result {
        for token in self.flatten_all(idents).iter() {
            match token {
                TokenTypes::Val(x) => stack._ins(*x),
                TokenTypes::Op(word) => match word.as_str() {
                    "_add" => stack._add(),
                    "_sub" => stack._sub(),
                    "_div" => stack._div(),
                    "_mult" => stack._mult(),
                    "_dup" => stack._dupl(),
                    "_drop" => stack._drop(),
                    "_swap" => stack._swap(),
                    "_over" => stack._over(),
                    _ => Err(Error::UnknownWord),
                },
                _ => Err(Error::InvalidWord),
            }?;
        }
        Ok(())
    }
}
