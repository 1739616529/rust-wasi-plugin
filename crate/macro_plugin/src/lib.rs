use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn rtools_plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct: ItemStruct = parse_macro_input!(item);
    let struct_name = &item_struct.ident;

    let ts = quote! {
        use std::sync::OnceLock;
        static PLUGIN_INSTANCE: OnceLock<#struct_name> = OnceLock::new();

        #[no_mangle]
        pub extern "C" fn _plugin_create() {
            PLUGIN_INSTANCE.get_or_init(|| {
                #struct_name::new()
            });
        }

        #[no_mangle]
        pub extern "C" fn _plugin_handle_message(message: *const u8, message_len: usize) {
            use plugin::get_string_from_pointer;
            let message_str = get_string_from_pointer(message, message_len);
            PLUGIN_INSTANCE.get().unwrap().handle_message(message_str);
        }

        #item_struct
    };

    ts.into()
}
