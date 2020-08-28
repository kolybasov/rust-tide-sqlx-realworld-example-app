INSERT INTO articles_favorites (article_id, user_id)
SELECT a.id, $2 FROM articles a WHERE a.slug = $1
