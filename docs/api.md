# Backend api description

Api uses **http** as proto and **json** as content type

# Authorization

Authorization with github and dev for test

## `POST` `/auth` _in dev_

_Description in progress..._

## `POST` `/auth/dev`

Dev signin for test profile features avaliable only at debug build

### request

```text
POST /auth/dev HTTP/1.1
...
```

### response

```text
HTTP/1.1 200 OK
Set-Cookie: token=[some session token]
```

# Profile

Data about user profile

## Features

For all requests outdate, invalid and without session token, response will be like this:

### response

```text
HTTP/1.1 401 Unauthorized
...
```

## `GET` `/profile`

Get general profile data

### request

```text
GET /profile HTTP/1.1
Cookie: token=[some active session token]
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
{
  "username": "boris2001",  // string
  "iconUrl": "https://...", // string
  "type": "Full",           // "Full"
  "addressAmount": 12       // unsigned number
}
```

## `GET` `/profile/username`

### request

```text
GET /profile HTTP/1.1
Cookie: token=[some active session token]
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
{
  "username": "boris2001", // string
}
```

# History

## Features

Return 401 Unauthorized code if request does not have session token cookie or invalid session token

## `GET` `/history?page=0` _in dev_

Get user ping history by page number

> One page contains 0-24 items, return empty list if page empty

### request

```text
GET /history?page=0 HTTP/1.1
Cookie: token=[some active session token]
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
[
  {
    "id": 1                   // unsigned number
    "address": "https://...", // string
    "timestamp": "12345789",  // unsigned number
    "ping": {
      "min": 29,              // unsigned number
      "avg": 85,              // unsigned number
      "max": 451              // unsigned number
    }
  },
  ...
]
```

