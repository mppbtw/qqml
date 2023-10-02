#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Flag {
    pub long:    &'static str,
    pub aliases: Vec<&'static str>,
    pub arg:     Option<FlagArgumentType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlagArgumentType {
    Int,
    Uint,
    String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AnsweredFlag {
    pub long: &'static str,
    pub arg:  Option<AnsweredFlagArgument>,
}
impl AnsweredFlag {
    pub fn int(&self) -> Result<&isize, ErrIncorrectArgType> {
        self.arg.as_ref().unwrap().int()
    }

    pub fn uint(&self) -> Result<&usize, ErrIncorrectArgType> {
        self.arg.as_ref().unwrap().uint()
    }

    pub fn string(&self) -> Result<&String, ErrIncorrectArgType> {
        self.arg.as_ref().unwrap().string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnsweredFlagArgument {
    Int(isize),
    Uint(usize),
    String(String),
}
impl AnsweredFlagArgument {
    pub fn int(&self) -> Result<&isize, ErrIncorrectArgType> {
        match self {
            Self::Int(i) => Ok(i),
            _ => Err(ErrIncorrectArgType),
        }
    }

    pub fn uint(&self) -> Result<&usize, ErrIncorrectArgType> {
        match self {
            Self::Uint(u) => Ok(u),
            _ => Err(ErrIncorrectArgType),
        }
    }

    pub fn string(&self) -> Result<&String, ErrIncorrectArgType> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(ErrIncorrectArgType),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrIncorrectArgType;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AnsweredFlags {
    pub flags: Vec<AnsweredFlag>,
}
impl AnsweredFlags {
    pub fn get(&self, name: &str) -> Option<&AnsweredFlag> {
        self.flags.iter().find(|f| f.long == name)
    }
}
