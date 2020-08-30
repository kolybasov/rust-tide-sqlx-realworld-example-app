CREATE TABLE IF NOT EXISTS "comments" (
  "id" SERIAL,
  "body" TEXT NOT NULL,
  "author_id" SERIAL NOT NULL,
  "article_id" SERIAL NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  PRIMARY KEY ("id"),
  CONSTRAINT "fk_author_id"
    FOREIGN KEY ("author_id") REFERENCES "users"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_article_id"
    FOREIGN KEY ("article_id") REFERENCES "articles"("id") ON DELETE CASCADE
);
