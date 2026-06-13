# SDKWork Forum PC

PC application for SDKWork Forum.

## Architecture

This application follows `APP_PC_ARCHITECTURE_SPEC.md` standards.

### Package Structure

- `sdkwork-forum-pc-core` - Core runtime, SDK clients, session management
- `sdkwork-forum-pc-commons` - Shared UI components and utilities
- `sdkwork-forum-pc-shell` - PC navigation and shell components
- `sdkwork-forum-pc-discussion` - Forum discussion feature package

### Development

```bash
# Install dependencies
pnpm install

# Start development server
pnpm dev

# Build for production
pnpm build
```

### Configuration

Runtime configuration is in `config/browser/`. See `CONFIG_SPEC.md` for details.
