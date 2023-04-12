
1. Build image
```bash
docker build --rm --tag server:local .
```
2. Run server
```bash
docker run -it --rm -p 50051:50051 server:local --server "0.0.0.0:50051"
```
3. Run client
```
cargo run --bin client -- -a localhost:50051
```
