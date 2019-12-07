use std::ops::Deref;
use std::rc::Rc;

pub use bindings::*;
pub use bindings::Gl as InnerGl;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    pub fn load_with<F>(load_fn: F) -> Gl
        where F: FnMut(&'static str) -> *const types::GLvoid,
    {
        Gl {
            inner: Rc::new(bindings::Gl::load_with(load_fn)),
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}
