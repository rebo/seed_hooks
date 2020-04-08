extern crate proc_macro;
use self::proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use quote::{quote,format_ident};
use syn::{parse_macro_input, Expr, ExprArray,  Token, };

struct PropertyLookUp {
    properties: ExprArray,
    values: ExprArray,
}

impl Parse for PropertyLookUp {
        fn parse(input: ParseStream) -> Result<Self> {
    
            let properties: ExprArray = input.parse()?;
            input.parse::<Token![,]>()?;
            let values: ExprArray = input.parse()?;
  
            Ok(PropertyLookUp {
                properties,
                values,
            })
        }
}


#[proc_macro]
pub fn generate_tw_sizes(input: TokenStream) -> TokenStream {
  
    let PropertyLookUp {
               properties,
               values,
           } = parse_macro_input!(input as PropertyLookUp);


let mut exp = quote!{};
    for property  in properties.elems.iter() {

        if let Expr::Lit(ref property) = property {
            if let syn::Lit::Str(ref property) = property.lit {
                
                for size_elem in values.elems.iter() {

                    if let Expr::Tuple(ref tuple) = size_elem {

                        let prop_size = tuple.elems.first().unwrap();
                        let css_size = tuple.elems.last().unwrap();

                        if let Expr::Lit(ref prop_size) = prop_size{
                            if let syn::Lit::Int(ref prop_size) = prop_size.lit{
                        


                    if let Expr::Lit(ref css_size) = css_size {
                        if let syn::Lit::Str(ref css_size) = css_size.lit {

              


                    let fnname_2 = format_ident!("{}_{}", property.value(), prop_size.base10_digits());
                   
                    // let complete_call = format_ident!("self.{}({})",call,css_size.value());
                    // fn #fnname_2(self)-> Sty { #complete_call} 
                    let css_size_value = css_size.value();
                    let property_call = format_ident!("{}",property.value());
                    let expanded = quote! {
                       #[track_caller]
                       pub fn #fnname_2(&self) -> Style { 
                           let mut new_style = self.clone();
                           new_style.updated_at.push(format!("{}", Location::caller()));   
                           new_style.#property_call(#css_size_value)
                        }
                    }; 
                    // println!("{}",expanded);
                    exp = quote!{
                        #exp
                        #expanded
                    };

                }
            }
            }
        }}


          
        }
    }
        }


    // let elem_ident = Ident::from(elem);
  
    }

    // let exp = quote!{};
    TokenStream::from(exp)

}


#[proc_macro]
pub fn generate_tw_colors(input: TokenStream) -> self::proc_macro::TokenStream {

    let PropertyLookUp {
               properties,
               values,
           } = parse_macro_input!(input as PropertyLookUp);


    let mut exp = quote!{};
    for property  in properties.elems.iter() {

        if let Expr::Tuple(ref tuple) = property {
            let tw_name = tuple.elems.first().unwrap();
            let variable_property = tuple.elems.last().unwrap();

            if let (Expr::Lit(ref tw_name) , Expr::Lit(ref variable_property)) =  (tw_name, variable_property){
                if let (syn::Lit::Str(ref tw_name), syn::Lit::Str(ref variable_property)) = (tw_name.lit.clone(),variable_property.lit.clone()) {
                for size_elem in values.elems.iter() {

                    if let Expr::Tuple(ref tuple) = size_elem {

                        let prop_size = tuple.elems.first().unwrap();
                        let css_size = tuple.elems.last().unwrap();

                        if let Expr::Lit(ref prop_size) = prop_size{
                            if let syn::Lit::Str(ref prop_size) = prop_size.lit{
                        


                    if let Expr::Lit(ref css_size) = css_size {
                        if let syn::Lit::Str(ref css_size) = css_size.lit {

              


                    let fnname_2 = format_ident!("{}_{}", tw_name.value(), prop_size.value());
                    
                    // let complete_call = format_ident!("self.{}({})",call,css_size.value());
                    // fn #fnname_2(self)-> Sty { #complete_call} 
                    let css_size_value = css_size.value();
                    let property_call = format_ident!("{}",variable_property.value());
                    
                    let expanded = quote!{
                       #[track_caller]
                       pub fn #fnname_2(&self) -> Style {
                            let mut new_style = self.clone();
                            new_style.updated_at.push(format!("{}", Location::caller()));   
                            new_style.#property_call(#css_size_value)
                        }
                    }; 

                    // println!("{}",expanded);
                    exp = quote!{
                        #exp
                        #expanded
                    };

                    // println!("{}",TokenStream::from(exp));
                }
            }
            }
        }
        }}

                


          
        }
    }
        }



    // let elem_ident = Ident::from(elem);
  
    }
    // let exp = quote!{};
    
    exp.into()
}

