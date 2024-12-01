# TTB

Brief description of your project.

## Prerequisites

Before you begin, ensure you have met the following requirements:

* You have installed the latest version of [Rust](https://www.rust-lang.org/tools/install)
* You have a Linux/Mac OS/Windows machine.
* You have installed [PostgreSQL](https://www.postgresql.org/download/) (or your preferred database supported by Diesel)

## Installing TTB

To install TTB, follow these steps:

1. Clone the repository: `https://github.com/tolu-afo/TTB.git`
2. Install the Diesel CLI: `https://diesel.rs/guides/getting-started`
3. Set Up the database: echo DATABASE_URL=postgres://username:password@localhost/database_name > .env
5. Run `diesel setup`
6. Run the migrations: `diesel migration run`


## Using TTB

To use Project Name, follow these steps:

1. In the .env add the following environment variables:
2. `BROADCASTER_ID`: The twitch id of the broadcaster/streamer whose chat you want the bot to connect to
3. `TWITCH_CLIENT_SECRET`: The client secret for your twitch app account
4. `TWITCH_CLIENT_ID`: The client id for your twitch app account
5. `BOT_OAUTH_TOKEN`: The oauth token for your bot account
6. `BOT_USERNAME`: The username for your bot account
7. Run `cargo run`

## Contributing to TTB

To contribute to TTB, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`
4. Push to the original branch: `git push origin <project_name>/<location>`
5. Create the pull request.

Alternatively, see the GitHub documentation on [creating a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

## Contributors

Thanks to the following people who have contributed to this project:

* [@nickk-dv](https://github.com/nickk-dv)
* [@rx80](https://github.com/rx80)

## Contact

If you want to contact me, you can reach me at <tolu@asokpo.com>.

## License

This project uses the following license: [MIT]([<link>](https://opensource.org/license/mit)) / [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0).
