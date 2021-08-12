use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gigantesque {
    name: String,
    on_memory: Option<String>,
}

impl Gigantesque {
    pub fn new(name: &str, on_memory: bool) -> Result<Self> {
        Ok(Self {
            name: format!("{}.gigantesque", name),
            on_memory: if on_memory { Some(String::new()) } else { None },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Gigantesque;

    #[test]
    fn creation() {
        assert_eq!(
            Gigantesque::new("name", true).unwrap(),
            Gigantesque {
                name: String::from("name.gigantesque"),
                on_memory: Some(String::new())
            }
        );
    }
}
