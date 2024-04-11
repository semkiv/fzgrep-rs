struct Filter {
    predicate: Box<dyn Fn(&str) -> bool>,
}

impl Filter {
    pub fn from_include(glob: &str) -> Result<Self, globset::Error> {
        let glob = globset::Glob::new(glob)?.compile_matcher();
        Ok(Self {
            predicate: Box::new(move |path| glob.is_match(path)),
        })
    }

    pub fn from_exclude(glob: &str) -> Result<Self, globset::Error> {
        let glob = globset::Glob::new(glob)?.compile_matcher();
        Ok(Self {
            predicate: Box::new(move |path| !glob.is_match(path)),
        })
    }

    pub fn include(self, glob: &str) -> Result<Self, globset::Error> {
        let glob = globset::Glob::new(glob)?.compile_matcher();
        Ok(Self {
            predicate: Box::new(move |path| (self.predicate)(path) || glob.is_match(path)),
        })
    }

    pub fn exclude(self, glob: &str) -> Result<Self, globset::Error> {
        let glob = globset::Glob::new(glob)?.compile_matcher();
        Ok(Self {
            predicate: Box::new(move |path| (self.predicate)(path) && !glob.is_match(path)),
        })
    }

    pub fn test(&self, path: &str) -> bool {
        (self.predicate)(path)
    }
}
