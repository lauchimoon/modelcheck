use std::path;

use test_case::test_case;

use modelcheck::ctl::model::Model;
use modelcheck::prop::formula::Formula;

#[test_case("EXEXc".to_string(), true)]
#[test_case("AGEFb".to_string(), true)]
#[test_case("EGA[a U b]".to_string(), false)]
#[test_case("EF(b ^ c)".to_string(), true)]
#[test_case("EX(a ^ c)".to_string(), true)]
#[test_case("AG(a v b)".to_string(), true)]
#[test_case("AG(~(a ^ b))".to_string(), true)]
#[test_case("AFb".to_string(), false)]
#[test_case("AG(c -> AFb)".to_string(), false)]
#[test_case("E[a U (b ^ ~c)]".to_string(), true)]
fn test_001(formula_string: String, expected: bool) {
    let path = path::absolute("tests/models/001.model").expect("file not found")
        .as_os_str()
        .to_str().expect("failed to convert to &str")
        .to_string();
    let model = Model::from_file(path);
    let formula = Formula::from_string(formula_string);
    assert_eq!(model.check(&formula), expected);
}
