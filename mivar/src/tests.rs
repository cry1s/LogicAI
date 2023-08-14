use crate::utils::process_function_string;
use crate::*;
use serde_json::{json, Value};

#[test]
fn triangle_test() {
    let mut kb = KnowledgeBase::new();
    let mut triangle_class = kb
        .new_class("Triangle", "Model of triangle")
        .expect("all names of classes and parametres are unique in layer of abstraction");
    let mut triangle_sides_class = triangle_class
        .new_class("Sides", "sides of triangle")
        .unwrap();
    let a_ref = triangle_sides_class
        .new_parameter("a", "First side", None)
        .unwrap();
    let b_ref = triangle_sides_class
        .new_parameter("b", "Second side", None)
        .unwrap();
    let c_ref = triangle_sides_class
        .new_parameter("c", "Third side", None)
        .unwrap();
    let mut triangle_parametres_class = triangle_class.new_class("Parametres", "").unwrap();
    let big_p_ref = triangle_parametres_class
        .new_parameter("P", "Perimeter", None)
        .unwrap();
    let small_p_ref = triangle_parametres_class
        .new_parameter("p", "Half of perimeter", None)
        .unwrap();
    let big_s_ref = triangle_parametres_class
        .new_parameter("S", "Square", None)
        .unwrap();
    let three_sum_ref = kb
        .new_relation(
            "function three_sum(a, b, c) { return a + b + c }",
            "Summing three numbers",
        )
        .expect("Name of function of each relation must be unique");
    let half_ref = kb
        .new_relation("function half(a) { return a / 2 }", "Half of number")
        .unwrap();
    let heron_ref = kb
        .new_relation(
            "function heron(a, b, c, p) { return Math.sqrt(p*(p-a)*(p-b)*(p-c)) }",
            "Heron formula",
        )
        .unwrap();
    kb.new_rule(
        "Calculating perimeter via all sides",
        "",
        three_sum_ref,
        &[a_ref.clone(), b_ref.clone(), c_ref.clone()],
        big_p_ref.clone(),
    )
    .expect("Same args number as in relation");
    kb.new_rule(
        "Calculating small p",
        "p is P/2",
        half_ref,
        &[big_p_ref.clone()],
        small_p_ref.clone(),
    )
    .unwrap();
    kb.new_rule(
        "Calculating square",
        "Heron formula",
        heron_ref,
        &[a_ref.clone(), b_ref.clone(), c_ref.clone(), small_p_ref],
        big_s_ref.clone(),
    )
    .unwrap();

    let solution = kb
        .solve(
            &[(a_ref, json!(3)), (b_ref, json!(4)), (c_ref, json!(5))],
            &[big_s_ref, big_p_ref],
        )
        .expect("Enough data to solve");
    assert_eq!(
        solution.get("Triangle/Parametres/S").unwrap(),
        &Value::Number(6.into())
    )
}

#[test]
fn triangle_builder_test() {
    let mut kbb = KnowledgeBase::builder();
    kbb.new_class("Triangle", "Model of triangle")
        .new_class("Sides", "sides of triangle")
        .add_parameter("a", "First side", None)
        .add_parameter("b", "Second side", None)
        .add_parameter("c", "Third side", None)
        // leaving "sides" class to upper level
        .leave_class()
        // now we at "triangle" class
        .new_class("Parametres of triangle", "")
        .add_parameter("P", "Perimeter", None)
        .add_parameter("p", "Half of perimeter", None)
        .add_parameter("S", "Square", None)
        .go_base() // we are in kb again
        .new_relation("function three_sum(a, b, c) { return a + b + c }")
        .new_relation("function half(a) { return a / 2 }")
        .new_relation("function heron(a, b, c, p) { return Math.sqrt(p*(p-a)*(p-b)*(p-c)) }")
        .new_rule(
            "three_sum",
            &["Triangle.Sides.a", "Triangle.Sides.b", "Triangle.Sides.c"].into(),
            &["Parametres of triangle"].into(),
            "Calc perimeter",
            "Using sum to count perimeter",
        )
        .new_rule(
            "half",
            &["Parametres of triangle.P"].into(),
            &["Parametres of triangle.p"].into(),
            "Calc perimeter",
            "Using sum to count perimeter",
        )
        .build();
    todo!();
}

#[test]
fn parse_func() {
    assert_eq!(
        process_function_string(
            "function heron(a, b, c, p) { return Math.sqrt(p*(p-a)*(p-b)*(p-c)) }"
        ),
        Some(("heron".to_string(), 4))
    )
}
