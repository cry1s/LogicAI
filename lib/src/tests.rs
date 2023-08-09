use super::*;



#[test]
fn perimeter_test() {
    let kbclass = KBClass::new("Triangle", "Model of triangle")
        .add_parameter("a", "First side")
        .add_parameter("b", "Second side")
        .add_parameter("c", "Third side")
        .add_parameter("P", "Perimeter");
}