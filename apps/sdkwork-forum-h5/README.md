# SDKWork Forum H5

H5 mobile application for SDKWork Forum.

## Architecture

This application follows `APP_H5_ARCHITECTURE_SPEC.md` standards.

### Package Structure

- `sdkwork-forum-h5-core` - Core runtime, SDK clients, session management
- `sdkwork-forum-h5-commons` - Shared UI components and utilities
- `sdkwork-forum-h5-shell` - Mobile navigation and shell components
- `sdkwork-forum-h5-discussion` - Forum discussion feature package

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
