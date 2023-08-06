/// PatternDoc
#[derive(Debug, Clone, Default, PatternDerive)]
pub struct PatternType {
    pub pattern_property: Vec<PatternPropertyProp>,
    pub pattern_parent: PatternParent,
}

#[automatically_derived]
impl PatternType {
    fn add_pattern_property(&mut self, value: impl Into<PatternPropertyProp>) { self.pattern_property.push(value.into()) }

    fn take_pattern_property(&mut self) -> Vec<PatternPropertyProp> { std::mem::take(&mut self.pattern_property) }

    fn set_pattern_property(&mut self, value: impl Into<PatternPropertyProp>) { self.pattern_property = vec![value.into()]; }

    fn set_pattern_property_vec(&mut self, value: Vec<PatternPropertyProp>) { self.pattern_property = value; }

    fn clear_pattern_property(&mut self) { self.pattern_property.clear(); }

    fn add_pattern_parent(&mut self, value: impl Into<PatternParent>) { self.pattern_parent.add_pattern_parent(value); }
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
            "pattern_prop_name_lc" => return Ok(pattern_prop_type_matcher),
            _ => {
                if PatternParent::has_lc_property(name) { return self.pattern_parent.add_lc_property(name, value); }
                return Err(Error::InvalidProperty);
            },
        }
    }
}

impl AsRef<PatternParent> for PatternType { fn as_ref(&self) -> &PatternParent { &self.pattern_parent } }
