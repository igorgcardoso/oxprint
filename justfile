build:
    cd frontend && pnpm build
    cp -r frontend/dist/* backend/static/
    cd backend && cargo build --release
