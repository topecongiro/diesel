#[doc(hidden)]
pub mod filter;
mod joins;

use expression::{Expression, SelectableExpression, NonAggregate};
use query_builder::*;
#[doc(hidden)]
pub use self::joins::{InnerJoinSource, LeftOuterJoinSource};
use types::{FromSqlRow, NativeSqlType};

pub use self::joins::JoinTo;

pub trait Queriable<ST: NativeSqlType> {
    type Row: FromSqlRow<ST>;

    fn build(row: Self::Row) -> Self;
}

pub trait QuerySource: Sized {
    fn from_clause(&self, out: &mut QueryBuilder) -> BuildQueryResult;
}

pub trait Column: Expression {
    type Table: Table;

    fn name() -> &'static str;
}

pub trait Table: QuerySource + AsQuery + Sized {
    type PrimaryKey: Column<Table=Self> + Expression + NonAggregate;
    type AllColumns: SelectableExpression<Self>;

    fn name() -> &'static str;
    fn primary_key(&self) -> Self::PrimaryKey;
    fn all_columns() -> Self::AllColumns;

    fn inner_join<T>(self, other: T) -> InnerJoinSource<Self, T> where
        T: Table,
        Self: JoinTo<T>,
    {
        InnerJoinSource::new(self, other)
    }

    fn left_outer_join<T>(self, other: T) -> LeftOuterJoinSource<Self, T> where
        T: Table,
        Self: JoinTo<T>,
    {
        LeftOuterJoinSource::new(self, other)
    }
}

impl<T: Table> UpdateTarget for T {
    type Table = Self;

    fn where_clause(&self, _out: &mut QueryBuilder) -> BuildQueryResult {
        Ok(())
    }

    fn table(&self) -> &Self::Table {
        self
    }
}