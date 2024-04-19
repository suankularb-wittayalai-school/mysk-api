<div align=center>
    <h3 align=center> MySK API </h3>
    <p>The API Interface for the MySK database</p>
    <p align='center'>
        <a href="https://mysk.school"><b>mysk.school</b></a> •
        <a href="http://api.mysk.school"><b>api.mysk.school</b></a>
    </p>
</div>

<br />

## :zap: Setup
After cloning this repository, run the following command at `/mysk-api-test-web-server`
```sh
# install dependencies for the API client server
npm i
```
### :herb: Environment
This project uses two environment files for configuration
| File                                                                                                                             | Description                   |
| -------------------------------------------------------------------------------------------------------------------------------- | ----------------------------- |
| [`.env`](.env.template)                                      | Global configuration file     |
| [`.env.local`](mysk-api-test-web-server/.env.local.template) | Web server configuration file |

> [!CAUTION]
> Do not commit `.env` and `.env.local` files to the repository. These files contain sensitive information and should be kept private.

<br />

## :gear: Development
To get started with developing on MySK API, run the following commands:
```sh
# Build and run cargo workspace at root
cargo run

# Start API testing client server at /mysk-api-test-web-server
npm run dev
```


### :book: Basic structure
This repository contains libraries and tools needed to get set up for developing on MySK's API. The basic structure of the monorepo are as follows:
```
.
├── mysk-api-test-web-server                                            // API testing client
├── mysk-data-api/                                                      // API
├── mysk-lib-derives/                                                   // derived traits
├── mysk-lib-macros/                                                    // macros
├── mysk-lib/                                                           // libraries
└── Cargo.toml                                                          // cargo workspace definition
```

