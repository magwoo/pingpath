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

- For request without session token backend create guest session
- For request with invalid session, response is `401 Unauthorized`

## `GET` `/profile`

Get general profile data

### request

For authorized with github user

```text
GET /profile HTTP/1.1
Cookie: token=[some authorized session token]
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
{
  "username": "boris2001", // string
  "imgUrl": "https://...", // string | null
  "type": "Full",          // "Trial" | "Full" 
  "addressAmount": 12      // unsigned number | null
}
```

### request

For guest

```text
GET /profile HTTP/1.1
Cookie: token=[some guest session token]
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
{
  "username": "guest", // string
  "type": "Trial"      // "Trial" | "Full"
}
```

## `GET` `/profile/username`

### request

For authorized with github session token

```text
GET /profile HTTP/1.1
Cookie: token=[some authorized session token]
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
{
  "username": "boris2001", // string
  "imgUrl": "https://..."  // string | null
}
```

### request

For guest session

```text
GET /profile HTTP/1.1
Cookie: token=[some guest session token]
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
{
  "username": "guest" // string
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

# Locations

## `GET` `/locations`

Get our locations

### request

```text
GET /locations HTTP/1.1
...
```

### response

```text
HTTP/1.1 200 OK
Content-Type: application/json
...
{
  "1.1.1.1": {
    "city": {                      // object | null
      "geoname_id": 1              // number
      "names": {                   // object | null
        "en": "City name"          // string
        ...
      }
    },
    "country": {                   // object | null
      "geoname_id": 1              // number
      "is_in_european_union": true // boolean
      "iso_code": "NL"             // string
      "names": {                   // object | null
        "en": "Country name"       // string
        ...
      }
    },
    "location": {                  // object | null
      "accuracy_radius": 20        // number
      "latitude": 52.3759          // number
      "longitude": 4.8975          // number
      "metro_code": 1              // number | null
      "time_zone": "TZ/name"       // string
    }
  },
  ...
}
```

