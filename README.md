# Learning Axum
Understanding axum rust web framework. Routes, Handlers, HTTP methods actions, authentication and custom middlewares.

## Language
### Rust
```
v1.85.0
```

## Installation
### Clone
```
git clone git@github.com:Njaya2019/learning_axum.git
```

### hello_world DIR
```
cd hello_world
```

### Start postgres database with docker-compose
```
docker-compose up -d
```

### data DIR
```
cd data
```

### Run axum
```
cargo watch -x run
```

### 📦 Base URL
```
localhost:3000
```

## 📘 API Endpoints

### 🔐 Authentication
Some routes require a Bearer token in the header:

### Tasks

| Method | Endpoint              | Description              | Auth Required |
|--------|-----------------------|--------------------------|---------------|
| POST   | `/tasks`              | Create task              | ✅ Yes        |
| GET    | `/tasks`              | Get all tasks            | ✅ Yes        |
| GET    | `/tasks/id`           | Get one task             | ✅ Yes        |
| PUT    | `/tasks/id`           | Update one task          | ✅ Yes        |
| PATCH  | `/tasks/id`           | Partial update one task  | ✅ Yes        |
| DELETE | `/tasks/id`           | Parmanently remove task  | ✅ Yes        |
| DELETE | `/tasks/id?soft=true` | Temporary remove task    | ✅ Yes        |


### Authentication
| Method | Endpoint              | Description              | Auth Required |
|--------|-----------------------|--------------------------|---------------|
| POST   | `/login`              | Login user               | ❌ No         |
| POST   | `/logout`             | Logout user              | ❌ No         |
| POST   | `/users`              | Add new user             | ❌ No         |
