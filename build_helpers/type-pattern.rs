/// PatternDoc
#[derive(Debug, Clone, Default, PatternDerive)]
pub struct PatternType {
    pub pattern_property: Vec<PatternPropertyProp>,
    pub pattern_parent: PatternParent,
}

#[automatically_derived]
impl Schema for PatternType {
    fn new() -> Self {
        Self::default()
    }

    fn has_lc_property(name: &str) -> bool {
        [
            "pattern_prop_name_lc",
        ]
        .contains(&name)
        || PatternParent::has_lc_property(name)
    }

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "pattern_prop_name_lc" => pattern_prop_type_matcher,
            _ => {
                if PatternParent::has_lc_property(name) { return self.pattern_parent.add_lc_property(name, value); }
                return Err(Error::InvalidProperty);
            },
        }
        Ok(())
    }
}