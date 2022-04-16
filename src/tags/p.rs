use super::Tag;

/// p.
#[derive(Debug, Clone)]
pub struct P;

impl Tag for P {
    fn name(&self) -> &'static str {
        "p"
    }
}
