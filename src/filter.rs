use globset::{Glob, GlobSet, GlobSetBuilder};

pub struct IgnoreFilter {
    set: GlobSet,
    patterns: Vec<String>,
}

impl IgnoreFilter {
    pub fn new(patterns: &[String]) -> Result<Self, globset::Error> {
        let mut builder = GlobSetBuilder::new();
        for p in patterns {
            let glob = Glob::new(&p.to_lowercase())?;
            builder.add(glob);
        }
        Ok(Self {
            set: builder.build()?,
            patterns: patterns.to_vec(),
        })
    }

    pub fn is_ignored(&self, value: &str) -> bool {
        self.set.is_match(value.to_lowercase())
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    pub fn patterns(&self) -> &[String] {
        &self.patterns
    }
}
