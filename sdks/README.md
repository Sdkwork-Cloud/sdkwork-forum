# Forum SDK Workspace

This directory is the SDKWork SDK generation workspace for the forum capability.

## SDK Families

| Family | Surface | Prefix | Auth | Status |
|--------|---------|--------|------|--------|
| `sdkwork-forum-app-sdk` | app-api | `/app/v3/api` | dual-token | Composed facade implemented |
| `sdkwork-forum-backend-sdk` | backend-api | `/backend/v3/api` | dual-token | Composed facade implemented |
| `sdkwork-forum-sdk` | open-api | `/forum/v3/api` | anonymous | Composed facade implemented |

## Structure

```
sdks/
  sdkwork-forum-app-sdk/
    openapi/                    # sdkgen configs
    composed/src/index.ts       # ForumAppFacade (22 methods)
    generated/server-openapi/   # sdkgen output (awaiting generation)
  sdkwork-forum-backend-sdk/
    openapi/
    composed/src/index.ts       # ForumBackendFacade (30+ methods)
    generated/server-openapi/
  sdkwork-forum-sdk/
    openapi/
    composed/src/index.ts       # ForumOpenFacade (8 methods)
    generated/server-openapi/
  _route-manifests/             # Route manifest JSON files
  _shared/                      # Shared schema fragments
  test/                         # SDK tests
```

## Generation

Generated transport output under each family `generated/server-openapi` is generator-owned and must not be hand-edited. Handwritten composition belongs in `composed/`.

Run canonical sdkgen after authority OpenAPI review:
```bash
../sdkwork-sdk-generator/bin/sdkgen.js --input sdks/sdkwork-forum-app-sdk/openapi/sdkwork-forum-app-api.sdkgen.yaml
```

## Tests

- `tests/sdk/forum-sdk.test.mjs` - Validates sdkgen configs, route manifests, and composed facades
