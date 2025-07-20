pub const SELECT_POSTS: &str = "select * from posts";
pub const INSERT_POST: &str =
    "insert into posts (title, body, published) values ($1, $2, $3) returning *";
pub const UPDATE_POST: &str =
    "update posts set title = $1, body = $2, published = $3 where id = $4 returning *";
pub const DELETE_POST: &str = "delete from posts where id = $1 returning *";
