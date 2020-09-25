INSERT INTO articles (slug, title, description, body, author_id)
VALUES ($1, $2, $3, $4, $5)
RETURNING id
