#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(VertexAttribPointers, attributes())]
pub fn vertex_attrib_pointers_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let _input = parse_macro_input!(input as DeriveInput);

    // Build the output
    let expanded = quote! {
        impl Vertex {
            fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = std::mem::size_of::<Self>(); // byte offset between consecutive attributes

                let location = 0; // layout (location = 0)
                let offset = 0; // offset of the first component

                unsafe { data::f32x3::vertex_attrib_pointer(gl, stride, location, offset); }

                let location = 1; // layout (location = 1)
                let offset = offset + std::mem::size_of::<data::f32x3>(); // offset of the first component

                unsafe { data::f32x3::vertex_attrib_pointer(gl, stride, location, offset); }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
