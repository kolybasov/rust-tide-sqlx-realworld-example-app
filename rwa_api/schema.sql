CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "users" (
  "id" UUID DEFAULT uuid_generate_v4(),
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
  "id" UUID DEFAULT uuid_generate_v4(),
  "slug" VARCHAR(255) NOT NULL,
  "title" VARCHAR(255) NOT NULL,
  "description" TEXT,
  "body" TEXT,
  "author_id" UUID NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  PRIMARY KEY ("id"),
  CONSTRAINT "unique_slug" UNIQUE ("slug"),
  CONSTRAINT "fk_author_id" FOREIGN KEY ("author_id") REFERENCES "users"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "comments" (
  "id" UUID DEFAULT uuid_generate_v4(),
  "body" TEXT NOT NULL,
  "author_id" UUID NOT NULL,
  "article_id" UUID NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  PRIMARY KEY ("id"),
  CONSTRAINT "fk_author_id"
    FOREIGN KEY ("author_id") REFERENCES "users"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_article_id"
    FOREIGN KEY ("article_id") REFERENCES "articles"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "tags" (
  "id" UUID,
  "tag" VARCHAR(50) NOT NULL UNIQUE,
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "articles_tags" (
  "article_id" UUID NOT NULL,
  "tag_id" UUID NOT NULL,

  PRIMARY KEY ("article_id", "tag_id"),
  CONSTRAINT "fk_article_id"
    FOREIGN KEY ("article_id") REFERENCES "articles"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_tag_id"
    FOREIGN KEY ("tag_id") REFERENCES "tags"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "articles_favorites" (
  "article_id" UUID NOT NULL,
  "user_id" UUID NOT NULL,

  PRIMARY KEY ("article_id", "user_id"),
  CONSTRAINT "fk_article_id"
    FOREIGN KEY ("article_id") REFERENCES "articles"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_user_id"
    FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "users_followers" (
  "follower_id" UUID NOT NULL,
  "following_id" UUID NOT NULL,

  PRIMARY KEY ("follower_id", "following_id"),
  CONSTRAINT "fk_follower_id"
    FOREIGN KEY ("follower_id") REFERENCES "users"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_following_id"
    FOREIGN KEY ("following_id") REFERENCES "users"("id") ON DELETE CASCADE
);
