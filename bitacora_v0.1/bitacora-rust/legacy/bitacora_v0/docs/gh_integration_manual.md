# Bitácora — Manual de integración de GitHub CLI (gh)

Este documento describe, paso a paso y con el nivel técnico adecuado, todo lo que hicimos para habilitar `gh` en el flujo de `bitácora`, cómo funciona, y cómo reproducirlo en otro entorno. Incluye los cambios de scripts realizados, variables de configuración y comprobaciones. También se incluye una captura de la pantalla de autorización para referencia visual.

## Objetivo
Que `bitácora` sea capaz de publicar información en GitHub (push de ramas y creación de PRs) de forma segura y controlada sin NUNCA realizar merges automáticos.

## Archivos modificados / añadidos
- `.bitacora/scripts/lib_bitacora.sh`
  - Añadidas funciones de automatización Git: `auto_commit_if_enabled`, `auto_create_pr_if_enabled`, `detect_git_remote_type`, `extract_repo_info`, `generate_pr_body_from_record` y `gh_authenticated`.
  - Cambios clave: empuja la rama antes de crear PRs (salvo si `BITACORA_NO_AUTO_PUSH=1`), crea PRs en draft por defecto y no realiza merges.
- `.bitacora/scripts/end_session.sh`
  - Ahora llama a la función de creación de PRs después del backup y parada del daemon; mensajes claros sobre "draft" y "no merges".
- `.bitacora/scripts/git_setup.sh`
  - Detección de `gh` y estado de autenticación; instrucciones para login interactivo o mediante token; ejemplo de comando `gh pr create --draft`.
- `.bitacora/instructions/GITSETUP.md`
  - Documentación de nuevas variables: `BITACORA_NO_AUTO_PUSH` y `BITACORA_PR_DRAFT`.

## Variables de configuración importantes
- `BITACORA_NO_AUTO_COMMIT=1` — Desactiva commits automáticos tras `ACTION`.
- `BITACORA_NO_AUTO_PUSH=1` — Evita que `bitácora` haga `git push` automático antes de crear PR.
- `BITACORA_NO_AUTO_PR=1` — Evita que `bitácora` cree PRs automáticamente.
- `BITACORA_PR_DRAFT` — Por defecto `1`. Si se pone `0`, `bitácora` intentará crear PRs no-draft.

> Recomendación: para un comportamiento seguro y auditable, dejar `BITACORA_PR_DRAFT=1` (o no definirla) y, si prefieres empujar manualmente, exportar `BITACORA_NO_AUTO_PUSH=1`.

## Pasos reproducibles para habilitar `gh` en un sistema nuevo

1. Instalar GitHub CLI (Linux/Debian):

```bash
sudo apt update
sudo apt install gh
```

2. Verificar instalación:

```bash
gh --version
```

3. Autenticación interactiva (recomendada si la máquina puede abrir un navegador):

```bash
gh auth login
```

- Selecciona `GitHub.com`.
- Selecciona protocolo `HTTPS` o `SSH` según prefieras; en nuestro caso usamos `SSH` porque la máquina ya tenía una llave configurada.
- Sigue el flujo (se abrirá una URL o se mostrará un código); autoriza la app y concede permisos.

4. Alternativa con token (sin navegador):

- Genera un Personal Access Token (PAT) en GitHub con permisos `repo` y `read:org` según necesidad.
- Importa temporalmente en el shell:

```bash
export GITHUB_TOKEN=ghp_xxx...
# opcional: hacer login automático con gh
echo "$GITHUB_TOKEN" | gh auth login --with-token
```

5. Comprobar estado de `gh` y la configuración de `bitácora`:

```bash
gh auth status
.bitacora/scripts/git_setup.sh
```

`git_setup.sh` mostrará si `gh` está instalado y autenticado, el repositorio remoto detectado y las variables de configuración actuales.

## Ejecución segura del flujo de bitácora (publicar info en GitHub sin hacer merges)

1. Trabaja normalmente usando `START`, `BRANCH` y `ACTION`.
2. Cuando termines la sesión, ejecuta `END` (o `.bitacora/scripts/end_session.sh`). El comportamiento (según configuración por defecto) será:
   - Registrar la sesión en el record
   - Crear backup del proyecto
   - Parar el daemon de timestamp
   - Empujar la rama a `origin` (a menos que `BITACORA_NO_AUTO_PUSH=1`)
   - Crear un PR como `draft` (a menos que `BITACORA_NO_AUTO_PR=1`)

Ejemplo (manual) para crear PR draft con `gh`:

```bash
# asegurarse de estar en la rama correcta
git checkout 20250820-1144_binnacle_to_bitacora_migration
# (opcional) push manual
git push origin 20250820-1144_binnacle_to_bitacora_migration
# crear PR draft
gh pr create --base main \
  --head 20250820-1144_binnacle_to_bitacora_migration \
  --title "chore(bitacora): migrate binnacle -> bitácora, add automation scripts and git integration" \
  --body "Migra contenido legacy de 'binnacle/' a '.bitacora/'. Añade scripts e instrucciones (START/BRANCH/ACTION/TOPIC/STATUS/BACKUP/GITSETUP/END) y fixes menores." \
  --draft
```

> Nota: `bitácora` ya respeta la política de NO hacer merges. PRs se crean solo para documentar y revisar.

## Qué cambiamos en el código de `bitácora` (resumen técnico)

- `lib_bitacora.sh`
  - `auto_commit_if_enabled(commit_msg)` — realiza `git add .` + `git commit -m` si hay cambios y si no estamos en `main`/`master` y si `BITACORA_NO_AUTO_COMMIT` no está establecida.
  - `auto_create_pr_if_enabled(branch, title, body)` — empuja la rama (a menos que `BITACORA_NO_AUTO_PUSH=1`) y usa `gh` o `glab` para crear PRs. Crea PRs como draft por defecto (`BITACORA_PR_DRAFT=1`). Nunca realiza merges.
  - `gh_authenticated()` — helper para comprobar autenticación de `gh`.
  - `generate_pr_body_from_record(record_file, branch)` — construye el cuerpo del PR a partir del record (resumen, checklist, últimas acciones).

- `end_session.sh`
  - Llama a `auto_create_pr_if_enabled` con título y cuerpo generados; mejora mensajes para enfatizar comportamiento "draft" y "no merges".

- `git_setup.sh` y `GITSETUP.md`
  - Añadimos instrucciones interactivas y por token, ejemplo de comando `gh pr create --draft`, y documentación de nuevas variables.

## Captura de pantalla de autorización

A continuación se incluye la captura que aparece durante el flujo de autorización de `gh` (usada como referencia visual para el usuario cuando autorice el dispositivo):

![Authorize GitHub CLI](gh_auth.png)

> Si el navegador muestra una pantalla similar a la captura, revisa los permisos solicitados (Public SSH keys, Repositories, Gists) y pulsa "Authorize github" si confías en el dispositivo.

## Seguridad y permisos
- `gh` usará la cuenta que autorices. En nuestro caso la autenticación usó la clave SSH del usuario `SlimBron57` y quedó autenticada.
- Revisa los tokens y no los expongas en scripts compartidos. Para automatización en CI usa `GITHUB_TOKEN` con permisos acotados.

## Recuperación y deshacer
- Si quieres desactivar la creación automática de PR/Push temporalmente, exporta las variables mencionadas arriba.
- Para revocar el acceso de `gh` o la clave SSH visita https://github.com/settings/keys o https://github.com/settings/tokens según corresponda.

## Registro de la sesión (qué hicimos exactamente aquí)
1. Instalamos `gh` en la máquina.
2. Añadimos funciones y mejoras en `.bitacora/scripts/lib_bitacora.sh` para soportar `gh` y control por variables de entorno.
3. Actualizamos `end_session.sh` para empujar ramas y crear PRs como draft por defecto.
4. Ejecutamos `gh auth login` en modo SSH y autenticamos la CLI usando la llave SSH ya presente en la máquina.
5. Ejecutamos `.bitacora/scripts/end_session.sh`, que:
   - Hizo backup del proyecto
   - Paró el daemon de timestamps
   - Hizo `git push` y creó un PR draft: https://github.com/SlimBron57/RUST_AUDIO_PROCESSOR/pull/1

---

Si quieres, puedo:
- Añadir una versión en inglés del manual.
- Generar un pequeño script `bitacora/scripts/gh_init.sh` que automatice la detección e invite al usuario a ejecutar `gh auth login` con instrucciones, o
- Incluir la imagen real en el repositorio en `.bitacora/docs/gh_auth.png` (si me confirmas que quieres que la guarde aquí y me pasas el binario o la autorización para subirla desde este entorno). 


---

Documento generado automáticamente por las acciones realizadas en sesión. Si quieres que lo divida en varios archivos (README + HOWTO + CHANGELOG) lo hago.
