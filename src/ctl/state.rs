#[derive(Default, Eq, Hash, PartialEq, Debug, Clone)]
pub struct State {
    pub labels: Vec<String>,
    pub transitions: Vec<String>,
}
