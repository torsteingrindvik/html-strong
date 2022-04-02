use super::Attribute;

#[derive(Debug, Clone)]
pub enum Lang {
    English,
    French,
    Spanish,
}

impl Attribute for Lang {
    fn name(&self) -> &'static str {
        "lang"
    }

    fn value(&self) -> String {
        match self {
            Lang::English => "en",
            Lang::French => "fr",
            Lang::Spanish => "es",
        }
        .into()
    }
}
