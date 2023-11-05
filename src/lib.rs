#![doc=include_str!("../README.md")]

use proc_macro::TokenStream;

/// This macro will yield an expression of type &'static [[u8; N]] which is the output of the command.
/// 
/// ```rust, no_run
/// let hash = cmd_execute!("git rev-parse --short HEAD");
/// let date = cmd_execute!("git log -1 --format=%cd");
/// let latest_tag = cmd_execute!("git describe --tags --abbrev=0");
/// let sub_version = cmd_execute!("git rev-list `git describe --tags --abbrev=0` ..HEAD --count --first-parent");
/// ```
#[proc_macro]
pub fn cmd_execute(input: TokenStream) -> TokenStream {
    let input: syn::LitStr = syn::parse(input).unwrap();

    #[cfg(target_os="windows")]
    let sh = "cmd";
    #[cfg(not(target_os="windows"))]
    let sh = "bash";

    let mut cmd = std::process::Command::new(sh);

    #[cfg(target_os="windows")]
    cmd.arg("/c");
    #[cfg(not(target_os="windows"))]
    cmd.arg("-c");

    cmd.arg(input.value());
    let output = match cmd.output() {
        Ok(out) => out,
        Err(e) => panic!("{}", e),
    };
    // println!("output: {:?}", output);
    if !output.status.success() {
        panic!("The command's output is: {:?}", output);
    }

    let stdout = output.stdout;

    quote::quote! {
        &[
            #(#stdout,)*
        ]
    }.into()
}


