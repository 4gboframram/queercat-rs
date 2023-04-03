extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use litrs::IntegerLit;
#[derive(Clone, Copy)]
#[repr(transparent)]
struct HexColor(u32);


impl HexColor {
   
    pub fn red(&self) -> f32 {
        (((self.0 & 0xff0000) >> 16) as f32) / 255.0
    }
    pub fn green(&self) -> f32 {
        ((self.0 & 0x00ff00) >> 8) as f32 / 255.0
    }
    pub fn blue(&self) -> f32 {
        (self.0 & 0x0000ff) as f32 / 255.0
    }
    
}

#[proc_macro_attribute]
pub fn stripes(mut attr: TokenStream, item: TokenStream) -> TokenStream {
    // dbg!(&item);
    if attr.is_empty() {
        attr = "::queercat".parse().unwrap()
    }

    let mut iter = item.into_iter();
    let mut out_stream: Vec<TokenTree> = Vec::new();
    let kind = iter.next().unwrap(); // let, static, or const
    out_stream.push(kind);
    let ident = iter.next().unwrap();
    let _ = iter.next().unwrap(); // :
    let _ = iter.next().unwrap(); // type that gets discarded
    out_stream.push(ident);
    
    let equals = match iter.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == '=' => p,
        _ => panic!("expected let/static/const {{ident}}: _ = {{array}}")
    };
    
    let arr = match iter.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == proc_macro::Delimiter::Bracket => g,
        _ =>  panic!("expected let/static/const {{ident}}: _ = {{array}}")
    };

    out_stream.extend(format!(": [{attr}::color::Color; ({}).len()]", arr.clone()).parse::<TokenStream>().expect("a"));
    
    out_stream.push(TokenTree::Punct(equals));
    
    let stream = arr.stream();
    let mut arr_stream = TokenStream::new();
    
    for tok in stream {
        match tok {
            TokenTree::Literal(l) => {
                use proc_macro::Literal as L;
                let l = IntegerLit::try_from(l).expect("expected an integer literal");
                let v = HexColor(l.value::<u32>().unwrap());
                let (r, g, b) = (v.red(), v.green(), v.blue());
                let (r, g, b) = (L::f32_suffixed(r), L::f32_suffixed(g), L::f32_suffixed(b));
                arr_stream.extend(
                    format!("{attr}::color::Color::new({r}, {g}, {b})").parse::<TokenStream>().expect("a")
                )
                
            },
            TokenTree::Punct(p) => {
                arr_stream.extend(
                    std::iter::once(TokenTree::Punct(p))
                )
            },
            TokenTree::Group(g) if g.delimiter() == proc_macro::Delimiter::None => {
                for tok in g.stream() {
                    use proc_macro::Literal as L;
                let l = IntegerLit::try_from(tok).expect("expected an integer literal");
                let v = HexColor(l.value::<u32>().unwrap());
                let (r, g, b) = (v.red(), v.green(), v.blue());
                let (r, g, b) = (L::f32_suffixed(r), L::f32_suffixed(g), L::f32_suffixed(b));
                arr_stream.extend(
                    format!("{attr}::color::Color::new({r}, {g}, {b})").parse::<TokenStream>().expect("a")
                )
                }
            }
            p => panic!("expected let/static/const {{ident}}: _ = {{array}}, got {p}")
        }
    }

    let arr = proc_macro::Group::new(proc_macro::Delimiter::Bracket, arr_stream);
    out_stream.push(TokenTree::Group(arr));
    out_stream.push(TokenTree::Punct(proc_macro::Punct::new(';', proc_macro::Spacing::Alone)));

    let mut out = TokenStream::new();
    out.extend(out_stream.iter().cloned());
   out
    
}