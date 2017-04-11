use super::git2;

pub struct GitReference<'a> {
    reference: git2::Reference<'a>,
}

impl<'a> GitReference<'a> {
    pub fn new(reference: git2::Reference<'a>) -> Self {
        GitReference { reference: reference }
    }

    pub fn name(&self) -> Option<&str> {
        self.reference.name()
    }
}