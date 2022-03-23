use async_graphql::{SimpleObject, ID};

// TODO
#[derive(SimpleObject)]
pub struct Book {
    id: String,
    title: String,
}

// TODO
impl Book {
    pub fn new(id: String, title: String) -> Self {
        Book { id, title }
    }
}

#[derive(SimpleObject)]
pub struct Author {
    pub id: ID,
    pub name: String,
}

impl Author {
    pub fn new(id: String, name: String) -> Self {
        Self { id: ID(id), name }
    }
}
