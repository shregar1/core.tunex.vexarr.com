# Security Policy

TuneX Core defines contracts that future modules will use for plugins,
serialization, distributed coordination, and runtime orchestration. Security
issues in these contracts can affect downstream modules.

## Reporting

Do not open a public issue for a suspected vulnerability. Contact the maintainers
privately with:

- affected version or commit
- impact summary
- reproduction steps
- suggested mitigation, if known

## Security Scope

Security-sensitive areas include:

- plugin loading contracts
- serialization and wire protocol contracts
- distributed coordination contracts
- error metadata and diagnostic disclosure
- future WASM, FFI, and remote execution boundaries

## Current Status

v0.1 defines contracts only. It does not execute untrusted plugins, open network
listeners, run GPU kernels, or implement distributed transport.
