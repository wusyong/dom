use gc::Trace;
use kuchiki::NodeRef;

unsafe impl Trace for Document {
    // NodeRef is already reference counted.
    unsafe_empty_trace!();
}

#[derive(Debug, Finalize)]
pub struct Document {
    pub node: NodeRef,
}
