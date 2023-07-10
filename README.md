# blog
Simple Personal Blog With Actix Web and SeaORM


### generate entities
```sh
sea-orm-cli generate entity -o src/models --with-serde both --serde-skip-deserializing-primary-key --date-time-crate chrono
```