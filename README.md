# Beaxplorer

Beaxplorer is an Ethereum 2.0 Beacon chain explorer written in Rust and
relying on [Lighthouse](https://lighthouse.sigmaprime.io/).

## Getting started

### Prerequisites

- Install PostgreSQL

- Install [Lighthouse](https://lighthouse.sigmaprime.io/) Ethereum 2.0 client
  and have it synced

- Install the [Diesel](https://diesel.rs/) CLI with the following command:
  `cargo install diesel_cli`

- Create a file named `.env` containing the database url and the Lighthouse
  endpoint url (see [.env.exexample](./.env.example))

- Init the database:

  ```
    cd db
    diesel migration run
  ```

- Build the web frontend:
  ```
    cd web/frontend
    yarn
    yarn run build
  ```

### Indexing

Run:

```
  cd indexer_cli
  cargo run
```

You can terminate the indexing process with ctrl-c.

### Start the web server

Run:

```
  cd web
  cargo run
```
