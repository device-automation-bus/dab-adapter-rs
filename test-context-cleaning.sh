RDK_IP=192.168.15.112

echo "Closing Cobalt"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":3, "method":"org.rdk.RDKShell.destroy", "params":{"callsign": "Cobalt"}}' http://$RDK_IP:9998/jsonrpc | jq

sleep 5

echo "Requesting Cobalt to launch with a deeplink"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":6, "method":"org.rdk.RDKShell.launch", "params":{"callsign": "Cobalt","configuration":{"url": "https://www.youtube.com/tv?list=OLAK5uy_mKAu6VNK3gMSq_L8fU_C6myQnuuuIzvWY"}}}' http://$RDK_IP:9998/jsonrpc | jq

sleep 10

echo "Sending a Enter to confirm who's watching"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":3, "method":"org.rdk.RDKShell.1.injectKey", "params":{"keyCode": 13}}' http://$RDK_IP:9998/jsonrpc | jq

echo "Waiting some seconds"

sleep 20

echo "Closing Cobalt"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":3, "method":"org.rdk.RDKShell.destroy", "params":{"callsign": "Cobalt"}}' http://$RDK_IP:9998/jsonrpc | jq

sleep 10

echo "Launching Cobalt without deeplink. It should shows up Youtube Initial screen"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":5, "method":"org.rdk.RDKShell.launch", "params":{"callsign": "Cobalt"}}' http://$RDK_IP:9998/jsonrpc | jq

echo "Waiting some seconds and check if it is in the Initial Screen"

sleep 20

echo "Closing Cobalt"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":7, "method":"org.rdk.RDKShell.destroy", "params":{"callsign": "Cobalt"}}' http://$RDK_IP:9998/jsonrpc | jq

sleep 5