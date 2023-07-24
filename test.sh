ID="B827EB43F831"
# HOST=192.168.15.112
HOST=127.0.0.1

# Define a function
delay() {
	echo "Waiting $1 seconds"
	sleep $1
}

dab_send() {
	params=$2
	if [ -z "$params" ]
	then
		params="{}"
	fi

	mosquitto_pub -h $HOST -t dab/$ID/$1 -m "$2" -D publish response-topic "dab/response/topic"
}

dab_send applications/list '{}'
# delay 3
# dab_send applications/exit '{ "appId": "Cobalt" }'
# delay 5
# dab_send applications/launch '{ "appId": "Cobalt" }'
# delay 30
# dab_send applications/exit '{ "appId": "Cobalt" }'
# delay 5
# dab_send applications/exit '{ "appId": "Youtube" }'
# delay 3
# dab_send applications/launch-with-content '{ "appId": "Youtube", "contentId": "https://www.youtube.com/tv?list=OLAK5uy_mKAu6VNK3gMSq_L8fU_C6myQnuuuIzvWY"}'
# delay 30
# dab_send applications/launch-with-content '{ "appId": "Youtube", "contentId": "https://www.youtube.com/tv?launch=voice&vq=Play%20mission%20Impossible"}'
# delay 30
# dab_send applications/exit '{ "appId": "Youtube" }'
# delay 3
# dab_send applications/launch-with-content '{ "appId": "Youtube", "contentId": "https://www.youtube.com/tv?list=OLAK5uy_mKAu6VNK3gMSq_L8fU_C6myQnuuuIzvWY"}'
# delay 15
# dab_send applications/exit '{ "appId": "Youtube" }'
# delay 3
# dab_send applications/exit '{ "appId": "Youtube" }'
# delay 3
# dab_send applications/get-state
# delay 3
# dab_send dab/discovery
# delay 3
# dab_send system/restart
# delay 3
# dab_send operations/list
# delay 3
# dab_send dab/bridge/teste/add-device '{ "ip": "192.168.15.112" }'
# delay 3
# dab_send dab/bridge/teste/list
# delay 3
# dab_send applications/launch '{ "appId": "Youtube" }'
# delay 3
# dab_send operations/list
# delay 3
# dab_send output/image '{"outputLocation": "http://192.168.15.185:3000"}'
# delay 3
# dab_send invalid
# delay 3
# dab_send voice/send-text '{ "requestText":  "Alexa, whats the weather in new york city?", "voiceSystem": "Alexa" }'
# delay 3
# dab_send voice/send-audio '{"voiceSystem": "Alexa", "fileLocation": "https://gitlab.collabora.com/collabora/dab-rdk-bridge/-/raw/main/voice-request-samples/weather.wav?inline=false" }'
# delay 3
# dab_send input/key/list
# delay 3
# dab_send input/key-press '{"keyCode": "KEY_VOLUME_UP"}'
# delay 3
# dab_send input/key-press '{"keyCode": "KEY_VOLUME_DOWN"}'
# delay 3
# dab_send input/long-key-press '{"keyCode": "KEY_RIGHT","durationMs": 5000}'
# delay 10
# dab_send input/long-key-press '{"keyCode": "KEY_LEFT","durationMs": 5000}'
# delay 5
# dab_send system/settings/list
# delay 3
# dab_send system/settings/get
# delay 3
# dab_send system/settings/set '{"outputResolution": { "width": 1280, "height": 720, "frequency": 60}}'
# # delay 3
# dab_send system/settings/set '{"outputResolution": { "width": 1920, "height": 1080, "frequency": 60}}'
# # delay 3
# dab_send system/settings/set '{"audioVolume": 75}'
# delay 3
# dab_send system/settings/set '{"audioVolume": 0}'
# dab_send device-telemetry/start '{"duration": 250}'
# delay 10
# dab_send device-telemetry/start '{"duration": 1000}'
# delay 10
# dab_send device-telemetry/stop