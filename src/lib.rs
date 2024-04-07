//! Macros for computing SHA1 hashes at compile-time
//!
//! # Examples
//! ```rust
//! # use sha1_macros::*;
//! # use hex_literal::hex;
//! assert_eq!(sha1_hex!("this is a test"), "fa26be19de6bff93f70bc2308434e4a440bbad02");
//! assert_eq!(sha1_bytes!("this is a test"), hex!("fa26be19de6bff93f70bc2308434e4a440bbad02"));
//! ```

use proc_macro::{Literal, Punct, Spacing, TokenStream, TokenTree};
use sha1::{Digest, Sha1};
use syn::parse::{self, Parse, ParseStream};
use syn::{parse_macro_input, LitByteStr, LitStr};

enum Input {
    String(LitStr),
    Bytes(LitByteStr),
}

impl Input {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::String(x) => x.value().into_bytes(),
            Self::Bytes(x) => x.value(),
        }
    }
}

impl Parse for Input {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        if input.peek(LitStr) {
            Ok(Input::String(input.parse()?))
        } else if input.peek(LitByteStr) {
            Ok(Input::Bytes(input.parse()?))
        } else {
            Err(input.error("expected a string or byte literal"))
        }
    }
}

/// Computes the SHA1 hash as a hexadecimal string
///
/// The resulting value is of type `&'static str`.
/// ```rust
/// # use sha1_macros::sha1_hex;
/// assert_eq!(sha1_hex!("this is a test"), "fa26be19de6bff93f70bc2308434e4a440bbad02");
/// ```
#[proc_macro]
pub fn sha1_hex(tokens: TokenStream) -> TokenStream {
    sha1_impl(tokens, |hash| {
        let hash = hex::encode(hash);
        TokenTree::Literal(Literal::string(hash.as_ref())).into()
    })
}

/// Computes the SHA1 hash as a base64 unpadded string
///
/// The resulting value is of type `&'static str`.
/// ```rust
/// # use sha1_macros::sha1_base64;
/// assert_eq!(sha1_base64!("this is a test"), "+ia+Gd5r/5P3C8IwhDTkpEC7rQI");
/// ```
#[proc_macro]
pub fn sha1_base64(tokens: TokenStream) -> TokenStream {
    use base64::engine::general_purpose::STANDARD_NO_PAD;
    use base64::Engine;

    sha1_impl(tokens, |hash| {
        let hash = STANDARD_NO_PAD.encode(hash);
        TokenTree::Literal(Literal::string(hash.as_ref())).into()
    })
}

/// Computes the SHA1 hash as a byte array
///
/// The resulting value is of type `[u8; 20]`.
/// ```rust
/// # use sha1_macros::sha1_bytes;
/// # use hex_literal::hex;
/// assert_eq!(sha1_bytes!("this is a test"), hex!("fa26be19de6bff93f70bc2308434e4a440bbad02"));
/// ```
#[proc_macro]
pub fn sha1_bytes(tokens: TokenStream) -> TokenStream {
    sha1_impl(tokens, |hash| {
        TokenStream::from_iter([
            TokenTree::Punct(Punct::new('*', Spacing::Joint)),
            Literal::byte_string(hash).into(),
        ])
    })
}

fn sha1_impl(tokens: TokenStream, f: impl FnOnce(&[u8]) -> TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Input);
    let bytes = input.to_bytes();

    let mut hasher = Sha1::new();
    hasher.update(bytes.as_slice());

    let hash = hasher.finalize();
    f(hash.as_ref())
}
