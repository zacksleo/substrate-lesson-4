use traffic_light::*;

#[test]
fn duration() {
    assert_eq!(60, TrafficLight::Red.duration());
    assert_eq!(5, TrafficLight::Yellow.duration());
    assert_eq!(30, TrafficLight::Green.duration());
}
