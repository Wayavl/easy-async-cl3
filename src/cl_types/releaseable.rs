pub trait Releaseable {
    unsafe fn increase_reference_count(&self);
}