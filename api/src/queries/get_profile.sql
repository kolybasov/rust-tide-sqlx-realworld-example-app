SELECT username, bio, image, (uf.leader_id IS NOT NULL) "following!"  FROM users u
LEFT JOIN users_followers uf ON uf.leader_id = u.id AND uf.follower_id = $2
WHERE u.username = $1
