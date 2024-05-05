<div align=center>
    <h3 align=center> MySK API </h3>
    <p>The API Interface for the MySK database</p>
    <p align='center'>
        <a href="https://mysk.school"><b>mysk.school</b></a> •
        <a href="http://api.mysk.school"><b>api.mysk.school</b></a>
    </p>
</div>

<br />

## 📦 Setup

1) To get set up, clone the repository and ensure that these tools & dependencies are installed on your system
    - [rust](https://rustup.rs/)
    - [npm](https://www.npmjs.com/)
    - [mprocs](https://github.com/pvolok/mprocs) (see [using-mprocs](#🛠️-using-mprocs))
    - git🙄

2) Run the following command in `/mysk-api-test-web-server`

```sh
# install dependencies for the API client server
$ npm i
```

### :herb: Environment

| File                                                                                  | Description                   |
| ------------------------------------------------------------------------------------- | ----------------------------- |
| [`.env`](.env.template)                                                               | Global configuration file     |
| [`mysk-api-test-web-server/.env.local`](mysk-api-test-web-server/.env.local.template) | Web server configuration file |

> [!CAUTION]
> Do not commit `.env` and `.env.local` files to the repository. These files contain sensitive information and should be kept private.

<br />

## 🚀 Development

### 🛠️ Using mprocs

If [mprocs](https://github.com/pvolok/mprocs) is installed, run the following command:

```sh
# This command will look for an mprocs.yaml configuration and start necessary services automatically
$ mprocs --config ./mprocs.yaml
```

### ⚙️  Manually

To start services manually run the following commands:

```sh
# Build and run cargo workspace at root
$ cargo run

# Start API testing client server at /mysk-api-test-web-server
$ npm run dev
```

<br />

### 📁 Basic structure

This repository contains libraries and tools needed to get set up for developing on MySK's API. The basic structure of the monorepo are as follows:

```
.
├── mysk-api-test-web-server                                            // testing client
├── mysk-data-api/                                                      // API
│   └── src/
│       ├── extractors/                                                 // extractor funtions
│       └── routes/                                                     // route definitions
│           └── v1
├── mysk-lib-derives/                                                   // derived traits
├── mysk-lib-macros/                                                    // macros
├── mysk-lib/                                                           // libraries
└── Cargo.toml                                                          // cargo workspace
```
