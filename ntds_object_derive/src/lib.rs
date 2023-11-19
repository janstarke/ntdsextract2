use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(NTDSObject, attributes(ntds_attribute))]
pub fn ntds_object_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;


    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl TryFrom<&CDataTable> for #name {
            type Error = anyhow::Error;
            fn try_from(data_table: &CDataTable) -> Result<Self, Self::Error> {
                let mut temporary_mapping = HashMap::new();
                let mut column_names = HashMap::new();
                for index in 0..data_table.count_columns() {
                    let column_res = data_table.column(index).unwrap();
                    let col_info = ColumnInformation::new(
                        index,
                        // column_res.name()?,
                        // column_res.variant()?
                    );
                    column_names.insert(index, column_res.name().to_owned());
                    temporary_mapping.insert(column_res.name(), col_info);
                    //log::info!("found column with name {name}", name=column_res.name());
                }

                Self {

                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
