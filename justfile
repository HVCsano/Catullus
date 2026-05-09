default:
    just dev

set windows-shell := ["powershell"]

dev:
    bun tauri dev

publish:
    git checkout release
    git rebase main
    git push
    git checkout main