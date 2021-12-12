# Curl Example

## Example

### Change post view

```bash
curl --location --request PATCH 'http://192.168.50.3:3000/api/v1/post/posts/385554834469696003' \
--header 'accept-language: zh,en-US;q=0.8,en;q=0.7' \
--header 'Authorization: Bearer token' \
--header 'Content-Type: application/json' \
--header 'x-client-version: 1.0.0' \
--header 'x-client-platform: iOS' \
--header 'x-client-id: 377844742802649603' \
--data-raw '{
    "viewed_count_action":"increase_one"
}'
```

```bash

```
