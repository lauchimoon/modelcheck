use std::path;

use test_case::test_case;

use modelcheck::ctl::model::Model;
use modelcheck::prop::formula::Formula;

#[test_case("A[a U b] v EXAGb".to_string(), true)]
#[test_case("AG(a -> EXb)".to_string(), true)]
#[test_case("EFAGb".to_string(), true)]
#[test_case("AFb".to_string(), true)]
#[test_case("AG(a -> AF(~a))".to_string(), true)]
#[test_case("EXEXa".to_string(), true)]
#[test_case("AFEGb".to_string(), true)]
#[test_case("~EGa".to_string(), true)]
#[test_case("EF(a ^ AXa)".to_string(), true)]
#[test_case("A[~a U (a v AGb)]".to_string(), false)]
#[test_case("E[~b U a]".to_string(), false)]
#[test_case("EGA[a U b]".to_string(), false)]
#[test_case("AXb".to_string(), false)]
#[test_case("AG(b -> AFa)".to_string(), false)]
#[test_case("AF(a ^ b)".to_string(), false)]
#[test_case("EG(~b)".to_string(), false)]
#[test_case("AG(a -> AXa)".to_string(), false)]
#[test_case("E[a U ~b]".to_string(), false)]
fn test_002(formula_string: String, expected: bool) {
    let path = path::absolute("tests/models/002.model").expect("file not found")
        .as_os_str()
        .to_str().expect("failed to convert to &str")
        .to_string();
    let model = Model::from_file(path);
    let formula = Formula::from_string(formula_string);
    assert_eq!(model.check(&formula), expected);
}
