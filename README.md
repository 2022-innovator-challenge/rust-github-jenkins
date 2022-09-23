# rust-github-scraper

![image](https://user-images.githubusercontent.com/22489773/192068435-513aea77-fdf1-4639-8f34-85c0ff464fb8.png)

Choice of technology:
* _Cargo_ - Dependency management
* _Rocket_ - Web service provisioning
* _Diesel_ - SQL ORM Client

Furthermore:
* _Postgres_ - (independent) Database
* _Docker_ - Compilation and execution


Useful commands / flags:


<details>
 <summary>(1) Setup</summary>
 
</details>
<details>
 <summary>(2) Running and building</summary>
 
 ```
 // install required cargo dependency
 cargo install diesel_cli --no-default-features --features postgres
 ```
 
 ```
 // setup database connection
 echo DATABASE_URL=postgres://postgres:rustaman@localhost/postgres > .env
 ```
 
 ```
 diesel setup
 diesel migration generate store_builds
 ```
</details>
 
<details>
 <summary>(2.1) Running and building when containerized</summary>

 ## Build

 ```
 // For web project, set nightly pipeline
 rustup override set nightly
 ```
 
 ```
 // build without registry
 docker run --rm -v ${pwd}/:/my rust cargo build --release --path /my/ --target-dir /my/target
 ```

 ```
 // build with registry
 docker run --rm -v ${pwd}/:/my -v ${pwd}/.registry:/usr/local/cargo/registry/ rust cargo build --release --path /my/ --target-dir /my/target
 ```

 ## Run locally

 ```
 // run without registry
 docker run -it --rm -v ${pwd}/:/my -w "/my" rust cargo run .
 ```

 ```
 // run with registry
 docker run -it --rm -v ${pwd}/:/my -v ${pwd}/.registry:/usr/local/cargo/registry/ -w "/my" rust bash
 cargo run
 ```

 ## Run remotely

 ```
 // run compiled application with debian
 docker run -it --rm -v ${pwd}/target/release:/my -w "/my" debian:bookworm-slim ./scrape
 ```

 ```
 // run compiled application with debian (fix certificate issue)
 docker run -it --rm -v ${pwd}/target/:/my -w "/my" debian bash
 apt update && apt install ca-certificates -y
 ./release/scrape
 ```

</details>





