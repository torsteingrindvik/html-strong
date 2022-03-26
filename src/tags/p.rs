use super::Tag;

/// p.
pub struct P;

impl Tag for P {
    fn name(&self) -> &'static str {
        "p"
    }
}
