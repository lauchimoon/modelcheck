use std::path;

use test_case::test_case;

use modelcheck::ctl::model::Model;
use modelcheck::prop::formula::Formula;

#[test_case("E[(r v p) U q]".to_string(), true)]
#[test_case("AG(r v p v q v f)".to_string(), true)]
#[test_case("EFf".to_string(), true)]
#[test_case("AG(p -> EXq)".to_string(), true)]
#[test_case("AG(q -> EFp)".to_string(), true)]
#[test_case("AG(f -> AXf)".to_string(), true)]
#[test_case("AXr".to_string(), true)]
#[test_case("E[r U (q ^ EXf)]".to_string(), false)]
#[test_case("AFAGf".to_string(), false)]
#[test_case("AF(p v f)".to_string(), false)]
#[test_case("A[r U (p v q)]".to_string(), false)]
#[test_case("AFp".to_string(), false)]
#[test_case("AF(q ^ r)".to_string(), false)]
#[test_case("AGr".to_string(), false)]
#[test_case("A[r U f]".to_string(), false)]
#[test_case("AG(p -> AFf)".to_string(), false)]
#[test_case("EX(p ^ q)".to_string(), false)]
#[test_case("EGp".to_string(), false)]
#[test_case("AG(r -> AXAXr)".to_string(), false)]
fn test_004(formula_string: String, expected: bool) {
    let path = path::absolute("tests/models/004.model").expect("file not found")
        .as_os_str()
        .to_str().expect("failed to convert to &str")
        .to_string();
    let model = Model::from_file(path);
    let formula = Formula::from_string(formula_string);
    assert_eq!(model.check(&formula), expected);
}
