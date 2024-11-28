use proc_macro2::{Ident as ProcIdent, Span};
use quote::quote;
use syn::Ident;

// Generates a wrapper for an Audio Worklet Processor in WebAssembly
pub fn worklet_wrapper(ident: &Ident) -> proc_macro2::TokenStream {
    // Create a new identifier for the processor, suffixed with "Processor"
    let worklet_ident = ProcIdent::new(&format!("_{ident}Processor"), Span::call_site());
    let worklet_ident_name = worklet_ident.to_string();

    // Generate the final TokenStream to be used in the macro
    quote! {
        // Import necessary WebAssembly bindings and modules
        use wasm_bindgen::prelude::*;
        
        // Define the struct wrapping the processor for the given module
        #[wasm_bindgen]
        pub struct #worklet_ident(waw::worklet::Processor<#ident>);

        // Implement methods for the Worklet Processor
        #[wasm_bindgen]
        impl #worklet_ident {
            // Constructor to initialize the Worklet Processor
            #[wasm_bindgen(constructor)]
            pub fn new(
                js_processor: waw::web_sys::AudioWorkletProcessor,
                initial_state: Option<<#ident as waw::worklet::AudioModule>::InitialState>
            ) -> Self {
                // Create an emitter and processor for the given module
                let emitter = waw::worklet::Emitter::<
                    <#ident as waw::worklet::AudioModule>::Event
                >::new(js_processor.port().unwrap());

                // Return the initialized Worklet Processor
                #worklet_ident(waw::worklet::Processor::new(
                    #ident::create(initial_state, emitter),
                    js_processor
                ))
            }

            // Method to connect the processor
            pub fn connect(&mut self) {
                self.0.connect();
            }

            // Method to process input, output, and parameters
            pub fn process(
                &mut self,
                input: &waw::js_sys::JsValue,
                output: &waw::js_sys::Array,
                params: &wasm_bindgen::JsValue
            ) -> bool {
                self.0.process(input, output, params)
            }

            // Retrieve the parameter descriptor for the processor
            pub fn parameter_descriptor() -> String {
                <#ident as waw::types::AudioModuleDescriptor>::parameter_descriptor_json()
            }
        }

        // Implement AudioModuleDescriptor for the provided module
        impl waw::types::AudioModuleDescriptor for #ident {
            fn processor_name() -> &'static str {
                &#worklet_ident_name
            }

            // Get the JSON descriptor for the parameters of the module
            fn parameter_descriptor_json() -> String {
                waw::serde_json::to_string(
                    &<<#ident as waw::worklet::AudioModule>::Param as waw::types::ParameterDescriptor>::descriptors()
                ).unwrap()
            }
        }
    }
}
