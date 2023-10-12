#[derive(Debug, Copy, Clone)]
pub struct Quantity {
    pub base: f64,
    pub multiplier: f64,
}

impl Default for Quantity {
    fn default() -> Self {
        Quantity {
            base: 0.0,
            multiplier: 1.0,
        }
    }
}

impl From<f64> for Quantity {
    fn from(value: f64) -> Self {
        Self {
            base: value,
            ..Default::default()
        }
    }
}

impl Quantity {
    pub fn value(&self) -> f64 {
        // TODO: figure out if this should be cached
        self.base * self.multiplier
    }

    pub fn new(base: f64) -> Self {
        base.into()
    }
}
