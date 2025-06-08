curl -f -X POST http://127.0.0.1:3000/upload \
  -F "png_input=@cc_red_alert_inet.png;type=image/png" \
  -F "json_input=@cc_red_alert_inet.json;type=application/json" \
  --output result.zip



url -f -X POST http://127.0.0.1:3033/upload \  
-F "png_input=@cc_red_alert_inet.png;type=image/png" \
-F "json_input=@cc_red_alert_inet.json;type=application/json" \
--output result.zip

curl -f -X POST http://127.0.0.1:3033/upload \  
-F "png_input=@cc_red_alert_inet.png;type=image/png" \
-F "json_input=@x__broken-config-1.json;type=application/json" \
--output result.zip



curl -s -w "%{http_code}" -X POST http://127.0.0.1:3033/upload \
-F "png_input=@reduced.png;type=image/png" \
-F "json_input=@reduced.json;type=application/json" \
--output result.zip