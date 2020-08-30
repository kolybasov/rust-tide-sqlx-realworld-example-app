DELETE FROM users_followers uf
USING users u
WHERE u.username = $1 AND
      uf.follower_id = $2 AND
      uf.leader_id = u.id
