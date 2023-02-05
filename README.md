# TextBin-axum

## Migrations

```
DATABASE_URL="postgres://postgres:password@localhost:5432/textbin" sea-orm-cli migrate refresh
```

# Docker

```
docker build -t textbin-axum:build .
```

```
docker run --net host -e DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/textbin -e GRC_SECRET=YOUR_KEY --restart unless-stopped -d textbin-axum:build
```