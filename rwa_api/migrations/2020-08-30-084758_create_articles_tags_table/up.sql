CREATE TABLE IF NOT EXISTS "articles_tags" (
  "article_id" SERIAL NOT NULL,
  "tag_id" VARCHAR(50) NOT NULL,

  PRIMARY KEY ("article_id", "tag_id"),
  CONSTRAINT "fk_article_id"
    FOREIGN KEY ("article_id") REFERENCES "articles"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_tag_id"
    FOREIGN KEY ("tag_id") REFERENCES "tags"("tag") ON DELETE CASCADE
);
