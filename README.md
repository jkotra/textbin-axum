# TextBin-axum

## Migrations

```
DATABASE_URL="postgres://postgres:password@localhost:5432/textbin" sea-orm-cli migrate refresh
```

# Docker

### Build

```
docker build -t textbin-axum:build .
```

### Export

```
docker save textbin-axum:build | gzip > textbin.tar.gz
```

### Import

```
docker load < textbin.tar.gz
```

### Run!

**Note**: Nake sure that a database named `textbin` exists (or change it to something else!) on your postgres DB.

```
docker run --net host -e DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/textbin -e GRC_SECRET=YOUR_KEY --name textbin-axum --restart unless-stopped -d textbin-axum:build
```