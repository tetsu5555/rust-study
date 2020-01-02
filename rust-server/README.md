```
cd docker
docker-compose up -d
docker-compose exec rust-actix-web bash

curl localhost:8777/get
curl -X POST localhost:8777/post
```

actix-webっていうフレームワークを使った
https://actix.rs/docs/installation/