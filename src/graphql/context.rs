use crate::db::Conn;

pub struct Context {
    pub conn: Conn,
}

impl juniper::Context for Context {}

impl AsRef<Self> for Context {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
