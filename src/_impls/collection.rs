use crate::Error;
use crate::error;

impl Error {
    pub fn is_collection(&self) -> bool { self.is(crate::collection) }
    
    pub fn contains_category(&self, prefix: &str) -> bool {
        match self.is_collection() {
            true => self.errors.iter().any(|e| e.is_category(prefix)),
            false => self.is_category(prefix),
        }
    }
    
    pub fn contains_exact(&self, etype: &str) -> bool {
        match self.is_collection() {
            true => self.errors.iter().any(|e| e.is(etype)),
            false => self.is(etype),
        }
    }
    
    pub fn get_error(&self, etype: &str) -> Option<&Error> {
        match self.is_collection() {
            true => self.errors.get(etype),
            false => self.is(etype).then_some(self),
        }
    }
    
    pub fn get_etypes(&self) -> Box<dyn Iterator<Item = &'static str> + '_> {
        match self.is_collection() {
            true => Box::new(self.errors.iter().map(|e| e.etype())),
            false => Box::new(std::iter::once(self.etype())),
        }
    }
}

pub enum OptionError {
    Some(Error),
    None,
} 

impl OptionError {
    pub fn into_option(self) -> Option<Error> { self.into() }
}

impl From<OptionError> for Option<Error> {
    fn from(value: OptionError) -> Self {
        match value {
            OptionError::Some(error) => Some(error),
            OptionError::None => None,
        }
    }
}

impl FromIterator<Error> for OptionError {
    fn from_iter<T: IntoIterator<Item = Error>>(iter: T) -> Self {
        let mut errors = iter.into_iter().peekable();
        
        let first = match errors.next() {
            Some(e) => e,
            None => return OptionError::None,
        };
        
        let peek = errors.peek();
        if peek.is_none() { return OptionError::Some(first); };
        
        let mut error = error!(
            crate::collection,
            "Multiple errors happend",
            "Here is a resume of them",
        );
        
        error = concat(error, first);
        
        while let Some(other) = errors.next() {
            error = concat(error, other)
        }
        
        if error.errors.len() == 1 {
            OptionError::Some(error.errors.into_iter().take(1).next().unwrap())
        } else {
            OptionError::Some(error)
        }
    }
}

fn concat(mut error: Error, other: Error) -> Error {
    match other.is_collection() {
        true => {
            error.errors.extend(other.errors);
            error
        },
        false => {
            error.errors.insert(other);
            error
        },
    }
}
