DELETE FROM comments 
WHERE id = $1 AND author_id = $2
