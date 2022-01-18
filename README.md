
1. Build image
```bash
docker build --tag server:local .
```
2. Run application
```bash
docker run -it --rm -p 50051:50051 server:local --server "0.0.0.0:50051"
```