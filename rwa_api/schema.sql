CREATE TABLE IF NOT EXISTS "users" (
  "id" SERIAL,
  "username" VARCHAR(50) NOT NULL,
  "email" VARCHAR(255) NOT NULL,
  "password" VARCHAR(255) NOT NULL,
  "bio" TEXT,
  "image" VARCHAR(255),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  PRIMARY KEY ("id"),
  CONSTRAINT "unique_username" UNIQUE ("username"),
  CONSTRAINT "unique_email" UNIQUE ("email")
);

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

CREATE TABLE IF NOT EXISTS "tags" (
  "tag" VARCHAR(50) NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  PRIMARY KEY ("tag")
);

CREATE TABLE IF NOT EXISTS "articles_tags" (
  "article_id" SERIAL NOT NULL,
  "tag_id" VARCHAR(50) NOT NULL,

  PRIMARY KEY ("article_id", "tag_id"),
  CONSTRAINT "fk_article_id"
    FOREIGN KEY ("article_id") REFERENCES "articles"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_tag_id"
    FOREIGN KEY ("tag_id") REFERENCES "tags"("tag") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "articles_favorites" (
  "article_id" SERIAL NOT NULL,
  "user_id" SERIAL NOT NULL,

  PRIMARY KEY ("article_id", "user_id"),
  CONSTRAINT "fk_article_id"
    FOREIGN KEY ("article_id") REFERENCES "articles"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_user_id"
    FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "users_followers" (
  "follower_id" SERIAL NOT NULL,
  "following_id" SERIAL NOT NULL,

  PRIMARY KEY ("follower_id", "following_id"),
  CONSTRAINT "fk_follower_id"
    FOREIGN KEY ("follower_id") REFERENCES "users"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_following_id"
    FOREIGN KEY ("following_id") REFERENCES "users"("id") ON DELETE CASCADE
);
