This is a fun little POC for a Rust service layer. Click on my cute friend to make sure you have the latest version of Rust and Cargo.

[![cute rustacean](https://rustacean.net/assets/corro.svg)](https://www.rust-lang.org/)

Next Click on the sweet Diesel logo to snag the diesel CLI and get up to speed.

[![diesel logo](https://diesel.rs/assets/images/diesel_logo_stacked_black.png)](https://diesel.rs)

I use a Linux machine so feel free to tweak these steps but the way I run this POC is by doing the following.

First I start docker.

`sudo systemctl start docker`

I make sure my docker daemon is running

`sudo docker ps -a`

Next I build the docker container

`sudo docker build -t postgres-db`

Then I start the docker container

`sudo docker -d postgres-db`

Go ahead and use diesel to run the migration which will add the greeting to the db.

`diesel migration run`

The table stucture is the exact same as the getting started example.

Now go ahead and run the application and see the greeting.

`cargo run`
