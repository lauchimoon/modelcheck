use std::path;

use test_case::test_case;

use modelcheck::ctl::model::Model;
use modelcheck::prop::formula::Formula;

#[test_case("E[b U AG(~b)]".to_string(), true)]
#[test_case("a ^ b".to_string(), true)]
#[test_case("EXb".to_string(), true)]
#[test_case("EF(AGa v AG(~a))".to_string(), true)]
#[test_case("EXEX(~b)".to_string(), true)]
#[test_case("EGEXb".to_string(), true)]
#[test_case("AFa".to_string(), true)]
#[test_case("A[EXa U ~b]".to_string(), false)]
#[test_case("AF(~a v AGa)".to_string(), false)]
#[test_case("EGa".to_string(), false)]
#[test_case("E[b U AGa]".to_string(), false)]
#[test_case("AXa".to_string(), false)]
#[test_case("AGb".to_string(), false)]
#[test_case("E[a U ~b]".to_string(), false)]
#[test_case("EF(a ^ ~b ^ EXb)".to_string(), false)]
#[test_case("AFAGb".to_string(), false)]
#[test_case("AG(a v b)".to_string(), false)]
fn test_003(formula_string: String, expected: bool) {
    let path = path::absolute("tests/models/003.model").expect("file not found")
        .as_os_str()
        .to_str().expect("failed to convert to &str")
        .to_string();
    let model = Model::from_file(path);
    let formula = Formula::from_string(formula_string);
    assert_eq!(model.check(&formula), expected);
}
