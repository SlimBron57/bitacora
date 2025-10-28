
# Speaker Player — Spec (MQTT On‑Demand, BYO API Key)

**Estado:** MVP plan • **Alcance:** Sólo reproductor/connector (sin emojis) • **Modelo licencias:** BYO API key por usuario

---

## 1) Licencias / DRM (BYO Key)
- **Modelo:** Cada usuario conecta su **API key** del proveedor (p. ej., Napster).
- **UX:** Pantalla “Conectar proveedor” → pegar key / OAuth → verificar scopes/caducidad.
- **Almacenamiento:** Key en **Keychain/Keystore**; jamás en logs ni texto plano.
- **Playback:** Siempre mediante **SDK/API oficial** (Android: ExoPlayer+Widevine; iOS: AVPlayer+FairPlay), conforme a TOS.
- **Offline/cache:** Sólo si lo permite el SDK y dentro de su **sandbox cifrado**.
- **Playlists en píxeles:** Guardan **metadatos** (IDs/ISRC/autor/álbum), **nunca audio** ni carátulas bloqueadas por licencia.
- **Atribución:** Incluir `source:"napster"` (u otro) en metadatos y respetar marcas/guías.

---

## 2) Arquitectura de control (MQTT “on‑demand”)
**Esquema de tópicos**
```
bitacora/{user}/players/{playerId}/cmd    // { op, args..., corrId }
bitacora/{user}/players/{playerId}/state  // retained snapshot: { status, track, posMs, durationMs, volume, ts }
bitacora/{user}/players/{playerId}/evt    // eventos puntuales (play, pause, error)
```

**Patrón de consulta sobria**
- Cuando el usuario **despierta** pantalla o **pide por voz**: enviar `REQ_STATE` a `.../cmd`.
- El Player responde actualizando `.../state` (retained) con snapshot **fresco**.
- En **pausa**: desconectar MQTT o usar keep‑alive alto (60–120 s).
- En **reproducción**: mantener MQTT conectado; publicar cambios relevantes con QoS1.
- **Reconexión:** backoff exponencial + re‑suscripción automática.

---

## 3) Reproducción y background (móvil)

### Android
- **Servicio en primer plano** + **MediaSessionCompat** (notificación con controles).
- **AudioFocus**/ducking y manejo de rutas de salida (speaker/BT).
- **Reproductor:** ExoPlayer (o SDK del proveedor) con DRM correspondiente.
- **MQTT:** conexión estable en reproducción; en pausa, “sleep” (desconectar o keep‑alive alto).
- **Permisos:** foreground service + internet. No pedir `RECORD_AUDIO` si no se usa voz.

### iOS
- **Background Audio** con `AVAudioSession` categoría `playback`.
- **Now Playing / Remote Command Center**: título, progreso, controles hardware/CarPlay.
- **MQTT:** activo en reproducción. En pausa, desconectar y **reconectar on‑demand** (pantalla/voz).
- **Silent push** (opcional) si necesitas “despertar” para consultar estado (cumpliendo políticas).
- **Interrupciones:** pausar/restaurar en llamadas/Siri.

---

## 4) Salida de audio: Alexa vs Parlantes BT
- **Alexa (Echo):** usar **sólo como parlante A2DP** (Bluetooth). No sirve como mic genérico.
- **Parlantes BT:** soportados como salida A2DP igual que cualquier speaker.
- **Recomendación:** implementar primero **parlantes BT**; Alexa entra “gratis” como un BT más.

---

## 5) Multi‑room (ganchos, sin implementar)
- Añadir campos opcionales a comandos/estado:
  - `startAtEpochMs`: arranque programado con reloj común (futuro).
  - `clockSkewMs`: medición/compensación de deriva.
- Cuando haya audiencia real: NTP para reloj, prebuffer y correcciones de drift.

---

## 6) Seguridad y privacidad
- API keys cifradas (Keychain/Keystore), rotación y revocación manejadas por el usuario.
- Telemetría mínima y agregada (errores, métricas de rendimiento), sin PII.
- Exportar playlists en píxeles: opción de **firma** (Ed25519) y **cifrado** de campos sensibles.

---

## 7) Entregables por fases

### Fase 1 — MVP (single‑device)
- Conectar API key → reproducir pista/cola local.
- MediaSession/NowPlaying funcionando (Android/iOS).
- MQTT básico: `state` retained + `REQ_STATE` on‑demand.
- Export/Import **playlist en píxeles** (sólo metadatos).

### Fase 2 — Control remoto
- Controller móvil ↔ Player en segundo dispositivo por MQTT.
- Conmutación de salida a **parlante BT** (incluye Echo como BT).

### Fase 3 — Robustez
- Reconexión sólida, backoff, tolerancia a redes.
- Pruebas de batería/CPU en sesiones largas.

### Fase 4 — (Opcional) Multi‑room
- Hooks activos (`startAtEpochMs`), pruebas de sincronía básica y drift.

---

## 8) API mínima de comandos (ejemplos)
```json
// CMD play
{ "op": "PLAY", "trackId": "tr_123", "source": "napster", "corrId": "abc" }

// CMD pause
{ "op": "PAUSE", "corrId": "def" }

// CMD seek
{ "op": "SEEK", "ms": 91321, "corrId": "ghi" }

// CMD set queue
{ "op": "SET_QUEUE", "items": [{ "id": "tr_1", "source": "napster" }], "corrId": "jkl" }
```

**STATE (retained)**
```json
{
  "status": "playing",
  "track": { "id": "tr_123", "source": "napster" },
  "posMs": 35123,
  "durationMs": 183000,
  "volume": 72,
  "ts": 1730486400000
}
```

---

## 9) Notas de implementación
- Mantener una **abstracción de Player**: Android (ExoPlayer), iOS (AVPlayer) detrás de una misma interfaz.
- Rutas de salida: preferir A2DP cuando haya BT; fallback a speaker.
- Documentar claramente en UI el **estado de conexión** y la **ruta** activa.
