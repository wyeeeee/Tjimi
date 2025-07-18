# v0.10.9

## Features

- **Usage Tracking**: Added comprehensive usage tracking for Claude API requests and responses with token counting
- **Enhanced Response Handling**: Improved response handling by adding usage information and token counting capabilities

## Fixes

- **Configuration Management**: Improved config saving with async handling and enhanced error logging
- **Cache System**: Simplified cache system by removing TTL handling for Claude Code prompts and updated to use time-to-idle instead of time-to-live
- **Dependencies**: Updated async-compression to version 0.4.27 and crc32fast to version 1.5.0, removed unused scopeguard and figlet-rs dependencies
- **Error Handling**: Enhanced error handling for token requests and improved cookie request handling
- **Web Admin**: Corrected web admin endpoint URL formatting and adjusted import order
- **Serialization**: Added default serialization for credential field in VertexConfig and reordered imports in GeminiState

## Documentation

- **README**: Improved formatting and clarity in README and README_zh files
- **Icons**: Added navigation SVG icon to README and updated SVG icon styling
- **License**: Removed incorrect MIT license reference from README
- **Serialization Docs**: Updated README for serialization and import changes

## Internal Improvements

- Simplified banner generation by removing figlet-rs dependency
- Improved asynchronous handling throughout the codebase
- Enhanced import organization and code structure
