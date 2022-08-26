pub mod github;
pub mod medium;

pub trait Processor {
    fn process(&self);
}
