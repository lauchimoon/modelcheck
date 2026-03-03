use std::path;

use test_case::test_case;

use modelcheck::ctl::model::Model;
use modelcheck::prop::formula::Formula;

#[test_case("r -> AXg".to_string(), false)]
#[test_case("y -> AXAXy".to_string(), true)]
#[test_case("EG(~g)".to_string(), false)]
#[test_case("AFg".to_string(), true)]
#[test_case("AFy".to_string(), true)]
#[test_case("AGy".to_string(), false)]
#[test_case("AGAFy".to_string(), true)]
#[test_case("A[b U ~b]".to_string(), true)]
#[test_case("A[~b U b]".to_string(), false)]
#[test_case("E[b U r]".to_string(), true)]
#[test_case("r -> AFg".to_string(), true)]
fn test_005(formula_string: String, expected: bool) {
    let path = path::absolute("tests/models/005.model").expect("file not found")
        .as_os_str()
        .to_str().expect("failed to convert to &str")
        .to_string();
    let model = Model::from_file(path);
    let formula = Formula::from_string(formula_string);
    assert_eq!(model.check(&formula), expected);
}
