extern crate proc_macro;

use proc_macro2::TokenStream;
use syn::{self, DeriveInput, parse_macro_input};

use quote::quote;

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = generate_impl(&input);
    proc_macro::TokenStream::from(gen)
}

fn generate_impl(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;
    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.data);

    quote! {
        impl #name #generics #where_clause {
            #[allow(unused_variables)]
            pub fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = std::mem::size_of::<Self>();
                let offset = 0;

                #(#fields_vertex_attrib_pointer)*
            }
        }
    }
}

fn generate_vertex_attrib_pointer_calls(data: &syn::Data) -> Vec<TokenStream> {
    match data {
        syn::Data::Enum(_) => panic!("VertexAttribPointers can not be implemented for enums"),
        syn::Data::Union(_) => panic!("VertexAttribPointers can not be implemented for unions"),
        syn::Data::Struct(ref s) => {
            if s.fields.is_empty() {
                panic!("VertexAttribPointers can not be implemented for Unit structs");
            }

            return s.fields
                .iter()
                .map(generate_struct_field_vertex_attrib_pointer_call)
                .collect();
        }
    }
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &syn::Field) -> TokenStream {
    let field_name = match field.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };
    let location_attr = field.attrs
        .iter()
        .filter(|a| a.path.is_ident("location"))
        .next()
        .unwrap_or_else(|| panic!(
            "Field {:?} is missing #[location = ?] attribute", field_name
        ));

    let meta = location_attr.parse_meta().unwrap();
    let location_value_literal = match meta {
        syn::Meta::NameValue(ref name_value) => match &name_value.lit {
            syn::Lit::Int(ref lit) => lit,
            _ => panic!("")
        },
        _ => panic!("Field {} location attribute value must be an integer literal", field_name),
    };

    let field_ty = &field.ty;
    quote! {
        let location = #location_value_literal;
        unsafe {
            #field_ty::vertex_attrib_pointer(gl, stride, location, offset);
        }
        let offset = offset + std::mem::size_of::<#field_ty>();
    }
}
