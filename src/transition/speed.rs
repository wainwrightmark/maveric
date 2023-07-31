

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct ScalarSpeed{
    pub amount_per_second: f32
}

impl ScalarSpeed {
    pub fn new(amount_per_second: f32) -> Self { Self { amount_per_second } }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct LinearSpeed{
    pub units_per_second: f32
}

impl LinearSpeed {
    pub fn new(units_per_second: f32) -> Self { Self { units_per_second } }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct AngularSpeed{
    pub radians_per_second: f32
}

impl AngularSpeed {
    pub fn new(radians_per_second: f32) -> Self { Self { radians_per_second } }
}

