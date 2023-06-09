# TextBin-axum

## Migrations

```
DATABASE_URL="postgres://postgres:password@localhost:5432/textbin" sea-orm-cli migrate refresh
```

# Docker

**NOTE:** `archlinux:base-devel` image is used to match local development environment.

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

```
docker run --net host -e DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/textbin -e GRC_SECRET=YOUR_KEY --restart unless-stopped -d textbin-axum:build
```
