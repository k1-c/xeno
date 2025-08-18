<div align="center">

# ⚡ Xeno

*A lightning-fast, portable web framework for Rust*

[![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-experimental-yellow.svg)](https://github.com/k1-c/xeno)

**Tiny core • Universal adapters • Web Standards**

*Inspired by Hono*

</div>

---

> ⚠️ **Experimental Status** - This repository is currently in experimental development. **Do not use in production environments.**

Built on **Web Standard Request/Response/Fetch API primitives** for maximum compatibility across modern web runtimes. Deploy the same `Request → Response` core logic across multiple hosting environments through lightweight adapters.

## Key Features

- 🌐 **Web Standards Compliant** - Built on Request/Response/Fetch API primitives
- ⚡ **Universal Deployment** - Same code runs on Cloudflare Workers, hyper servers, and more
- 🪶 **Minimal Dependencies** - Tiny core with essential features only
- 🔒 **Type Safe** - Full TypeScript client generation from OpenAPI specs
- 🚀 **Performance Focused** - Zero-cost abstractions and minimal runtime overhead

## Documentation

- **[Design Doc](docs/DESIGN.md)** - Comprehensive technical design and architecture
- **[Development Guide](docs/DEVELOPMENT.md)** - Quick setup and development commands

## Quick Start

```bash
git clone https://github.com/k1-c/xeno
cd xeno
cargo build
```
