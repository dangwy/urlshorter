# 🦀 urlshorter
`urlshorter` is a high-performance and fully functional short url service in rust, you can use it directly in the production environment.

Uses [Auxm](https://github.com/tokio-rs/axum), [SeaOrm](https://github.com/SeaQL/sea-orm) and [PostgreSQL](https://www.postgresql.org/),  the system architecture references [RUSTfulapi](https://github.com/robatipoor/rustfulapi).

The [Redis](https://github.com/mitsuhiko/redis-rs) is optional, you can uncomment the code in the program if you want to use it.

---

## Features
- It can be transformed into a SaaS service with simple modifications
- Provides short url CRUD functionality
- Support labeling different URLs
- Support JWT authentication (just need to modify it according to your business requirements)

## How To Deploy
### Running locally
```shell
./run
# open swagger panel
xdg-open http://127.0.0.1:8080/swagger-ui/
# manually testing your API routes with curl commands
curl -X GET http://127.0.0.1:8080/api/v1/server/health_check
```

### Running in Docker
```shell
cd ./docker/dev/ && ./up.sh
```

## How To Use It
### Configure with toml files
```
settings
├── base.toml # default config file 
├── dev.toml # development config file 
├── prod.toml # production config file
└── test.toml # test config file
```

### Switching profiles
Before running the application, export this variable:
```shell
export APP_PROFILE=prod # Switch to production profile
```

### Migrate database
```shell
cargo run --bin migration -- up -u $DATABASE_URL
```

---

## License

Licensed under either of

* MIT license
  ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)


## Contributing

Contributors are welcome! please fork and send pull requests, If you find a bug
or have any ideas on how to improve this project please submit an issue.

See [CONTRIBUTING.md](CONTRIBUTING.md).