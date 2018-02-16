use git2;

pub struct GitReference {
    name: String,
}

impl GitReference {
    pub fn new(reference: &git2::Reference) -> Self {
        let name = match reference.name() {
            Some(n) => String::from(n),
            None => String::new(),
        };

        Self { name: name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
