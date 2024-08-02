# Rust Actix based micro service for pet tracking

## About
This is a simple microservice which uses mongo db as backend for storing all pet informations.APIs will fetch or create pet records within database.

## How to run
```
export MONGO_URI=<mongo connection uri>
make run
```

## Linting the code
```
make lint
```

## Testing
```
make test
```

## ToDo
- Authentication via JWT 
- BDD based integration test
