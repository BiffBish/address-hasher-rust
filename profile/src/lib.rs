// pub fn profile(attr: TokenStream, input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as ItemFn);
//     let attr = parse_macro_input!(attr as ExprBlock);

//     let (fn_name, args, ret_type, block) = match input {
//         ItemFn {
//             attrs,
//             vis,
//             sig,
//             block,
//             ..
//         } => (sig.ident, sig.inputs, sig.output, block),
//     };

//     let mut arg_names = vec![];
//     let mut arg_types = vec![];
//     for input in args {
//         match input {
//             BareFnArg::typed(arg) => {
//                 let name = match arg.pat {
//                     Pat::Ident(ident) => ident.ident,
//                     _ => panic!("Expected an identifier pattern"),
//                 };
//                 arg_names.push(name);
//                 arg_types.push(arg.ty);
//             }
//             _ => panic!("Expected a typed argument"),
//         }
//     }

//     let clone_vars = arg_names.iter().zip(arg_types).map(|(arg_name, arg_type)| {
//         let arg_clone_name = Ident::new(
//             &format!("{}_clone", arg_name),
//             proc_macro2::Span::call_site(),
//         );
//         quote! {
//             let #arg_clone_name: #arg_type = #arg_name.clone();
//         }
//     });

//     let profiled_fn_name = Ident::new(
//         &format!("__profile_{}", fn_name),
//         proc_macro2::Span::call_site(),
//     );

//     let call_profiled_fn = arg_names.iter().map(|arg_name| quote! { #arg_name });

//     let profiled_fn = quote! {
//         fn #profiled_fn_name(#(#args),*) #ret_type #block

//         #[allow(unused_variables)]
//         fn #fn_name(#(#args),*) #ret_type {
//             #(#clone_vars)*

//             #attr

//             #profiled_fn_name(#(#call_profiled_fn),*)
//         }
//     };

//     profiled_fn.into()
// }

// #[proc_macro_derive(Profile)]
// pub fn profile_macro_derive(input: TokenStream) -> TokenStream {
//     let input = syn::parse(input).unwrap();

//     impl_profile_macro(&input)
// }

// fn impl_profile_macro(ast: &DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let gen = quote! {
//         impl Profile for #name {
//             fn profile(&self) {
//                 println!("Profiled {}", stringify!(#name));
//             }
//         }
//     };
//     gen.into()
// }

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ItemFn};
#[cfg(feature = "profile")]
#[proc_macro_attribute]
pub fn profile(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_profile_macro(attr, ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(not(feature = "profile"))]
#[proc_macro_attribute]
pub fn profile(attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

fn impl_profile_macro(
    attr: TokenStream,
    ast: syn::ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    let (fn_name, args, ret_type, block) = match ast {
        ItemFn {
            attrs,
            vis,
            sig,
            block,
            ..
        } => (sig.ident, sig.inputs, sig.output, block),
        _ => panic!("Expected a function"),
    };
    let cln_args = args.clone();

    let mut arg_names = vec![];
    let mut arg_types = vec![];
    for input in cln_args {
        match input {
            syn::FnArg::Typed(arg) => {
                let name = match *arg.pat {
                    syn::Pat::Ident(ident) => ident.ident,
                    _ => panic!("Expected an identifier pattern"),
                };
                arg_names.push(name);
                arg_types.push(arg.ty);
            }
            _ => panic!("Expected a typed argument"),
        }
    }

    let clone_vars = arg_names.iter().zip(arg_types).map(|(arg_name, arg_type)| {
        let arg_clone_name = syn::Ident::new(
            &format!("{}_clone", arg_name),
            proc_macro2::Span::call_site(),
        );
        quote! {
            let #arg_clone_name: #arg_type = #arg_name.clone();
        }
    });

    let clone_clone_vars = clone_vars.clone();

    let profiled_fn_name = syn::Ident::new(
        &format!("__profile_{}", fn_name),
        proc_macro2::Span::call_site(),
    );
    // call the profiled function with the cloned variables. so just add _clone to the end of the variable name
    let call_profiled_fn_clone = arg_names.iter().map(|arg_name| {
        let arg_clone_name = syn::Ident::new(
            &format!("{}_clone", arg_name),
            proc_macro2::Span::call_site(),
        );
        quote! { #arg_clone_name }
    });
    let clone_call_profiled_fn_clone = call_profiled_fn_clone.clone();
    let call_profiled_fn = arg_names.iter().map(|arg_name| quote! { #arg_name });
    let clone_call_profiled_fn = call_profiled_fn_clone.clone();
    let atomic_bool = syn::Ident::new(
        &format!("{}_atomic_bool", fn_name),
        proc_macro2::Span::call_site(),
    );
    let profiled_fn: proc_macro2::TokenStream = quote! {
        #[allow(unused_variables)]
        static #atomic_bool: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
        fn #fn_name(#args) #ret_type {
            fn #profiled_fn_name(#args) #ret_type #block
            if #atomic_bool.swap(false, std::sync::atomic::Ordering::Relaxed) && !is_profiling.load(std::sync::atomic::Ordering::Relaxed) {
                let mut i = 1;
                let mut num_iterations = 1;

                #(#clone_clone_vars)*
                let start = std::time::Instant::now();
                #profiled_fn_name(#(#clone_call_profiled_fn_clone),*);
                let end = std::time::Instant::now();
                i = 1_000_000_000 / (end - start).as_nanos();
                if i == 0 {
                    i = 1;
                }
                num_iterations = i;

                let mut total_duration = std::time::Duration::new(0, 0);
                loop {
                    if i == 0 {
                        break;
                    }
                    #(#clone_vars)*
                    let start = std::time::Instant::now();
                    #profiled_fn_name(#(#call_profiled_fn_clone),*);
                    let end = std::time::Instant::now();
                    total_duration += end - start;
                    i -= 1;
                }

                // [Profiled function name]: [Average time per iteration in best fitting unit] {ns, us, ms, s} {# of iterations}
                let average_time_per_iteration = total_duration.as_nanos() / num_iterations;

                if average_time_per_iteration < 1_000 {
                    println!("[{}]: {}ns {:#?}", stringify!(#fn_name), average_time_per_iteration, num_iterations);
                } else if average_time_per_iteration < 1_000_000 {
                    println!("[{}]: {}us  {:#?}", stringify!(#fn_name), average_time_per_iteration / 1_000, num_iterations);
                } else if average_time_per_iteration < 1_000_000_000 {
                    println!("[{}]: {}ms {:#?}", stringify!(#fn_name), average_time_per_iteration / 1_000_000, num_iterations);
                } else {
                    println!("[{}]: {}s {:#?}", stringify!(#fn_name), average_time_per_iteration / 1_000_000_000, num_iterations);
                }
                is_profiling.store(false, std::sync::atomic::Ordering::Relaxed);
            }

            #profiled_fn_name(#(#call_profiled_fn),*)
        }
    };

    Ok(profiled_fn)
}
