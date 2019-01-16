# rust-restful-api

RESTful API Server Example

## Endpoint `/posts`  

#### GET `/posts` 

Returns a list of all messages

`curl http://localhost:8000/posts`

#### POST `/posts`

Creates a new post

**Data Params:**

- title - String

- description - String

```
curl -H "Content-Type: application/json" \
>   --data '{"title": "Hello", "description": "world"}' \
>   -X POST \
>   http://localhost:8000/posts
```

#### PUT `/posts/<id>`

Updates the post by id

**Url param**: 

- id - int

**Data Params:**

- title - String

- description - String

```
curl -H "Content-Type: application/json" \
>   --data '{"title": "Hello", "description": "world"}' \
>   -X PUT \
>   http://localhost:8000/posts/1
```

#### DELETE `/posts/<id>`

Deletes the post by id

**Url param**: 

- id - int

`curl -X DELETE http://localhost:8000/posts/1`


## Deploy

Build app
```
docker run --rm \
    -v cargo-cache:/root/.cargo \
    -v $$PWD:/volume \
    -w /volume \
    -it clux/muslrust:nightly \
    cargo build --release
```
move to current directory

`mv target/x86_64-unknown-linux-musl/release/rust_restful_api .`

and run `docker-compose up -d`
