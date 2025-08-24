#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib_bitacora.sh"

echo "🔧 Bitácora Git Integration Setup"
echo ""

# Detect git remote type
git_type="$(detect_git_remote_type)"
repo_info="$(extract_repo_info)"

echo "📡 Repository Info:"
echo "   Remote type: $git_type"
echo "   Repository: $repo_info"
echo ""

case "$git_type" in
    "github")
        echo "🐙 GitHub detected"
        if command -v gh >/dev/null 2>&1; then
            echo "   ✅ GitHub CLI (gh) is installed"
            if gh_authenticated; then
                echo "   ✅ GitHub CLI is authenticated"
                echo "   🚀 Auto-PR functionality ready!"
            else
                echo "   ⚠️  GitHub CLI not authenticated"
                echo "   💡 Interactive login: gh auth login"
                echo "   💡 Or use a token: export GITHUB_TOKEN=ghp_xxx... (store in your shell profile)"
            fi
        else
            echo "   ⚠️  GitHub CLI (gh) not installed"
            echo "   💡 Install with:"
            echo "      sudo apt update && sudo apt install gh"
            echo "   💡 Or visit: https://cli.github.com/"
        fi
        ;;
    "gitlab")
        echo "🦊 GitLab detected"
        if command -v glab >/dev/null 2>&1; then
            echo "   ✅ GitLab CLI (glab) is installed"
            if glab auth status >/dev/null 2>&1; then
                echo "   ✅ GitLab CLI is authenticated"
                echo "   🚀 Auto-MR functionality ready!"
            else
                echo "   ⚠️  GitLab CLI not authenticated"
                echo "   💡 Run: glab auth login"
            fi
        else
            echo "   ⚠️  GitLab CLI (glab) not installed"
            echo "   💡 Install with:"
            echo "      sudo apt update && sudo apt install glab"
            echo "   💡 Or visit: https://gitlab.com/gitlab-org/cli"
        fi
        ;;
    *)
        echo "❓ Unknown or unsupported git remote"
        echo "   Auto-PR functionality will be limited"
        ;;
esac

echo ""
echo "⚙️  Configuration Options:"
echo ""
echo "To disable auto-commit (commits after each ACTION):"
echo "   export BITACORA_NO_AUTO_COMMIT=1"
echo ""
echo "To disable auto-PR (pull requests on END):"
echo "   export BITACORA_NO_AUTO_PR=1"
echo ""
echo "To disable both:"
echo "   export BITACORA_NO_AUTO_COMMIT=1 BITACORA_NO_AUTO_PR=1"
echo ""
echo "Add these to your ~/.bashrc or ~/.zshrc to make them permanent."
echo ""

# Check current settings
if [ "${BITACORA_NO_AUTO_COMMIT:-}" = "1" ]; then
    echo "🔒 Auto-commit is currently DISABLED"
else
    echo "✅ Auto-commit is currently ENABLED"
fi

if [ "${BITACORA_NO_AUTO_PR:-}" = "1" ]; then
    echo "🔒 Auto-PR is currently DISABLED"
else
    echo "✅ Auto-PR is currently ENABLED"
fi

echo ""
echo "✨ Git integration setup complete!"
echo "   Use ACTION commands to trigger auto-commits"
echo "   Use END command to trigger auto-PR creation"

echo ""
echo "Example: create a PR draft for this branch (locally):"
echo "gh pr create --base main --head \\$(git rev-parse --abbrev-ref HEAD) --title \"chore(bitacora): migrate binnacle -> bitacora\" --body \"See branch changes.\" --draft"
