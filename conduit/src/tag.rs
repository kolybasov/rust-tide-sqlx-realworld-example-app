use crate::error::Result;
use crate::query::generate_mass_insert_placeholder;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use slug::slugify;
use sqlx::{query, query_file_as, Executor, Postgres};

pub struct TagService<E: Executor<'static, Database = Postgres> + Copy> {
    db: E,
}

impl<E> TagService<E>
where
    E: Executor<'static, Database = Postgres> + Copy,
{
    pub fn new(executor: E) -> Self {
        TagService { db: executor }
    }

    pub async fn get_tags(&self) -> Result<Vec<String>> {
        let tags = query_file_as!(Tag, "./src/queries/get_tags.sql")
            .fetch_all(self.db)
            .await?;
        Ok(tags.into_iter().map(|raw_tag| raw_tag.tag).collect())
    }

    pub async fn create_tags(&self, tags: &[String]) -> Result<Vec<String>> {
        let tag_list: Vec<String> = tags.iter().map(slugify).collect();

        let query_str = format!(
            "INSERT INTO tags (tag) VALUES {} ON CONFLICT (tag) DO NOTHING",
            generate_mass_insert_placeholder(&tag_list, 1)
        );

        tag_list
            .iter()
            .fold(query(&query_str), |query, tag| query.bind(tag))
            .execute(self.db)
            .await?;

        Ok(tag_list)
    }

    pub async fn assign_tags_to_article(&self, tags: &[String], article_id: i32) -> Result<()> {
        let query_str = format!(
            "INSERT INTO articles_tags (tag_id, article_id) VALUES {}",
            generate_mass_insert_placeholder(tags, 2)
        );

        tags.iter()
            .fold(query(&query_str), |query, tag| {
                query.bind(tag).bind(article_id)
            })
            .execute(self.db)
            .await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub tag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
