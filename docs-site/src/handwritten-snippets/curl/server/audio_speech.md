---
id: legacy_curl_server_audio_speech
language: bash
target: bash
level: syntax
requires: []
side_effect: network
---

```bash
curl http://localhost:4000/v1/audio/speech \
  -H "Authorization: Bearer $LITER_LLM_MASTER_KEY" \
  -H "Content-Type: application/json" \
  --output speech.mp3 \
  -d '{
    "model": "tts-1",
    "voice": "alloy",
    "input": "Backoff with jitter prevents synchronised retries."
  }'
```
