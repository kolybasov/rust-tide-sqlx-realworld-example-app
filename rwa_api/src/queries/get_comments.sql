SELECT c.*,
       u.username "author_username!",
       u.bio "author_bio",
       u.image "author_image",
       (uf.follower_id IS NOT NULL) "author_following!"
FROM comments c
INNER JOIN articles a ON a.id = c.article_id
INNER JOIN users u
    LEFT JOIN users_followers uf ON uf.leader_id = u.id AND uf.follower_id = $3
    ON u.id = c.author_id
WHERE ($1::INTEGER IS NULL OR c.id = $1) AND
      ($2::VARCHAR IS NULL OR a.slug = $2)
