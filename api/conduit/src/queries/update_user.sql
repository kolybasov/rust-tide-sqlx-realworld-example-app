UPDATE users u
SET email = COALESCE($2, u.email),
    username = COALESCE($3, u.username),
    password = COALESCE($4, u.password),
    image = $5,
    bio = $6
WHERE id = $1
RETURNING *
