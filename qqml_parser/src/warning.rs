#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Warning {
    MaxMarkImpossible(usize, usize)
}
