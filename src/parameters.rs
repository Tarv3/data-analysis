use std::{error::Error, path::Path, fmt::Display, collections::HashMap};

#[derive(Copy, Clone, Debug)]
pub struct ParamAlreadyExists;

impl Display for ParamAlreadyExists {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parameter already exists")
    }
}

impl Error for ParamAlreadyExists {}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum ParamType {
    Double,
    Int
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub _type: ParamType,
    pub name: String, 
    pub pos: usize,
}

pub fn split_str_num_alpha_groups<F: FnMut(&str)>(input: &str, mut f: F) {
    let mut found_alpha = false;
    let mut found_num = false;
    let mut start = 0;

    for (index, character) in input.char_indices() {
        if character.is_alphabetic() {
            if !found_alpha {
                found_alpha = true;
            }
        }
        else if character.is_numeric() && !found_num {
            found_num = true;
            start = index;
        }
        else {
            if found_alpha {
                found_alpha = false;
                found_num = character.is_numeric();
                f(&input[start..index]);
                start = index;
            }
        }
    }

    f(&input[start..]);
}

// Structure for loading parameter values from file names
pub struct Parameters {
    param_list: HashMap<String, Parameter>,
    current: Vec<Option<Box<dyn Display>>>, 
}

impl Parameters {
    pub fn new() -> Self {
        Parameters {
            param_list: HashMap::new(),
            current: Vec::new(),
        }
    }

    pub fn set_params_list(&mut self, params: impl Iterator<Item = (String, String, ParamType)>) -> Result<(), ParamAlreadyExists> {
        for (pos, (shorthand, name, _type)) in params.enumerate() {
            let param = Parameter { pos, name, _type };

            let previous = self.param_list.insert(shorthand, param);

            if previous.is_some() {
                return Err(ParamAlreadyExists);
            }
        }

        Ok(())
    }

    fn update_single_param(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        let middle = input.find(|a: char| a.is_alphabetic()).unwrap();

        let (value, parameter) = input.split_at(middle);
        let mut value = value.replace('_', ".");

        if let Some(Parameter { pos, _type, .. }) = self.param_list.get(parameter) {
            let value = match _type {
                ParamType::Double => {
                    let value = value.parse::<f64>()?;
                    Box::new(value) as Box<dyn Display>
                },
                ParamType::Int => {
                    let value = value.parse::<i64>()?;
                    Box::new(value) as Box<dyn Display>
                }
            };

            self.current[*pos] = Some(value);
        }

        Ok(())
    }

    pub fn update_current(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        self.current.clear();

        for _ in 0..self.param_list.len() {
            self.current.push(None);
        }

        split_str_num_alpha_groups(input, |value| {
            match self.update_single_param(value) {
                Ok(()) => (),
                Err(e) => println!("Param Error: {}", e)
            }
        });

        Ok(())
    }

    pub fn write_headers(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
        let mut params = Vec::with_capacity(self.param_list.len());

        for _ in 0..self.param_list.len() {
            params.push(None);
        }

        for param in self.param_list.values() {
            params[param.pos] = Some(&param.name);
        }

        for param in params.into_iter() {
            write!(f, "{},", param.unwrap())?;
        }
        Ok(())
    }

    pub fn write_as_line(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
        for value in self.current.iter() {
            match value {
                Some(value) => write!(f, "{},", value)?,
                None => write!(f, ",")?,
            }
        }

        Ok(())
    }
} 