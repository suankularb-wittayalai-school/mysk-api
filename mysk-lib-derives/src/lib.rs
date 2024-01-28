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
            fn get_by_id(
                pool: &PgPool,
                id: Uuid,
            ) -> impl std::future::Future<Output = Result<Self, Error>> + Send
            where
                Self: Sized,
            {
                let query = format!("{} WHERE id = $1", Self::base_query());
                sqlx::query_as::<_, Self>(&query)
                    .bind(id)
                    .fetch_one(pool)
            }

            fn get_by_ids(
                pool: &PgPool,
                ids: Vec<Uuid>,
            ) -> impl std::future::Future<Output = Result<Vec<Self>, Error>> + Send
            where
                Self: Sized,
            {
                let query = format!("{} WHERE id = ANY($1)", Self::base_query());
                sqlx::query_as::<_, Self>(&query)
                    .bind(ids)
                    .fetch_all(pool)
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
