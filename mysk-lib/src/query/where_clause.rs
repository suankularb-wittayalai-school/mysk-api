use crate::query::{building_blocks::QueryParamType, QueryFragment, QueryParam};
use sqlx::{Postgres, QueryBuilder};

/// A collection of multiple `QueryFragment`s.
#[derive(Debug)]
pub struct SqlWhereClause<'sql>(Vec<QueryFragment<'sql>>);

impl<'sql> SqlWhereClause<'sql> {
    /// Creates a new `SqlWhereClause` that starts with a SQL WHERE clause (" WHERE ").
    pub fn new() -> Self {
        SqlWhereClause(vec![QueryFragment::Sql(" WHERE ")])
    }

    /// Creates a new `SqlWhereClause` that starts empty.
    pub fn new_empty() -> Self {
        SqlWhereClause(Vec::new())
    }

    /// Pushes a SQL fragment into self.
    pub fn push_sql(&mut self, sql: &'sql str) -> &mut Self {
        self.0.push(QueryFragment::Sql(sql));

        self
    }

    /// Pushes a SQL query parameter into self.
    pub fn push_param(&mut self, param: QueryParam) -> &mut Self {
        self.0.push(QueryFragment::Param(param));

        self
    }

    /// Pushes an indicator into self that tells it to use the previously pushed SQL query parameter
    /// as a bind argument when calling the functions `into_query_builder` or `into_sql`. \
    /// **Warning:** If no parameter precedes the call to this function, a panic will occur when
    /// calling `into_query_builder` or `into_sql`.
    pub fn push_prev_param(&mut self) -> &mut Self {
        self.0.push(QueryFragment::PreviousParam);

        self
    }

    /// Pushes a SQL AND clause (" AND ") into self.
    pub fn push_sep(&mut self) -> &mut Self {
        self.0.push(QueryFragment::Separator);

        self
    }

    /// Executes the given closure and pushes the `SqlWhereClause` returned by the closure into self
    /// if the optional SQL query parameter given returns `true` on a `.is_some()` predicate.
    /// Additionally, automatically push a separator when required.
    pub fn push_if_some<T, F>(&mut self, param: Option<T>, make_sections: F) -> &mut Self
    where
        T: QueryParamType,
        F: FnOnce(SqlWhereClause<'sql>, T) -> SqlWhereClause<'sql>,
    {
        if let Some(param) = param {
            match self.0.last() {
                Some(QueryFragment::Sql(" WHERE ") | QueryFragment::Separator) => (),
                _ => {
                    self.0.push(QueryFragment::Separator);
                }
            }

            make_sections(SqlWhereClause(Vec::new()), param)
                .0
                .into_iter()
                .for_each(|section| self.0.push(section));
        };

        self
    }

    /// Append elements in self into a `sqlx::QueryBuilder` that is ready to be built. \
    /// **Warning:** If a `QueryFragment::PreviousParam` was encountered before any
    /// `QueryFragment::Param`s, a panic will occur.
    pub fn append_into_query_builder(self, qb: &mut QueryBuilder<'sql, Postgres>) {
        let mut param_count = 0;

        self.0.into_iter().for_each(|section| match section {
            QueryFragment::Sql(sql) => {
                qb.push(sql);
            }
            QueryFragment::Param(param) => {
                param_count += 1;

                param.push_bind(qb);
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
                qb.push(" AND ");
            }
        });
    }

    /// Transforms self into an owned SQL string.
    /// **Warning:** If a `QueryFragment::PreviousParam` was encountered before any
    /// `QueryFragment::Param`s, a panic will occur.
    pub fn into_sql(self) -> String {
        let mut param_count = 0;

        self.0
            .into_iter()
            .map(|section| match section {
                QueryFragment::Sql(sql) => sql.to_string(),
                QueryFragment::Param(_) => {
                    param_count += 1;

                    format!("${param_count}")
                }
                QueryFragment::PreviousParam => {
                    assert!(
                        param_count != 0,
                        "`QueryFragment::PreviousParam` cannot be pushed before a \
                         `QueryFragment::Param` is pushed",
                    );

                    format!("${param_count}")
                }
                QueryFragment::Separator => " AND ".to_string(),
            })
            .collect()
    }
}
