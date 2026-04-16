use sea_orm::{ConnectionTrait, DatabaseTransaction, TransactionTrait};

/// Basically a `DbConn`
pub trait ServiceConnection:
    ConnectionTrait + TransactionTrait<Transaction = DatabaseTransaction>
{
}

impl<C> ServiceConnection for C where
    C: ConnectionTrait + TransactionTrait<Transaction = DatabaseTransaction>
{
}
