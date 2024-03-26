# cc-curl

Curl clone created for [codingchallenges.fyi](codingchallenges.fyi)'s curl [challenge](codingchallenges.fyi/challenges/challenge-curl).
Supports `GET`, `POST`, `PUT`, and `DELETE` over http. Can send arbitrairy http methods using the `-X` option. Supports `-d` and `-H` options for adding headers and data. `-v` for verbose output.

### Building
cc-crul be built and run using [Cargo](https://github.com/rust-lang/cargo). To pass options to the program (not cargo), add a `--` after all of cargo's options.  
example:
```bash
cargo run -- -v https://github.com/rust-lang/cargo
```

### Examples

###### GET
```bash
% cc-curl http://eu.httpbin.org/get
{
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "eu.httpbin.org",
  },
  "url": "http://eu.httpbin.org/get"
}
```

###### DELETE
```bash
% cc-curl -X DELETE http://eu.httpbin.org/delete
{
  "args": {},
  "data": "",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Host": "eu.httpbin.org",
  },
  "json": null,
  "origin": "199.231.248.34",
}
```

###### POST
```bash
% cc-curl -X POST http://eu.httpbin.org/post \
-d '{"key": "value"}' \
-H "Content-Type: application/json"
{
  "args": {},
  "data": "{\"key\": \"value\"}",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Content-Length": "16",
    "Content-Type": "application/json",
    "Host": "eu.httpbin.org",
  },
  "json": {
    "key": "value"
  },
  "url": "http://eu.httpbin.org/post"
}
```

###### PUT
```bash
% cc-curl -X PUT http://eu.httpbin.org/put \
-d '{"key": "value2"}' \
-H "Content-Type: application/json"
{
  "args": {},
  "data": "{\"key\": \"value2\"}",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Content-Length": "17",
    "Content-Type": "application/json",
    "Host": "eu.httpbin.org",
  },
  "json": {
    "key": "value2"
  },
  "url": "http://eu.httpbin.org/put"
}
```

###### Verbose
```bash
% cc-curl -v http://eu.httpbin.org/get
connecting to eu.httpbin.org
Sending request:
> GET /get HTTP/1.1
> Connection: close
> Host: eu.httpbin.org
> Accept: */*
>
< HTTP/1.1 200 OK
< Date: Tue, 26 Mar 2024 17:18:53 GMT
< Content-Type: application/json
< Content-Length: 227
< Connection: close
< Server: gunicorn/19.9.0
< Access-Control-Allow-Origin: *
< Access-Control-Allow-Credentials: true
<
{
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "eu.httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-6603037d-40849eea6427eb6e5e9d586e"
  },
  "origin": "199.231.248.34",
  "url": "http://eu.httpbin.org/get"
}
```

### Future Improvements
* Tests
* Add support for HEAD and PATCH
* Add HTTPS support
* Support url-encoded data in `-d` option
* Follow redirects
