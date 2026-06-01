# Integrity Protocol SDKs

This directory contains the legacy SDKs utilized by the Integrity Protocol to interface with existing agent harnesses (e.g., AutoGPT, LangChain, and other Python/Node.js frameworks).

Rather than rewriting these integration layers, we have fully ported the legacy SDKs to ensure **100% backwards compatibility** with your current agent harnesses.

## Available SDKs

1. **[Node.js SDK](./nodejs/README.md)**
   Provides standard JavaScript/TypeScript classes to broadcast execution telemetry to the Oracle and interface with the L2 Smart Contracts via `ethers`.

2. **[Python SDK](./python/README.md)**
   Provides robust Python middleware and CLI tools (`integrity_cli.py`, `integrity_middleware.py`) designed to wrap Python-based autonomous agent execution loops seamlessly.
