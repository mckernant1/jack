# Jack

Cloudwatch log fetching CLI. Works off local aws config

### Regex Searching

```bash
jack search \
  --key ERROR \
  --log-group /ecs/lol-predictions-bot \
  --start-time '2 hours ago' \
  --end-time 'now'
```

### Raw fetching with Query
```bash
jack raw \
  --log-group /ecs/lol-predictions-bot \
  --start-time '2 hours ago' \
  --end-time 'now' \
  --format json \
  --query-string "
fields @message, @timestamp
| filter @message like /INFO/
| limit 100
"
```
