--this is the same table found in the diesel getting started example
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
);

--we add this line for the demo
insert into posts (title, body, published) values ('greeting', 'hello world', true)