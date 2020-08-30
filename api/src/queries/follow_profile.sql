INSERT INTO users_followers (follower_id, leader_id) 
SELECT $1, u.id FROM users u WHERE u.username = $2
