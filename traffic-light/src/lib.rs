pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait HasDuration {
    fn duration(&self) -> usize;
}

impl HasDuration for TrafficLight {
    fn duration(&self) -> usize {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 30,
        }
    }
}
