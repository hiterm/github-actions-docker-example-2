use async_graphql::{Context, Object};

use crate::{extractors::Claims, presentational::error::error::PresentationalError};

use super::{
    object::{Author, Book},
    query_service::QueryService,
};

pub struct Query<QS> {
    query_service: QS,
}

impl<QS> Query<QS> {
    pub fn new(query_service: QS) -> Self {
        Query { query_service }
    }
}

#[Object]
impl<QS> Query<QS>
where
    QS: QueryService,
{
    async fn book(&self, id: String) -> Result<Book, PresentationalError> {
        let book = self.query_service.find_book_by_id(&id).await?;
        Ok(Book::new(book.id, book.title))
    }

    async fn author(
        &self,
        ctx: &Context<'_>,
        author_id: String,
    ) -> Result<Author, PresentationalError> {
        let claims = ctx
            .data::<Claims>()
            .map_err(|err| PresentationalError::OtherError(anyhow::anyhow!(err.message)))?;
        let author = self
            .query_service
            .find_author_by_id(&claims.sub, &author_id)
            .await?;
        Ok(Author::new(author.id, author.name))
    }
}
