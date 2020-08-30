CREATE TABLE IF NOT EXISTS "articles" (
  "id" SERIAL,
  "slug" VARCHAR(255) NOT NULL,
  "title" VARCHAR(255) NOT NULL,
  "description" TEXT NOT NULL,
  "body" TEXT NOT NULL,
  "author_id" SERIAL NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  PRIMARY KEY ("id"),
  CONSTRAINT "unique_slug" UNIQUE ("slug"),
  CONSTRAINT "fk_author_id" FOREIGN KEY ("author_id") REFERENCES "users"("id") ON DELETE CASCADE
);
