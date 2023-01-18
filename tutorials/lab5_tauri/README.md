# Task scheduler

Simple "todo" app made with tauri. Registers users and creates tasks for them.  
Tasks are stored in PostgreSQL db via [sqlx](https://github.com/launchbadge/sqlx).

<details> <summary>Demo</summary>

![Demo](demo/demo.png)</details>

## Setup

0. Open `src-tauri` directory:

```sh
$ cd src-tauri
```

1. Declare postgres db url:

```sh
$ export DATABASE_URL="postgres://postgres:password@localhost/todos"
```

2. Create db

```sh
$ sqlx db create
```

3. Run migrations

```sh
$ sqlx migrate run
```

4. Launch\build

```sh
# launch
$ cargo tauri dev
# build
$ cargo tauri build
```
