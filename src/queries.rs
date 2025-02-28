pub const SELECT_POST_BY_ID: &str = "select * from posts where id = $1";
pub const INSERT_POST: &str =
    "insert into posts (title, body, published) values ($1, $2, $3) returning *";
