#[derive(Debug)]
pub struct Valve {
    pub rate: u16,
    pub paths: Vec<usize>,
    pub name: usize,
}
