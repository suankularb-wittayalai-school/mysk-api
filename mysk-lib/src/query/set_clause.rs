use super::{
    fragment::{query_param_push_bind, QueryParamType},
    QueryFragment,
};
use crate::common::{requests::QueryParam, string::FlexibleMultiLangString};
use sqlx::{Postgres, QueryBuilder};

/// A collection of multiple `QueryFragment`s.
#[derive(Debug)]
pub struct SqlSetClause<'sql>(Vec<QueryFragment<'sql>>);

impl<'sql> SqlSetClause<'sql> {
    pub fn new() -> Self {
        SqlSetClause(vec![QueryFragment::Sql(" SET ")])
    }

    pub fn new_fragment() -> Self {
        SqlSetClause(Vec::new())
    }

    pub fn push_sql(mut self, sql: &'sql str) -> Self {
        self.0.push(QueryFragment::Sql(sql));

        self
    }

    pub fn push_param(mut self, param: QueryParam) -> Self {
        self.0.push(QueryFragment::Param(param));

        self
    }

    pub fn push_prev_param(mut self) -> Self {
        self.0.push(QueryFragment::PreviousParam);

        self
    }

    pub fn push_sep(mut self) -> Self {
        self.0.push(QueryFragment::Separator);

        self
    }

    pub fn push_multilang_update_field(
        self,
        column: &'sql str,
        param: Option<FlexibleMultiLangString>,
    ) -> Self {
        if let Some(param) = param {
            let s = if let Some(th) = param.th {
                self.push_sep()
                    .push_sql(column)
                    .push_sql("_th = COALESCE(")
                    .push_param(QueryParam::String(th))
                    .push_sep()
                    .push_sql(column)
                    .push_sql("_th)")
            } else {
                self
            };

            if let Some(en) = param.en {
                s.push_sep()
                    .push_sql(column)
                    .push_sql("_en = COALESCE(")
                    .push_param(QueryParam::String(en))
                    .push_sep()
                    .push_sql(column)
                    .push_sql("_en)")
            } else {
                s
            }
        } else {
            self
        }
    }

    pub fn push_update_field<T: QueryParamType>(
        self,
        column: &'sql str,
        param: Option<T>,
        make_variant: impl FnOnce(T) -> QueryParam,
    ) -> Self {
        let s = if (self.0.last().unwrap() != &QueryFragment::Sql(" SET ")) && param.is_some() {
            self.push_sep()
        } else {
            self
        };

        s.push_update_field_no_sep(column, param, make_variant)
    }

    pub fn push_update_field_no_sep<T: QueryParamType>(
        self,
        column: &'sql str,
        param: Option<T>,
        make_variant: impl FnOnce(T) -> QueryParam,
    ) -> Self {
        if let Some(param) = param {
            let s = self
                .push_sql(column)
                .push_sql(" = COALESCE(")
                .push_param(make_variant(param))
                .push_sep()
                .push_sql(column)
                .push_sql(")");

            s
        } else {
            self
        }
    }

    pub fn push_if_some<T: QueryParamType>(
        mut self,
        param: Option<T>,
        make_sections: impl FnOnce(SqlSetClause<'sql>, T) -> SqlSetClause<'sql>,
    ) -> Self {
        if let Some(param) = param {
            match &self.0.last() {
                Some(QueryFragment::Sql(" WHERE ") | QueryFragment::Separator) => (),
                _ => {
                    self.0.push(QueryFragment::Separator);
                }
            }

            make_sections(SqlSetClause::new_fragment(), param)
                .0
                .into_iter()
                .for_each(|section| self.0.push(section));
        };

        self
    }

    pub fn into_query_builder(self, init: &'sql str) -> QueryBuilder<'sql, Postgres> {
        let mut qb = QueryBuilder::new(init);
        let mut param_count = 0;

        self.0.into_iter().for_each(|section| match section {
            QueryFragment::Sql(sql) => {
                qb.push(sql);
            }
            QueryFragment::Param(param) => {
                param_count += 1;

                query_param_push_bind(&mut qb, param);
            }
            QueryFragment::PreviousParam => {
                assert!(
                    !(param_count == 0),
                    "`QueryFragment::PreviousParam` cannot be pushed before a \
                     `QueryFragment::Param` is pushed",
                );

                qb.push(format!("${param_count}"));
            }
            QueryFragment::Separator => {
                qb.push(", ");
            }
        });

        qb
    }
}
