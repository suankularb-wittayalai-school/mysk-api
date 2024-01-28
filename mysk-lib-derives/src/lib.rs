use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(base_query))]
struct BaseQueryOpts {
    query: String,
}

#[proc_macro_derive(GetById)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;

    let expanded = quote! {
        impl GetById for #ident {
            async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
                sqlx::query_as::<_, #ident>(format!("{} WHERE id = $1", Self::base_query()).as_str())
                    .bind(id)
                    .fetch_one(pool)
                    .await
            }

            async fn get_by_ids(pool: &sqlx::PgPool, id: Vec<Uuid>) -> Result<Vec<Self>, sqlx::Error> {
                sqlx::query_as::<_, #ident>(format!("{} WHERE id = ANY($1)", Self::base_query()).as_str())
                    .bind(id)
                    .fetch_all(pool)
                    .await
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(BaseQuery, attributes(base_query))]
pub fn derive_base_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = BaseQueryOpts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let query = opts.query;

    let expanded = quote! {
        impl BaseQuery for #ident {
            fn base_query() -> &'static str {
                #query
            }
        }
    };

    expanded.into()
}
