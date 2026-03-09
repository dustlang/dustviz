# Architecture

Current buildable profile:

- single compilation unit: `src/main.ds`
- top-level grammar: `K main { ... }`

The executable models deterministic DustViz pipeline stages inline (input check, graph tokenization, format-select render stub).
