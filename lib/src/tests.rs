use super::*;

#[test]
fn triangle_test() {
    let mut kb = KnowledgeBase::new();
    kb.new_class("Triangle", "Model of triangle").unwrap()
        .new_class("Sides", "sides of triangle").unwrap()
        .add_parameter("a", "First side", None).unwrap()
        .add_parameter("b", "Second side", None).unwrap()
        .add_parameter("c", "Third side", None).unwrap()
        // leaving "sides" class to upper level
        .leave_class()
        // now we at "triangle" class
        .new_class("Parametres of triangle", "").unwrap()
        .add_parameter("P", "Perimeter", None).unwrap()
        .add_parameter("p", "Half of perimeter", None).unwrap()
        .add_parameter("S", "Square", None).unwrap()
        .go_base() // we are in kb again

        .new_relation("function three_sum(a, b, c) { return a + b + c }").unwrap()
        .new_relation("function half(a) { return a / 2 }").unwrap()
        .new_relation("function heron(a, b, c, p) { return Math.sqrt(p*(p-a)*(p-b)*(p-c)) }").unwrap()

        .new_rule("three_sum",
            &["Triangle.Sides.a", "Triangle.Sides.b", "Triangle.Sides.c"],
            &["Parametres of triangle"],
            "Calc perimeter",
            "Using sum to count perimeter"
        )
        .new_rule("half",
                  &["Parametres of triangle.P"],
                  &["Parametres of triangle.p"],
                  "Calc perimeter",
                  "Using sum to count perimeter"
        )
        // # TODO
    ;

    kb.add_rule(three_sum, &["a", "b", "c"], &["P"])
}