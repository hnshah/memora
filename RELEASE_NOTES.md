# Memora v0.9.0 Release Notes

**Release Date:** May 25, 2026

## Highlights
- Real local embeddings with `nomic-embed-text` via Ollama
- Full LanceDB vector search with HNSW index
- Browser extension supporting 15+ sites (ChatGPT, Claude, Gemini, Google, YouTube, Reddit, Linear, Notion, Cursor, Perplexity, Windsurf, Grok, DeepSeek, Qwen)
- Desktop app with memory graph, timeline, and live context injection
- Export tools (JSON, Markdown, Obsidian vault)
- Settings panel with privacy controls and model selection
- iCloud-ready storage with basic conflict handling
- GitHub Actions CI (check + build)

## New Features
- Grammarly-style live underlining + 1-click injection
- Interactive memory graph and timeline view
- Native desktop app capture (Claude Desktop, Cursor, etc.)
- Batch embedding support
- Exponential backoff for Ollama

## Bug Fixes & Improvements
- Proper error sanitization
- Improved site parsers
- Better iCloud path handling

## Known Limitations
- Full cross-platform packaging (Windows/Linux) in progress
- Some advanced vector search optimizations pending

## Installation
See README.md for build and install instructions.

**Thank you to everyone who helped build Memora!**