RDK_IP=192.168.15.112

echo "Getting clients"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":3, "method":"org.rdk.RDKShell.getAvailableTypes", "params":{}}' http://$RDK_IP:9998/jsonrpc | jq


echo "Closing Cobalt"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":3, "method":"org.rdk.RDKShell.destroy", "params":{"callsign": "Cobalt"}}' http://$RDK_IP:9998/jsonrpc | jq

sleep 10

echo "Launching Cobalt without deeplink"
curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":5, "method":"org.rdk.RDKShell.launch", "params":{"callsign": "Cobalt"}}' http://$RDK_IP:9998/jsonrpc | jq

sleep 20

echo "Requesting Cobalt to open a deeplink"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":6, "method":"org.rdk.RDKShell.launch", "params":{"callsign": "Cobalt","configuration":{"deeplink": "https://www.youtube.com/tv?list=OLAK5uy_mKAu6VNK3gMSq_L8fU_C6myQnuuuIzvWY"}}}' http://$RDK_IP:9998/jsonrpc | jq

sleep 10

echo -s "Sending a Enter to confirm who's watching"

curl --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":3, "method":"org.rdk.RDKShell.1.injectKey", "params":{"keyCode": 13}}' http://$RDK_IP:9998/jsonrpc | jq

sleep 40

echo "Closing Cobalt"

curl -s --header "Content-Type: application/json" --request POST --data '{"jsonrpc":"2.0", "id":7, "method":"org.rdk.RDKShell.destroy", "params":{"callsign": "Cobalt"}}' http://$RDK_IP:9998/jsonrpc | jq

sleep 10