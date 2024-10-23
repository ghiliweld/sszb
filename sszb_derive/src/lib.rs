use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

/// Field-level configuration.
#[derive(Debug, Default, FromMeta)]
struct FieldOpts {
    #[darling(default)]
    skip_encode: bool,
    #[darling(default)]
    skip_decode: bool,
}

fn parse_ssz_fields(
    struct_data: &syn::DataStruct,
) -> impl Iterator<Item = (&syn::Type, Option<&Ident>, Vec<FieldOpts>)> {
    struct_data.fields.iter().map(|field| {
        let ty = &field.ty;
        let ident = field.ident.as_ref();

        // possible field options include skip_encode, skip_decode, skip_hash
        let field_opts = field
            .attrs
            .iter()
            .filter(|attr| {
                attr.path()
                    .get_ident()
                    .map_or(false, |ident| *ident == "ssz")
            })
            .map(|attr| FieldOpts::from_meta(&attr.meta).unwrap())
            .collect::<Vec<_>>();

        (ty, ident, field_opts)
    })
}

#[proc_macro_derive(SszbEncode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let struct_data = match derive_input.data {
        syn::Data::Struct(data) => data,
        _ => panic!(), // TODO: fix
    };
    let name = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();

    let fixed_len_stmts = &mut vec![];
    let static_stmts = &mut vec![];
    let bytes_len_stmts = &mut vec![];
    let max_len_stmts = &mut vec![];
    let ssz_write_fixed_stmts = &mut vec![];
    let write_fixed_stmts = &mut vec![];
    let write_variable_stmts = &mut vec![];

    for (ty, ident, field_opts) in parse_ssz_fields(&struct_data) {
        if field_opts.iter().any(|opt| opt.skip_encode) {
            continue;
        }

        static_stmts.push(quote! { <#ty as sszb::SszEncode>::is_ssz_static() });
        fixed_len_stmts.push(quote! { <#ty as sszb::SszEncode>::ssz_fixed_len() });
        bytes_len_stmts.push(quote! { self.#ident.ssz_bytes_len() });
        max_len_stmts.push(quote! { <#ty as sszb::SszEncode>::ssz_max_len() });
        ssz_write_fixed_stmts.push(quote! { self.#ident.ssz_write_fixed(offset, buf) });
        write_fixed_stmts.push(quote! { self.#ident.ssz_write_fixed(&mut offset, buf) });
        write_variable_stmts.push(quote! { self.#ident.ssz_write_variable(buf) });
    }

    let output = quote! {
        impl #impl_generics sszb::SszEncode for #name #ty_generics #where_clause {
            fn is_ssz_static() -> bool {
                #(
                    #static_stmts &&
                )*
                    true
            }

            fn ssz_fixed_len() -> usize {
                if <Self as sszb::SszEncode>::is_ssz_static() {
                    let mut len: usize = 0;
                    #(
                        len = len
                            .checked_add(#fixed_len_stmts)
                            .expect("encode ssz_fixed_len length overflow");
                    )*
                    len
                } else {
                    sszb::BYTES_PER_LENGTH_OFFSET
                }
            }

            fn ssz_bytes_len(&self) -> usize {
                if <Self as sszb::SszEncode>::is_ssz_static() {
                    <Self as sszb::SszEncode>::ssz_fixed_len()
                } else {
                    let mut len: usize = 0;
                    #(
                        if #static_stmts {
                            len = len
                                .checked_add(#fixed_len_stmts)
                                .expect("encode ssz_bytes_len length overflow");
                        } else {
                            len = len
                                .checked_add(sszb::BYTES_PER_LENGTH_OFFSET)
                                .expect("encode ssz_bytes_len length overflow for offset");
                            len = len
                                .checked_add(#bytes_len_stmts)
                                .expect("encode ssz_bytes_len length overflow for bytes");
                        }
                    )*

                    len
                }
            }

            fn ssz_max_len() -> usize {
                let mut len: usize = 0;
                #(
                    len = len
                        .checked_add(#max_len_stmts)
                        .expect("encode ssz_max_len length overflow");
                )*
                len
            }

            fn ssz_write_fixed(&self, offset: &mut usize, buf: &mut impl BufMut) {
                // if self is fixed-sized then write the data outright
                // or else we write the offset to the buffer and point to the end of the buffer
                if <Self as sszb::SszEncode>::is_ssz_static() {
                    #(
                        #ssz_write_fixed_stmts;
                    )*
                } else {
                    buf.put_slice(&offset.to_le_bytes()[0..sszb::BYTES_PER_LENGTH_OFFSET]);
                    *offset += self.ssz_bytes_len();
                }
            }

            fn ssz_write_variable(&self, buf: &mut impl BufMut) {
                if !<Self as sszb::SszEncode>::is_ssz_static() {
                    self.ssz_write(buf);
                }
            }

            // using this function direct is most efficient with a &mut [u8]
            // the slice must have enough capacity, otherwise it will panic on failure
            // if used with a Vec<u8>, capacity may be increased automatically if needed
            fn ssz_write(&self, buf: &mut impl BufMut) {
                // offset is the length of all fixed size items
                // this lets us point dynamic length items to the area *after* the fixed size items
                let mut offset: usize = 0;
                #(
                    offset = offset
                        .checked_add(#fixed_len_stmts)
                        .expect("encode ssz_fixed_len length overflow");
                )*

                // first we write the fixed portion of each field in self
                // offset is passed to the write_fixed call in case we point to variable-sized data
                #(
                    #write_fixed_stmts;
                )*

                // then we write the variable portion into the buffer
                #(
                    #write_variable_stmts;
                )*
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SszbDecode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let struct_data = match derive_input.data {
        syn::Data::Struct(data) => data,
        _ => panic!(), // TODO: fix
    };
    let name = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();

    let fixed_len_stmts = &mut vec![];
    let static_stmts = &mut vec![];
    let max_len_stmts = &mut vec![];
    let read_stmts = &mut vec![];
    let read_stmts_var = &mut vec![];

    for (ty, ident, field_opts) in parse_ssz_fields(&struct_data) {
        let ident = match ident {
            Some(ref ident) => ident,
            _ => panic!(
                "#[ssz(struct_behaviour = \"container\")] only supports named struct fields."
            ),
        };

        if field_opts.iter().any(|opt| opt.skip_decode) {
            // should deserialize default
            read_stmts.push(quote! {
                #ident = <_>::default();
            });

            continue;
        }

        static_stmts.push(quote! { <#ty as sszb::SszDecode>::is_ssz_static() });
        fixed_len_stmts.push(quote! { <#ty as sszb::SszDecode>::ssz_fixed_len() });
        max_len_stmts.push(quote! { <#ty as sszb::SszDecode>::ssz_max_len() });
        read_stmts.push(quote! {
            #ident: <#ty as sszb::SszDecode>::ssz_read(fixed_bytes, variable_bytes)?
        });
        read_stmts_var.push(quote! {
            #ident: if <#ty as sszb::SszDecode>::is_ssz_static() {
                <#ty as sszb::SszDecode>::ssz_read(fixed_bytes, variable_bytes)?
            } else {
                // read the offset to advanced the fixed_bytes buffer
                sszb::read_offset_from_buf(fixed_bytes)?;
                // read the next length of the variable type
                // by calling .next() on offset_iter (defined in ssz_read)
                let field_len = offset_iter.next().unwrap();
                let bytes = variable_bytes.copy_to_bytes(field_len);
                // both the fixed and variable buffers are advanced at this point
                // even though we don't make a call to ssz_read with them
                <#ty as sszb::SszDecode>::from_ssz_bytes(&bytes)?
            }
        });
    }

    let output = quote! {
        impl #impl_generics sszb::SszDecode for #name #ty_generics #where_clause {
            fn is_ssz_static() -> bool {
                #(
                    #static_stmts &&
                )*
                    true
            }

            fn ssz_fixed_len() -> usize {
                if <Self as sszb::SszEncode>::is_ssz_static() {
                    let mut len: usize = 0;
                    #(
                        len = len
                            .checked_add(#fixed_len_stmts)
                            .expect("encode ssz_fixed_len length overflow");
                    )*
                    len
                } else {
                    sszb::BYTES_PER_LENGTH_OFFSET
                }
            }

            fn ssz_max_len() -> usize {
                let mut len: usize = 0;
                #(
                    len = len
                        .checked_add(#max_len_stmts)
                        .expect("encode ssz_max_len length overflow");
                )*
                len
            }

            fn ssz_read(fixed_bytes: &mut impl Buf, variable_bytes: &mut impl Buf) -> Result<Self, sszb::DecodeError>  {
                if <Self as sszb::SszDecode>::is_ssz_static() {
                    if fixed_bytes.remaining() < <Self as sszb::SszDecode>::ssz_fixed_len() {
                        return Err(sszb::DecodeError::InvalidByteLength {
                            len: fixed_bytes.remaining(),
                            expected: <Self as sszb::SszDecode>::ssz_fixed_len(),
                        });
                    }

                    Ok(Self {
                        #(
                            #read_stmts,
                        )*
                    })
                } else {
                    // to get the length of variable-sized items before the decoding call
                    // we have to instantiate an iterator that iterates over the fixed_bytes section
                    // and only returns the length between the *offsets*.
                    // the read_stmts will handle calling iter.next() to get the next variable length
                    let mut fixed_vec = fixed_bytes.chunk().to_vec();
                    let mut start_index: usize = 0;
                    #(
                        if #static_stmts {
                            // splice fixed_vec by fixed_len_stmts starting at start
                            let end_index: usize = start_index
                                .checked_add(#fixed_len_stmts)
                                .expect("ssz fixed length overflow");
                            // skip over fixed-sized types
                            fixed_vec.drain(start_index..end_index);
                        } else {
                            // retain offsets
                            start_index = start_index
                                .checked_add(sszb::BYTES_PER_LENGTH_OFFSET)
                                .expect("BYTES_PER_LENGTH_OFFSET overflow");
                        }
                    )*

                    // now we have a fixed_vec containing only offsets
                    // h/t the grandine team: https://github.com/grandinetech/grandine/blob/develop/ssz/src/shared.rs#L174
                    let mut offset_iter = fixed_vec
                        .chunks_exact(sszb::BYTES_PER_LENGTH_OFFSET)
                        .map(sszb::read_offset_from_slice)
                        .chain(core::iter::once(Ok(fixed_bytes.remaining() + variable_bytes.remaining())))
                        .tuple_windows()
                        .map(move |(start_result, end_result)| {
                            let start = start_result.unwrap();
                            let end = end_result.unwrap();
                            end - start
                        });

                    Ok(Self {
                        #(
                            #read_stmts_var,
                        )*
                    })
                }
            }

            fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, sszb::DecodeError> {
                let mut len: usize = 0;
                #(
                    len = len
                        .checked_add(#fixed_len_stmts)
                        .expect("decode ssz_fixed_len length overflow");
                )*
                let (mut fixed_bytes, mut variable_bytes) = bytes.split_at(len);
                <Self as SszDecode>::ssz_read(&mut fixed_bytes, &mut variable_bytes)
            }
        }
    };
    output.into()
}
