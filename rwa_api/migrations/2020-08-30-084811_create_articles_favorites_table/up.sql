CREATE TABLE IF NOT EXISTS "articles_favorites" (
  "article_id" SERIAL NOT NULL,
  "user_id" SERIAL NOT NULL,

  PRIMARY KEY ("article_id", "user_id"),
  CONSTRAINT "fk_article_id"
    FOREIGN KEY ("article_id") REFERENCES "articles"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_user_id"
    FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);
