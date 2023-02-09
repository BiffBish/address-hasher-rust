#[allow(unused, dead_code)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ItemFn, Type};

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
pub fn profile(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[allow(dead_code)]
fn impl_profile_macro(
    input_params: TokenStream,
    ast: syn::ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    let (fn_name, og_args, ret_type, block) = match ast {
        ItemFn { sig, block, .. } => (sig.ident, sig.inputs, sig.output, block),
        // _ => panic!("Expected a function"),
    };
    let cln_args = og_args.clone();

    // input_param examples:
    // 1. #[profile()]
    // 2. #[profile(no_sub)]
    // 3. #[profile(no_sub, prefix = "my_prefix")]
    // 4. #[profile(prefix = "my_prefix")]
    // 5. #[profile(prefix = "my_prefix", no_sub)]
    let input_params = input_params.to_string();
    let input_params = input_params.trim();
    let input_params = input_params.trim_matches(|c| c == '[' || c == ']');

    // split by comma
    let input_params: Vec<&str> = input_params.split(',').collect();

    let mut no_subtree = false;
    // let mut prefix = None;

    for param in input_params {
        let param = param.trim();
        if param == "no_sub" {
            no_subtree = true;
        } else if param.starts_with("prefix") {
            // let param = param.trim_matches(|c| c == ' ' || c == '"');
            // let param = param.trim_start_matches("prefix=");
            // prefix = Some(param.to_string());
        } else if param != "" {
            panic!("Invalid parameter: {}", param);
        }
    }

    struct Arg {
        name: syn::Ident,
        ty: Option<Box<syn::Type>>,
        is_ref: bool,
        is_mut: bool,
    }

    let mut args: Vec<Arg> = vec![];
    let mut is_self = false;
    for input in cln_args {
        match input {
            syn::FnArg::Typed(arg) => {
                let name = match *arg.pat {
                    syn::Pat::Ident(ident) => ident.ident,
                    _ => panic!("Expected an identifier pattern"),
                };
                // the name will always be "self"

                args.push(Arg {
                    name,
                    ty: Some(arg.ty.clone()),
                    is_ref: match *arg.ty {
                        Type::Reference(_) => true,
                        _ => false,
                    },
                    is_mut: match *arg.ty {
                        Type::Reference(ref_type) => match ref_type.mutability {
                            Some(_) => true,
                            None => false,
                        },
                        _ => false,
                    },
                });
                // arg_names.push(name);
                // arg_types.push(arg.ty);
            }
            syn::FnArg::Receiver(arg) => {
                is_self = true;
                args.push(Arg {
                    name: proc_macro2::Ident::new("self", proc_macro2::Span::call_site()),
                    ty: None,
                    is_ref: match arg.reference {
                        Some(_) => true,
                        None => false,
                    },
                    is_mut: match arg.mutability {
                        Some(_) => true,
                        None => false,
                    },
                });
            }
            _ => panic!("Expected a typed argument"),
        }
    }

    let clone_vars = args.iter().map(|arg| {
        let arg_name = arg.name.clone();
        let arg_clone_name = syn::Ident::new(
            &format!("{}_clone", arg_name),
            proc_macro2::Span::call_site(),
        );
        let arg_ty = arg.ty.clone();

        if arg.is_ref {
            if arg.is_mut {
                quote! { let mut #arg_clone_name = #arg_name.clone(); }
            } else {
                quote! { let #arg_clone_name = #arg_name.clone(); }
            }
        } else {
            quote! { let #arg_clone_name = #arg_name.clone(); }
        }
    });

    // let clone_clone_vars = clone_vars.clone();

    let profiled_fn_name = if is_self {
        let ident = syn::Ident::new(
            &format!("{}_profiled", fn_name),
            proc_macro2::Span::call_site(),
        );

        quote! { #ident }
    } else {
        let ident = syn::Ident::new(
            &format!("{}_profiled", fn_name),
            proc_macro2::Span::call_site(),
        );
        quote! { #ident }
    };

    let call_profiled_fn = if is_self {
        let ident = syn::Ident::new(
            &format!("{}_profiled", fn_name),
            proc_macro2::Span::call_site(),
        );

        quote! { self.#ident }
    } else {
        let ident = syn::Ident::new(
            &format!("{}_profiled", fn_name),
            proc_macro2::Span::call_site(),
        );
        quote! { #ident }
    };
    let call_profiled_fn1 = call_profiled_fn.clone();
    let call_profiled_fn2 = call_profiled_fn.clone();
    let call_profiled_fn3 = call_profiled_fn.clone();
    let call_profiled_fn4 = call_profiled_fn.clone();

    let clone_call_profiled_fn = if is_self {
        let ident = syn::Ident::new(
            &format!("{}_profiled", fn_name),
            proc_macro2::Span::call_site(),
        );

        quote! { self_clone.#ident }
    } else {
        let ident = syn::Ident::new(
            &format!("{}_profiled", fn_name),
            proc_macro2::Span::call_site(),
        );
        quote! { #ident }
    };

    let call_profiled_fn_args_before = args.iter().map(|arg| {
        let arg_name = arg.name.clone();
        quote! { black_box(#arg_name) }
    });
    let call_profiled_fn_args = if is_self {
        call_profiled_fn_args_before
            .skip(1)
            .collect::<Vec<_>>()
            .into_iter()
    } else {
        call_profiled_fn_args_before.collect::<Vec<_>>().into_iter()
    };

    let call_profiled_fn_args1 = call_profiled_fn_args.clone();
    let call_profiled_fn_args2 = call_profiled_fn_args.clone();
    let call_profiled_fn_args3 = call_profiled_fn_args.clone();
    let call_profiled_fn_args4 = call_profiled_fn_args.clone();
    // call the profiled function with the cloned variables. so just add _clone to the end of the variable name
    let call_profiled_fn_clone_args_before = args.iter().map(|arg| {
        let arg_name = arg.name.clone();
        let arg_clone_name = syn::Ident::new(
            &format!("{}_clone", arg_name),
            proc_macro2::Span::call_site(),
        );

        if arg.is_ref {
            if arg.is_mut {
                quote! { black_box(&mut #arg_clone_name) }
            } else {
                quote! { black_box(&#arg_clone_name) }
            }
        } else {
            quote! { black_box(#arg_clone_name) }
        }
    });
    // if we are is_self then remove the first arg from the call
    let call_profiled_fn_clone_args = if is_self {
        call_profiled_fn_clone_args_before
            .skip(1)
            .collect::<Vec<_>>()
            .into_iter()
    } else {
        call_profiled_fn_clone_args_before
            .collect::<Vec<_>>()
            .into_iter()
    };

    let disable_profiling_before = if no_subtree {
        quote! {}
    } else {
        quote! { IS_PROFILING.store(false, std::sync::atomic::Ordering::Relaxed); }
    };

    let disable_profiling_after = if no_subtree {
        quote! { IS_PROFILING.store(false, std::sync::atomic::Ordering::Relaxed); }
    } else {
        quote! {}
    };

    let profiled_fn: proc_macro2::TokenStream = quote! {
        fn #profiled_fn_name(#og_args) #ret_type #block
        pub fn #fn_name(#og_args) #ret_type {
            if !IS_PROFILING.load(std::sync::atomic::Ordering::Relaxed)  {
                if IS_PROFILE_RECONCILING.load(std::sync::atomic::Ordering::Relaxed) {
                    unsafe {
                        let this_path = (*PROFILING_PATH.load(std::sync::atomic::Ordering::Relaxed)).join(".");
                        let fn_path = format!("{}.{}", this_path, stringify!(#fn_name));
                        let profile_data = (*PROFILING_MAP.load(std::sync::atomic::Ordering::Relaxed)).get(&fn_path);

                        match profile_data {
                            Some(data) => {
                                let prefix = data.depth;
                                let mut prefix_str = String::new();
                                for _ in 0..prefix {
                                    prefix_str.push_str("    ");
                                }

                                let average_time = data.speed;
                                let speed_string;
                                if average_time < 1_000 {
                                    speed_string = format!("{}ns", average_time);
                                } else if average_time < 100_000 {
                                    speed_string = format!("{}us", (average_time) as f64 / 1000.0);
                                } else if average_time < 1_000_000 {
                                    speed_string = format!("{}us", average_time / 1_000);
                                } else if average_time < 100_000_000 {
                                    speed_string = format!("{}ms", (average_time / 1_000) as f64 / 1000.0);
                                } else {
                                    speed_string = format!("{}ms", average_time / 1_000_000);
                                }



                                if (data.count > 1) {
                                    let total_speed = data.speed * data.count;
                                    let total_string;
                                    if total_speed < 1_000 {
                                        total_string = format!("{}ns", total_speed);
                                    } else if total_speed < 100_000 {
                                        total_string = format!("{}us", (total_speed) as f64 / 1000.0);
                                    } else if total_speed < 1_000_000 {
                                        total_string = format!("{}us", total_speed / 1_000);
                                    } else if total_speed < 100_000_000 {
                                        total_string = format!("{}ms", (total_speed / 1_000) as f64 / 1000.0);
                                    } else {
                                        total_string = format!("{}ms", total_speed / 1_000_000);
                                    }
                                    let count_str = format!("{}", data.count);
                                    println!("{} [{}]: {} ({} x {})", prefix_str, data.fn_name.magenta(), total_string.green(), speed_string.blue(), count_str.cyan());
                                } else{
                                    println!("{} [{}]: {}", prefix_str, data.fn_name.magenta(), speed_string.green());
                                }

                                // Remove this key from the map
                                (*PROFILING_MAP.load(std::sync::atomic::Ordering::Relaxed)).remove(&fn_path);
                            },
                            None => {
                                // println!("{}[{}]: {}", "", stringify!(#fn_name), &fn_path);
                            }
                        }
                    }
                    unsafe {
                        (*PROFILING_PATH.load(std::sync::atomic::Ordering::Relaxed)).push(stringify!(#fn_name).to_string());
                    }
                    let res = #call_profiled_fn1(#(#call_profiled_fn_args1),*);
                    unsafe {
                        (*PROFILING_PATH.load(std::sync::atomic::Ordering::Relaxed)).pop();
                    }
                    return res;
                }
                unsafe{
                    let ptr = PROFILING_PATH.load(std::sync::atomic::Ordering::Relaxed);
                    if ptr.is_null() {
                        // Initialize the profiling path to a vector of strings
                        let mut profiling_path = Vec::new();
                        PROFILING_PATH.store(Box::into_raw(Box::new(profiling_path)), std::sync::atomic::Ordering::Relaxed);
                    }
                    let ptr = PROFILING_MAP.load(std::sync::atomic::Ordering::Relaxed);
                    if ptr.is_null() {
                        // Profiling Map is a map of String and {
                        //     count: u64,
                        //     speed: Duration,
                        // }
                        let profiling_map = std::collections::HashMap::new();
                        PROFILING_MAP.store(Box::into_raw(Box::new(profiling_map)), std::sync::atomic::Ordering::Relaxed);
                    }
                }

                let fn_path;
                unsafe {
                    let this_path = (*PROFILING_PATH.load(std::sync::atomic::Ordering::Relaxed)).join(".");

                    fn_path = format!("{}.{}", this_path, stringify!(#fn_name));

                    if (*PROFILING_MAP.load(std::sync::atomic::Ordering::Relaxed)).contains_key(&fn_path) {
                        // Profiling Map is a map of String and {
                        //     count: u64,
                        //     speed: Duration,
                        // }
                        (*PROFILING_MAP.load(std::sync::atomic::Ordering::Relaxed)).entry(fn_path).and_modify(|e| {
                            e.count += 1;
                        });
                        return #call_profiled_fn2(#(#call_profiled_fn_args2),*)
                    }


                }

                IS_PROFILING.store(true, std::sync::atomic::Ordering::Relaxed);
                let prefix = PROFILING_DEPTH.load(std::sync::atomic::Ordering::Relaxed);
                let mut prefix_str = String::new();
                for _ in 0..prefix {
                    prefix_str.push_str("    ");
                }

                let mut num_iterations = 1;
                let mut total_duration = std::time::Duration::new(0, 0);




                loop {
                    #(#clone_vars)*
                    // b.iter(|| workload());
                    // time the fn and make sure to use black box to prevent the compiler from optimizing it out
                    let start = std::time::Instant::now();
                    black_box(#clone_call_profiled_fn(#(#call_profiled_fn_clone_args),*));
                    let end = std::time::Instant::now();
                    // println!("#[{}]: {} ns",stringify!(#fn_name),  (end - start).as_nanos());
                    total_duration += end - start;
                    // total_duration is greater then 2 seconds break
                    if total_duration.as_secs() > 1 {
                        break;
                    }
                    num_iterations += 1;
                    if num_iterations > 10_000 {
                        break;
                    }
                }
                let average_time_per_iteration = total_duration.as_nanos() / num_iterations;

                unsafe{
                    (*PROFILING_MAP.load(std::sync::atomic::Ordering::Relaxed)).insert(fn_path, Profile {
                        count: 1,
                        fn_name: stringify!(#fn_name).to_string(),
                        speed: average_time_per_iteration,
                        depth: prefix,
                    });
                }

                // if no_sub
                #disable_profiling_before
                PROFILING_DEPTH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                unsafe {
                    (*PROFILING_PATH.load(std::sync::atomic::Ordering::Relaxed)).push(stringify!(#fn_name).to_string());
                }
                let res = #call_profiled_fn4(#(#call_profiled_fn_args4),*);
                PROFILING_DEPTH.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                unsafe {
                    (*PROFILING_PATH.load(std::sync::atomic::Ordering::Relaxed)).pop();
                }
                #disable_profiling_after
                return res
            }
            #call_profiled_fn3(#(#call_profiled_fn_args3),*)
        }
    };

    Ok(profiled_fn)
}

// #[cfg(not(feature = "profile"))]
// #[proc_macro_attribute]
// pub fn profile_method(options: TokenStream, input: TokenStream) -> TokenStream {
//     input
// }
// #[cfg(feature = "profile")]
// #[proc_macro_attribute]
// pub fn profile_method(attr: TokenStream, input: TokenStream) -> TokenStream {
//     let ast = syn::parse(input).unwrap();

//     impl_profile_macro(attr, ast)
//         .unwrap_or_else(syn::Error::into_compile_error)
//         .into()
// }

// #[allow(dead_code)]
// fn impl_profile_method_macro(
//     options: TokenStream,
//     ast: syn::ItemFn,
// ) -> syn::Result<proc_macro2::TokenStream> {
//     // we will pass in the class name into the options
//     let options = syn::parse::<syn::LitStr>(options).unwrap();
//     let class_name = options.value();

//     let (fn_name, args, ret_type, block) = match ast {
//         ItemFn { sig, block, .. } => (sig.ident, sig.inputs, sig.output, block),
//         _ => panic!("Expected a function"),
//     };

//     // go through the args and replace self with the class name
//     let cln_args = args.clone();

//     let mut arg_names = vec![];
//     let mut arg_types: Vec<Option<Box<Type>>> = vec![];
//     let mut method = false;
//     for input in cln_args {
//         match input {
//             syn::FnArg::Typed(arg) => {
//                 let name = match *arg.pat {
//                     syn::Pat::Ident(ident) => ident.ident,
//                     _ => panic!("Expected an identifier pattern"),
//                 };
//                 // the name will always be "self"
//                 arg_names.push(name);
//                 arg_types.push(Some(arg.ty));
//             }
//             syn::FnArg::Receiver(arg) => {
//                 method = true;
//                 arg_names.push(arg.self_token.into());
//                 arg_types.push(None);
//             }
//             _ => panic!("Expected a typed argument"),
//         }
//     }

//     let clone_vars = arg_names.iter().zip(arg_types).map(|(arg_name, arg_type)| {
//         let arg_clone_name = syn::Ident::new(
//             &format!("{}_clone", arg_name),
//             proc_macro2::Span::call_site(),
//         );
//         if let Some(arg_type) = arg_type {
//             quote! {
//                 let #arg_clone_name: #arg_type = #arg_name.clone();
//             }
//         } else {
//             quote! {
//                 let #arg_clone_name = #arg_name.clone();
//             }
//         }
//     });

//     // let clone_clone_vars = clone_vars.clone();

//     let profiled_fn_name = syn::Ident::new(
//         &format!("__profile_{}", fn_name),
//         proc_macro2::Span::call_site(),
//     );
//     let clone_profiled_fn_name = profiled_fn_name.clone();

//     // call the profiled function with the cloned variables. so just add _clone to the end of the variable name
//     let call_profiled_fn_clone = arg_names.iter().map(|arg_name| {
//         let arg_clone_name = syn::Ident::new(
//             &format!("{}_clone", arg_name),
//             proc_macro2::Span::call_site(),
//         );
//         quote! { #arg_clone_name }
//     });
//     // let clone_call_profiled_fn_clone = call_profiled_fn_clone.clone();
//     let call_profiled_fn = arg_names.iter().map(|arg_name| quote! { #arg_name });
//     let clone_call_profiled_fn = call_profiled_fn.clone();
//     let atomic_bool = syn::Ident::new(
//         &format!("{}_atomic_bool", fn_name),
//         proc_macro2::Span::call_site(),
//     );

//     let atomic_book = match method {
//         true => quote! {},
//         false => quote! {
//             #[allow(unused_variables)]
//             static #atomic_bool: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
//         },
//     };

//     let profiled_fn: proc_macro2::TokenStream = quote! {
//         #atomic_book
//         pub fn #fn_name(#args) #ret_type {
//             fn #profiled_fn_name(#args) #ret_type #block

//             if !IS_PROFILING.load(std::sync::atomic::Ordering::Relaxed) && #atomic_bool.swap(false, std::sync::atomic::Ordering::Release) {
//                 IS_PROFILING.store(true, std::sync::atomic::Ordering::Relaxed);
//                 let prefix = PROFILING_DEPTH.load(std::sync::atomic::Ordering::Relaxed);
//                 let mut prefix_str = String::new();
//                 for _ in 0..prefix {
//                     prefix_str.push_str("    ");
//                 }

//                 // IS_PROFILING.store(true, std::sync::atomic::Ordering::Relaxed);
//                 let mut num_iterations = 1;
//                 let mut total_duration = std::time::Duration::new(0, 0);
//                 loop {
//                     #(#clone_vars)*
//                     let start = std::time::Instant::now();
//                     #profiled_fn_name(#(#call_profiled_fn_clone),*);
//                     let end = std::time::Instant::now();
//                     // println!("#[{}]: {} ns",stringify!(#fn_name),  (end - start).as_nanos());
//                     total_duration += end - start;
//                     // total_duration is greater then 2 seconds break
//                     if total_duration.as_secs() > 2 {
//                         break;
//                     }
//                     num_iterations += 1;
//                 }
//                 // Total time: [Total time in best fitting unit] {ns, us, ms, s}
//                 // println!("{}#[{}]: {}ns / {}",prefix_str, stringify!(#fn_name), total_duration.as_millis(), num_iterations);

//                 // [Profiled function name]: [Average time per iteration in best fitting unit] {ns, us, ms, s} {# of iterations}
//                 let average_time_per_iteration = total_duration.as_nanos() / num_iterations;

//                 if average_time_per_iteration < 1_000 {
//                     println!("{}[{}]: {}ns ",prefix_str, stringify!(#fn_name), average_time_per_iteration);
//                 } else if average_time_per_iteration < 1_000_000 {
//                     println!("{}[{}]: {}us  ",prefix_str, stringify!(#fn_name), average_time_per_iteration / 1_000);
//                 } else if average_time_per_iteration < 1_000_000_000 {
//                     println!("{}[{}]: {}ms ",prefix_str, stringify!(#fn_name), average_time_per_iteration / 1_000_000);
//                 } else {
//                     println!("{}[{}]: {}s ",prefix_str, stringify!(#fn_name), average_time_per_iteration / 1_000_000_000);
//                 }
//                 IS_PROFILING.store(false, std::sync::atomic::Ordering::Relaxed);
//                 PROFILING_DEPTH.fetch_add(1, std::sync::atomic::Ordering::Release);
//                 let res = #profiled_fn_name(#(#call_profiled_fn),*);
//                 PROFILING_DEPTH.fetch_sub(1, std::sync::atomic::Ordering::Release);
//                 res
//             } else {
//                 #clone_profiled_fn_name(#(#clone_call_profiled_fn),*)
//             }
//         }
//     };

//     Ok(profiled_fn)
// }

// a macro that takes a method name and returns
// #[allow(unused_variables)]
// const {name}_atomic_bool: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(
//     true,
// );
