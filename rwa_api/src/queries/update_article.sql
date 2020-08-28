UPDATE articles a
SET slug = COALESCE($1, a.slug),
    title = COALESCE($2, a.title),
    description = COALESCE($3, a.description),
    body = COALESCE($4, a.body)
WHERE slug = $5 AND author_id = $6
