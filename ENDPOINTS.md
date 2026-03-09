> [!WARNING]
> Because react-router is a pretentious b\*tch, and I must satisfy it, if you try to access any undefined route, you'll get the index.html instead of a proper error

## Endpoints

### Users

- POST /api/users/signup:
  Accepts: `SignupDto`
  Returns: `UserDto`
- POST /api/users/login:
  Accepts: `LoginDto`
  Returns: NoContent

### Tags

- POST /api/tags/:
  Accepts: `TagPostDto`
  Returns `TagGetDto`
- PUT /api/tags/{id}:
  Accepts: `TagPutDto`
  Returns: NoContent
- GET /api/tags/{id}:
  Accepts: NoContent
  Returns: `TagGetDto`
- GET /api/tags/:
  Accepts: NoContent
  Returns: `TagGetDto[]`
- DELETE /api/tags/{id}:
  Accepts: NoContent
  Returns: NoContent

### Categories

- POST /api/categories/:
  Accepts: `CategoryPostDto`
  Returns `CategoryGetDto`
- PUT /api/categories/{id}:
  Accepts: `CategoryPutDto`
  Returns: NoContent
- GET /api/categories/{id}:
  Accepts: NoContent
  Returns: `CategoryGetDto`
- GET /api/categories/:
  Accepts: NoContent
  Returns: `CategoryGetDto[]`
- DELETE /api/categories/{id}:
  Accepts: NoContent
  Returns: NoContent

## Types

### Users

LoginDto:

```json
{
  "email": "email@string.com",
  "password": "string"
}
```

SignupDto:

```json
{
  "name": "string",
  "email": "email@string.com",
  "password": "string"
}
```

UserDto:

```json
{
  "id": 0,
  "created_at": "2000-01-01",
  "modified_at": "2000-01-01",
  "name": "string",
  "email": "string"
}
```

### Tags

TagPostDto:

```json
{
  "name": "string"
}
```

TagGetDto:

```json
{
  "id": 0,
  "created_at": "2000-01-01",
  "modified_at": "2000-01-01",
  "name": "string"
}
```

TagPutDto:

```json
{
  "id": 0,
  "name": "string"
}
```

### Categories

CategoryPostDto:

```json
{
  "name": "string"
}
```

CategoryGetDto:

```json
{
  "id": 0,
  "created_at": "2000-01-01",
  "modified_at": "2000-01-01",
  "name": "string"
}
```

CategoryPutDto:

```json
{
  "id": 0,
  "name": "string"
}
```
