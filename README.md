This is a fun little POC for a Rust service layer. Click on my cute friend to make sure you have the latest version of Rust and Cargo.

[![cute rustacean](https://rustacean.net/assets/corro.svg)](https://www.rust-lang.org/)

Next Click on the sweet Diesel logo to snag the diesel CLI and get up to speed.

[![diesel logo](https://diesel.rs/assets/images/diesel_logo_stacked_black.png)](https://diesel.rs)

I use a Linux machine so feel free to tweak these steps but the way I run this POC is by doing the following.

If you don't have docker on your computer now would be a great time to get it. 

https://www.docker.com/

make sure you have docker on your machine.

`docker -v`

cd to your app directory and open the terminal in your IDE

start docker.

`sudo systemctl start docker`

check and see if the docker daemon is running

`sudo docker ps -a`

build and start the postgres db docker container

`sudo docker run --name postgres-db -p 5432:5432 -e POSTGRES_PASSWORD=changeit -d postgres-db`

use diesel to run the migration which will add the greeting to the first row of the db.

`diesel migration run`

The table stucture is the exact same as the Diesel getting started example.

run the application, and see the greeting.

`cargo run`

don't forget to set the path for the diesel cli in the diesel.toml file. I sometimes forget to do this and wonder for a second why my diesel migration commands don't work.
