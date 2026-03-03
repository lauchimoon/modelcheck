use std::path;

use test_case::test_case;

use modelcheck::ctl::model::Model;
use modelcheck::prop::formula::Formula;

#[test_case("E[EXa U ~a]".to_string(), true)]
#[test_case("EF(a ^ b)".to_string(), true)]
#[test_case("AG(a v b v ~(a v b))".to_string(), true)]
#[test_case("AF(a v b)".to_string(), true)]
#[test_case("EXa".to_string(), true)]
#[test_case("A[~b U (a v b)]".to_string(), true)]
#[test_case("AG(a -> AX(a ^ b))".to_string(), false)]
#[test_case("EFAGb".to_string(), false)]
#[test_case("EGAF(~b)".to_string(), false)]
#[test_case("AX(EG~a v EGb)".to_string(), false)]
#[test_case("AFAGa".to_string(), false)]
#[test_case("A[b U a]".to_string(), false)]
#[test_case("AFAG(~a ^ ~b)".to_string(), false)]
#[test_case("AXAXa".to_string(), false)]
#[test_case("AF(a ^ AX(a ^ AX(a ^ b)))".to_string(), false)]
fn test_006(formula_string: String, expected: bool) {
    let path = path::absolute("tests/models/006.model").expect("file not found")
        .as_os_str()
        .to_str().expect("failed to convert to &str")
        .to_string();
    let model = Model::from_file(path);
    let formula = Formula::from_string(formula_string);
    assert_eq!(model.check(&formula), expected);
}
