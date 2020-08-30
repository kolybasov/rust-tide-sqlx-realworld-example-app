CREATE TABLE IF NOT EXISTS "users_followers" (
  "follower_id" SERIAL NOT NULL,
  "leader_id" SERIAL NOT NULL,

  PRIMARY KEY ("follower_id", "leader_id"),
  CONSTRAINT "fk_follower_id"
    FOREIGN KEY ("follower_id") REFERENCES "users"("id") ON DELETE CASCADE,
  CONSTRAINT "fk_leader_id"
    FOREIGN KEY ("leader_id") REFERENCES "users"("id") ON DELETE CASCADE
);
