INSERT INTO comments (body, article_id, author_id) 
SELECT $2, a.id, $3 FROM articles a WHERE a.slug = $1
RETURNING id
