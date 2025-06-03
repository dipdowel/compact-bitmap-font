curl -f -X POST http://127.0.0.1:3000/upload \
  -F "png=@cc_red_alert_inet.png;type=image/png" \
  -F "json=@cc_red_alert_inet.json;type=application/json" \
  --output result.zip
