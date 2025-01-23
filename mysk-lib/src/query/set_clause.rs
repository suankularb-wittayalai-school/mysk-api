use crate::{
    common::{requests::QueryParam, string::FlexibleMultiLangString},
    query::{fragment::QueryParamType, QueryFragment},
};
use sqlx::{Postgres, QueryBuilder};

/// A collection of multiple `QueryFragment`s.
#[derive(Debug)]
pub struct SqlSetClause<'sql>(Vec<QueryFragment<'sql>>);

impl<'sql> SqlSetClause<'sql> {
    pub fn new() -> Self {
        SqlSetClause(vec![QueryFragment::Sql(" SET ")])
    }

    pub fn push_sql(&mut self, sql: &'sql str) -> &mut Self {
        self.0.push(QueryFragment::Sql(sql));

        self
    }

    pub fn push_param(&mut self, param: QueryParam) -> &mut Self {
        self.0.push(QueryFragment::Param(param));

        self
    }

    pub fn push_prev_param(&mut self) -> &mut Self {
        self.0.push(QueryFragment::PreviousParam);

        self
    }

    pub fn push_sep(&mut self) -> &mut Self {
        if self.0.last().unwrap() != &QueryFragment::Sql(" SET ") {
            self.0.push(QueryFragment::Separator);
        }

        self
    }

    pub fn push_multilang_update_field(
        &mut self,
        column: &'sql str,
        param: Option<FlexibleMultiLangString>,
    ) -> &mut Self {
        if let Some(param) = param {
            if let Some(th) = param.th {
                self.push_sep()
                    .push_sql(column)
                    .push_sql("_th = COALESCE(")
                    .push_param(QueryParam::String(th))
                    .push_sep()
                    .push_sql(column)
                    .push_sql("_th)");
            }

            if let Some(en) = param.en {
                self.push_sep()
                    .push_sql(column)
                    .push_sql("_en = COALESCE(")
                    .push_param(QueryParam::String(en))
                    .push_sep()
                    .push_sql(column)
                    .push_sql("_en)");
            }
        }

        self
    }

    pub fn push_update_field<T: QueryParamType>(
        &mut self,
        column: &'sql str,
        param: Option<T>,
        make_variant: impl FnOnce(T) -> QueryParam,
    ) -> &mut Self {
        if let Some(param) = param {
            self.push_sep()
                .push_sql(column)
                .push_sql(" = COALESCE(")
                .push_param(make_variant(param))
                .push_sep()
                .push_sql(column)
                .push_sql(")");
        }

        self
    }

    pub fn push_if_some<T, F>(&mut self, param: Option<T>, make_sections: F) -> &mut Self
    where
        T: QueryParamType,
        F: FnOnce(SqlSetClause<'sql>, T) -> SqlSetClause<'sql>,
    {
        if let Some(param) = param {
            match self.0.last() {
                Some(QueryFragment::Sql(" SET ") | QueryFragment::Separator) => (),
                _ => {
                    self.0.push(QueryFragment::Separator);
                }
            }

            make_sections(SqlSetClause(Vec::new()), param)
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

                param.push_bind(&mut qb);
            }
            QueryFragment::PreviousParam => {
                assert!(
                    param_count != 0,
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
