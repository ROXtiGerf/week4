
enum TrafficLight{
    Red,
    Green,
    Yellow,
}


fn main() {
    let yellow = TrafficLight::Yellow;
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;

    println!("Yellow light will show for  {:?} seconds",time(yellow));
    println!("Red light will show for  {:?} seconds",time(red));
    println!("Green light will show for  {:?} seconds",time(green) )
}


fn time(color: TrafficLight) ->u8 {
    match color {
        TrafficLight::Yellow => 3,
        TrafficLight::Red => 40,
        TrafficLight::Green => 37,            
    }
}