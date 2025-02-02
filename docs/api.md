# Backend api description

Api uses **http** as proto and **json** as content type

# Authorization

Authorization with github and dev for test

## `POST` `/signin` _in dev_

_Description in progress..._

## `POST` `/signin/dev` _in dev_

Dev signin for test profile features avaliable only at debug build

### request

```text
POST /signin/dev HTTP/1.1
...
```

### response

```text
HTTP/1.1 200 OK
Set-Cookie: [some session token]
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

## `GET` `/profile` _in dev_

Get general profile data

### request

```text
GET /profile HTTP/1.1
Cookie: [some active session token]
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
{
  "nickname": "boris2001",  // string
  "iconUrl": "https://...", // string
  "type": "Full",           // "Full"
  "addressAmount": 12       // unsigned number
}
```

## `GET` `/profile/username` _in dev_

### request

```text
GET /profile HTTP/1.1
Cookie: [some active session token]
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
{
  "nickname": "boris2001",  // string
}
```
