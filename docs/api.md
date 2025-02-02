# Backend api description

Api uses _http_ as proto and _json_ as content type

## Profile _`in dev`_

Data about user profile

### General `/profile`

#### Features

For all requests outdate, invalid and without session token, response will be like this:

```http
HTTP/1.1 401 Unauthorized
...
```

#### `GET` general profile data

**request:**

```http
GET /profile HTTP/1.1
Cookie: [some active session token]
...
```

**response:**

```json
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
