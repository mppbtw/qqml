//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2024 'mppbtw'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Flag {
    pub usage:   &'static str,
    pub aliases: Vec<&'static str>,
    pub arg:     Option<FlagArgumentType>,
    pub long:    Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlagArgumentType {
    Int,
    Uint,
    String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AnsweredFlag {
    pub usage: &'static str,
    pub arg:   Option<AnsweredFlagArgument>,
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
        self.flags.iter().find(|f| f.usage == name)
    }
}
