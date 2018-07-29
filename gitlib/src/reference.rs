use crate::Error;
use git2;

pub struct Reference {
    name: String,
}

impl Reference {
    pub fn new(reference: &git2::Reference<'_>) -> Result<Self, Error> {
        Ok(Self {
            name: reference
                .name()
                .map(|x| x.into())
                .ok_or(Error::InvalidUtf8)?,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
