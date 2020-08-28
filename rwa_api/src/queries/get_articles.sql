SELECT a.*,
       ARRAY_REMOVE(ARRAY_AGG(at.tag_id), NULL) "tag_list!",
       COUNT(DISTINCT af.article_id) "favorites_count!",
       BOOL_OR(af.user_id = $1) "favorited!",
       u.username "author_username",
       u.bio "author_bio",
       u.image "author_image",
       BOOL_OR(uf.follower_id IS NOT NULL) "author_following!"
FROM articles a
-- Join tags
LEFT JOIN articles_tags at ON at.article_id = a.id
-- Join favorites
LEFT JOIN articles_favorites af
    -- Join user that favorited article
    INNER JOIN users u2 ON u2.id = af.user_id
    ON af.article_id = a.id
-- Join author
INNER JOIN users u
    -- Join author's followers
    LEFT JOIN users_followers uf ON uf.leader_id = u.id AND uf.follower_id = $1
    ON u.id = a.author_id
WHERE ($2::VARCHAR IS NULL OR a.slug = $2::VARCHAR) AND
      ($3::VARCHAR IS NULL OR at.tag_id = $3::VARCHAR) AND
      ($4::VARCHAR IS NULL OR u.username = $4::VARCHAR) AND
      ($5::VARCHAR IS NULL OR u2.username = $5::VARCHAR)
GROUP BY a.id, u.username, u.bio, u.image
ORDER BY a.id DESC
LIMIT $6
OFFSET $7
