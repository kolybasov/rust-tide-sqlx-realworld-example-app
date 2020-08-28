DELETE FROM articles_favorites af
USING articles a
WHERE a.slug = $1 AND af.user_id = $2
