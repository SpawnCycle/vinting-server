> [!WARNING]
> Because react-router is a pretentious b\*tch, and I must satisfy it, if you try to access any undefined route, you'll get the index.html instead of a proper error

## Endpoints

### Products

- POST /api/products/:
  Requires: Logged in
  Accepts: `ProductPostDto` | `ProductForm`
  Returns: `ProductGetDto`
- POST /api/products/{id}/order:
  Requires: Logged in
  Accepts: `OrderPostDto`
  Returns: `OrderGetDto`
- PUT /api/products/{id}:
  Requires: Logged in and the product is yours && the new stock is higher
  Accepts: `ProductPutDto`
  Returns: NoContent
- DELETE /api/products/{id}:
  Requires: Logged in and the product is yours || Admin role
  Accepts: NoContent
  Returns: NoContent
- GET /api/products/{id}
  Accepts: NoContent
  Returns: `ProductGetDto`
- GET /api/products
  Accepts: NoContent
  Returns: `ProductGetDto[]`
  URI Options:
  - gender: `string` (optional)
  - size: `string` (optional)
  - color: `string` (optional)
  - condition: `string` (optional)
  - categories: `string[]` (optional)
  - page: `number` (default: 1, first page has the index of 1)
  - page_size: `number` (default: 10)

### Orders

- GET /api/orders/:
  Details: Returns all of the orders for the logged in user
  Requires: Logged in
  Accepts: NoContent
  Returns: `OrderGetDto[]`
- GET /api/orders/{id}:
  Requires: Logged in | Admin role
  Accepts: NoContent
  Returns: `OrderGetDto[]`
- GET /api/orders/all:
  Details: Returns all of the orders regardless of who it belongs to
  Requires: Admin role
  Accepts: NoContent
  Returns: `OrderGetDto[]`
- DELETE /api/orders/{id}:
  Requires: Admin role
  Accepts: NoContent
  Returns: NoContent

### Users

- POST /api/users/signup:
  Accepts: `SignupDto`
  Returns: `UserDto`
- POST /api/users/login:
  Accepts: `LoginDto` | `LoginForm`
  Returns: NoContent
- POST /api/users/logout:
  Accepts: NoContent
  Returns: NoContent
- GET /api/users/whoami
  Requires: Logged in
  Accepts: NoContent
  Returns: `WhoamiDto`
- GET /api/users/
  Accepts: NoContent
  Returns: `UserDto[]`
- GET /api/users/{id}
  Accepts: NoContent
  Returns: `UserDto`

### Images

- POST /api/images
  Requires: Logged in
  Accepts: `ImageForm`
  Returns: `ImageDto`
- GET /api/images
  Accepts: NoContent
  Returns: `ImageDto[]`
- GET /api/images/{id}
  Accepts: NoContent
  Returns: `ImageDto`

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
  Requires: Admin role
  Accepts: `CategoryPostDto`
  Returns `CategoryGetDto`
- PUT /api/categories/{id}:
  Requires: Admin role
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

## Forms:

LoginForm:

- email: string checked against the "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$" regex string
- password: string

ImageForm:

- image: file (<!-- Currently only --> accepts png files)

ProductForm:

- title: string
- description: string
- brand: string
- categories: string[]
- tags: string[] (if a tag doesn't exist, it will create it)
- condition: string
- gender: string
- size: string
- color: string
- price: number
- stock: number
- images: file[] (Accepts the same file types and the image form)

## Types

### Error

Any and all errors are returned in this object

```json
{
  "code": 400,
  "timestamp": "2000-01-01 andsomeotherutcstuff",
  "message": "string"
}
```

### Products

ProductPostDto:

```json
{
  "name": "string",
  "description": "string",
  "price": 1,
  "size": "string",
  "color": "string",
  "brand": "string | null",
  "condition": "New | Like new | Used | Heavily used",
  "sex": "Male | Female | Unisex",
  "stock": 1,
  "categories": [1],
  "tags": [1],
  "images": [1]
}
```

ProductPutDto:

```json
{
  "id": 1,
  "name": "string",
  "description": "string",
  "price": 1,
  "size": "string",
  "color": "string",
  "brand": "string | null",
  "condition": "New | Like new | Used | Heavily used",
  "sex": "Male | Female | Unisex",
  "stock": 1,
  "categories": [1],
  "tags": [1],
  "images": [1]
}
```

ProductPagination:

```json
{
  "pages": 5
  "items": 10,
  "data": [
    {
      "id": 1,
      "created_at": "2000-01-01",
      "modified_at": "2000-01-01",
      "name": "string",
      "description": "string",
      "price": 1,
      "size": "string",
      "brand": "string | null",
      "condition": "New | Like new | Used | Heavily used",
      "sex": "Male | Female | Unisex",
      "available_stock": 5,
      "has_stock": true,
      "user": {
        "id": 1,
        "created_at": "2000-01-05",
        "modified_at": "2000-01-05",
        "name": "string",
        "email": "email@email.com"
      },
      "categories": [
        {
          "id": 1,
          "created_at": "2000-01-05",
          "modified_at": "2000-01-05",
          "name": "string"
        }
      ],
      "tags": [
        {
          "id": 1,
          "created_at": "2000-01-05",
          "modified_at": "2000-0l-05",
          "name": "string"
        }
      ],
      "images": ["string"]
    }
  ],
}
```

ProductGetDto:

```json
{
  "id": 1,
  "created_at": "2000-01-01",
  "modified_at": "2000-01-01",
  "name": "string",
  "description": "string",
  "price": 1,
  "size": "string",
  "brand": "string | null",
  "condition": "New | Like new | Used | Heavily used",
  "sex": "Male | Female | Unisex",
  "available_stock": 5,
  "has_stock": true,
  "user": {
    "id": 1,
    "created_at": "2000-01-05",
    "modified_at": "2000-01-05",
    "name": "string",
    "email": "email@email.com"
  },
  "categories": [
    {
      "id": 1,
      "created_at": "2000-01-05",
      "modified_at": "2000-01-05",
      "name": "string"
    }
  ],
  "tags": [
    {
      "id": 1,
      "created_at": "2000-01-05",
      "modified_at": "2000-01-05",
      "name": "string"
    }
  ],
  "images": ["string"]
}
```

### Orders

OrderPostDto

```json
{
  "ammount": 1
}
```

OrderGetDto

```json
{
  "id": 1,
  "created_at": "2000-01-05",
  "modified_at": "2000-01-05",
  "user_id": 1,
  "ammount": 1,
  "arrived_at": "2000-01-05 | null",
  "product": {
    "id": 1,
    "created_at": "2000-01-01",
    "modified_at": "2000-01-01",
    "name": "string",
    "description": "string",
    "price": 1,
    "size": "string",
    "brand": "string | null",
    "condition": "New | Like new | Used | Heavily used",
    "sex": "Male | Female | Unisex",
    "available_stock": 5,
    "has_stock": true,
    "user": {
      "id": 1,
      "created_at": "2000-01-05",
      "modified_at": "2000-01-05",
      "name": "string",
      "email": "email@email.com"
    },
    "categories": [
      {
        "id": 1,
        "created_at": "2000-01-05",
        "modified_at": "2000-01-05",
        "name": "string"
      }
    ],
    "tags": [
      {
        "id": 1,
        "created_at": "2000-01-05",
        "modified_at": "2000-01-05",
        "name": "string"
      }
    ],
    "images": ["string"]
  }
}
```

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

WhoamiDto:

```json
{
  "id": 0,
  "created_at": "2000-01-01",
  "modified_at": "2000-01-01",
  "name": "string",
  "email": "string",
  "roles": ["role1", "role2"]
}
```

### Images

ImageDto

```json
{
  "id": 0,
  "created_at": "2000-01-01",
  "modified_at": "2000-01-01",
  "url": "http://localhost:8000/img/afilename.png"
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
