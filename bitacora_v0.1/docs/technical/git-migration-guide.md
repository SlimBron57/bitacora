# Guía: Migración de Repositorio Git entre Cuentas y Configuración de Repo Privado

## Descripción del Problema

Durante el desarrollo del proyecto Bitacora, nos encontramos con un repositorio local que estaba conectado a múltiples remotos de diferentes cuentas de GitHub:
- **Cuenta anterior**: `EDGIJA/bitacora-rust` (cuenta que queríamos desconectar)
- **Cuenta nueva**: `SlimBron57/bitacora` (cuenta objetivo, repositorio privado)

### Problemas específicos identificados:
1. **Múltiples remotos**: El repositorio tenía remotos archivados apuntando a diferentes cuentas
2. **Branch mismatch**: Rama local `master` vs. rama remota por defecto `main` en GitHub
3. **Autenticación para repo privado**: Errores de "Repository not found" por falta de credenciales apropiadas
4. **Upstream tracking incorrecto**: La rama local apuntaba a remotos inexistentes o desactualizados

## Solución Paso a Paso

### 1. Análisis del Estado Inicial

**Comando para inspeccionar remotos:**
```bash
git remote -v
git branch -vv
```

**Verificar contenido de `.git/config`:**
```bash
cat .git/config
```

### 2. Limpieza de Remotos (Desconexión de Cuenta Anterior)

**Script para remover remotos de cuenta no deseada:**
```bash
# Detectar y eliminar remotos que apunten a cuenta EDGIJA
edgiya_remotes=$(git remote -v | awk '/github.com\/EDGIJA/{print $1}' | sort -u)
for remote in $edgiya_remotes; do
    git remote remove "$remote"
    echo "Removed remote: $remote"
done
```

**Alternativamente, editar `.git/config` directamente:**
- Eliminar bloques `[remote "nombre"]` que contengan URLs no deseadas
- Actualizar sección `[branch "master"]` si apunta a remotos eliminados

### 3. Configuración de Remoto para Nueva Cuenta

**Verificar/establecer remoto correcto:**
```bash
# Si origin no existe
git remote add origin https://github.com/SlimBron57/bitacora.git

# Si origin existe pero apunta a URL incorrecta
git remote set-url origin https://github.com/SlimBron57/bitacora.git
```

### 4. Migración de Branch Master → Main

**Script para renombrar rama local:**
```bash
#!/usr/bin/env bash
# rename_master_to_main.sh

# Si estamos en master, renombrarla a main
if git show-ref --verify --quiet refs/heads/master; then
    current=$(git rev-parse --abbrev-ref HEAD)
    if [ "$current" = "master" ]; then
        git branch -m main
    else
        git branch -m master main
    fi
    echo "Renamed 'master' -> 'main'"
fi

# Actualizar tracking si el remoto tiene main
git fetch origin --quiet || true
if git ls-remote --heads origin refs/heads/main | grep -q refs/heads/main; then
    git branch --set-upstream-to=origin/main main
    echo "Set upstream to origin/main"
fi
```

**Edición manual de `.git/config`:**
```bash
[branch "main"]
    remote = origin
    merge = refs/heads/main  # Cambiar de refs/heads/master
```

### 5. Organización de Commits (Opcional)

**Script para commits organizados:**
```bash
#!/usr/bin/env bash
# organize_commits.sh

# Función para commitear solo si hay cambios staged
commit_if_changes() {
    msg="$1"
    if git diff --staged --quiet; then
        echo "No staged changes for: $msg"
    else
        git commit -m "$msg" && echo "Committed: $msg"
    fi
}

# Commits por categoría
git add Cargo.toml Cargo.lock src || true
commit_if_changes "chore(init): initial project scaffold"

git add crates/bitacora-core crates/bitacora-session crates/bitacora-config || true
commit_if_changes "feat(core): domain models, timestamp and session utilities"

git add crates/bitacora-records crates/bitacora-storage config docker docker-compose.yml || true
commit_if_changes "feat(storage): mongodb schemas and repository adapters"

git add crates/bitacora-commands scripts || true
commit_if_changes "feat(cli/scripts): session and action scripts, command crate scaffold"

git add docs _map README.md || true
commit_if_changes "docs: architecture, checklist and integration notes"

git add -A || true
commit_if_changes "chore: misc fixes, formatting and CI setup"
```

### 6. Resolución de Problemas de Autenticación para Repos Privados

#### Opción 1: GitHub CLI (Recomendada)
```bash
# Verificar autenticación
gh auth status

# Si no está autenticado
gh auth login

# Verificar acceso al repo privado
gh repo view SlimBron57/bitacora

# Push usando gh (maneja autenticación automáticamente)
gh repo set-default SlimBron57/bitacora
git push -u origin main
```

#### Opción 2: SSH (Más Confiable para Repos Privados)
```bash
# Cambiar remote a SSH
git remote set-url origin git@github.com:SlimBron57/bitacora.git

# Push (requiere SSH keys configuradas)
git push -u origin main
```

#### Opción 3: Personal Access Token (HTTPS)
```bash
# Configurar credential helper
git config --global credential.helper store

# El push pedirá credenciales:
# Username: SlimBron57
# Password: tu_personal_access_token
git push -u origin main
```

### 7. Verificación Final

**Comandos para confirmar configuración correcta:**
```bash
# Verificar branches y tracking
git branch -vv

# Verificar remotos
git remote -v

# Verificar estado
git status

# Verificar conexión remota
git ls-remote origin HEAD
```

## Comandos de Scripts Utilizados

### Script Completo de Migración
```bash
#!/usr/bin/env bash
# migrate_git_account.sh
set -euo pipefail

REPO_DIR="$1"
OLD_ACCOUNT="$2"
NEW_ACCOUNT="$3"
REPO_NAME="$4"

cd "$REPO_DIR"

echo "=== Migrating git repository ==="
echo "From: $OLD_ACCOUNT"
echo "To: $NEW_ACCOUNT/$REPO_NAME"

# 1. Remove old remotes
echo "--- Removing old remotes ---"
old_remotes=$(git remote -v | awk -v account="$OLD_ACCOUNT" '$2 ~ account {print $1}' | sort -u)
for remote in $old_remotes; do
    git remote remove "$remote" && echo "Removed: $remote"
done

# 2. Set new origin
echo "--- Setting new origin ---"
if git remote get-url origin >/dev/null 2>&1; then
    git remote set-url origin "git@github.com:$NEW_ACCOUNT/$REPO_NAME.git"
else
    git remote add origin "git@github.com:$NEW_ACCOUNT/$REPO_NAME.git"
fi

# 3. Rename master to main if needed
echo "--- Renaming master to main ---"
if git show-ref --verify --quiet refs/heads/master; then
    git branch -m master main
fi

# 4. Push and set upstream
echo "--- Pushing to new remote ---"
git push -u origin main

echo "=== Migration completed ==="
git remote -v
git branch -vv
```

## Casos de Uso Cubiertos

### Caso 1: Desconexión Completa de Cuenta Anterior
- Eliminar todos los remotos que apunten a cuenta no deseada
- Preservar remotos archivados si se desea (renombrar con timestamp)
- Limpiar referencias en `.git/config`

### Caso 2: Repo Local Existente + Repo GitHub Privado Nuevo
- Crear repo privado en GitHub primero (manual o con `gh repo create`)
- Configurar remoto local para apuntar al nuevo repo
- Manejar autenticación para repo privado
- Push inicial con upstream

### Caso 3: Migración Master → Main
- Renombrar rama local
- Actualizar tracking en `.git/config`
- Sincronizar con rama `main` remota

### Caso 4: Múltiples Cuentas/Remotos
- Archival de remotos antiguos con timestamp
- Configuración de remote principal (`origin`)
- Preservación de histórico para rollback si es necesario

## Problemas Comunes y Soluciones

### "Repository not found" en Repo Privado
**Causa**: Falta de autenticación apropiada para repo privado
**Solución**: Usar SSH o configurar PAT/gh CLI

### "Remote already exists"
**Causa**: Intento de crear remote que ya existe
**Solución**: Usar `git remote set-url` en lugar de `git remote add`

### Branch tracking a remoto inexistente
**Causa**: Rama local configurada para seguir remote eliminado
**Solución**: Actualizar `.git/config` o usar `git branch --set-upstream-to=origin/main`

### Push rechazado por divergent histories
**Causa**: Historia local difiere de remota
**Solución**: `git pull --rebase` o `git push --force-with-lease` (con precaución)

## Comandos de Rollback

### Restaurar Remote Archivado
```bash
# Si archivaste como origin_archived_TIMESTAMP
git remote rename origin_archived_20250824-102744 origin
git branch --set-upstream-to=origin/master main  # o branch apropiada
```

### Restaurar Branch Master
```bash
git branch -m main master
# Actualizar .git/config manualmente si es necesario
```

Esta guía cubre los escenarios más comunes de migración entre cuentas Git y resolución de problemas de autenticación en repositorios privados.
