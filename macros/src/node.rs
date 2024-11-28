use proc_macro2::{Ident as ProcIdent, Span};
use quote::quote;
use syn::Ident;

// Generates a wrapper for the Audio Node, binding it to JavaScript
pub fn node_wrapper(ident: &Ident) -> proc_macro2::TokenStream {
    // Create a new identifier for the Node struct
    let node_ident = ProcIdent::new(&format!("{ident}Node"), Span::call_site());

    // Generate the TokenStream for the wrapper
    quote! {
        // The `#[wasm_bindgen]` attribute binds the Rust struct to a JS object.
        #[wasm_bindgen(js_name = #ident)]
        pub struct #node_ident(waw::node::Node<#ident>);

        // The JS class name is bound to the struct via the `js_class` attribute.
        #[wasm_bindgen(js_class = #ident)]
        impl #node_ident {
            // `create` is an asynchronous function that creates a new Node instance.
            pub async fn create(
                ctx: waw::web_sys::AudioContext,
                initial_state: Option<<#ident as waw::worklet::AudioModule>::InitialState>
            ) -> Result<#node_ident, wasm_bindgen::JsValue> {
                let result = waw::node::Node::<#ident>::create(ctx, initial_state).await?;
                Ok(#node_ident(result))
            }

            // Returns the underlying AudioWorkletNode.
            pub fn node(&self) -> Result<waw::web_sys::AudioWorkletNode, wasm_bindgen::JsValue> {
                Ok(self.0.inner.clone())
            }

            // Get the AudioParam for a specific parameter of the module.
            pub fn get_param(&self, param: <#ident as waw::worklet::AudioModule>::Param) -> waw::web_sys::AudioParam {
                self.0.get_param(param)
            }

            // Sends a command to the module's processor.
            pub fn command(&self, message: <#ident as waw::worklet::AudioModule>::Command) {
                self.0.command(message)
            }

            // Subscribes to events from the module's processor.
            pub fn subscribe(
                &mut self,
                callback: waw::utils::callback::Callback<<#ident as waw::worklet::AudioModule>::Event>
            ) {
                self.0.subscribe(callback.0)
            }

            // Destroys the node, cleaning up resources.
            pub fn destroy(&mut self) {
                self.0.destroy();
            }
        }
    }
}
