use expression::{Expression, SelectableExpression};
use query_builder::{QueryBuilder, BuildQueryResult};
use types::{SqlOrd, NativeSqlType};

macro_rules! ord_function {
    ($fn_name:ident, $type_name:ident, $operator:expr, $docs:expr) => {
        #[doc=$docs]
        pub fn $fn_name<ST, T>(t: T) -> $type_name<T> where
            ST: NativeSqlType + SqlOrd,
            T: Expression<SqlType=ST>,
        {
            $type_name {
                target: t,
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct $type_name<T: Expression> {
            target: T,
        }

        impl<T: Expression> Expression for $type_name<T> {
            type SqlType = T::SqlType;

            fn to_sql(&self, out: &mut QueryBuilder) -> BuildQueryResult {
                out.push_sql(concat!($operator, "("));
                try!(self.target.to_sql(out));
                out.push_sql(")");
                Ok(())
            }
        }

        impl<T: Expression, QS> SelectableExpression<QS> for $type_name<T> {
        }
    }
}

ord_function!(max, Max, "MAX",
"Represents a SQL `MAX` function. This function can only take types which are
ordered.");

ord_function!(min, Min, "MIN",
"Represents a SQL `MIN` function. This function can only take types which are
ordered.");