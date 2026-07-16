import { copyFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

const root = process.cwd();

const materializations = [
  {
    source: "apis/app-api/forum/openapi.yaml",
    target: "sdks/sdkwork-forum-app-sdk/openapi/sdkwork-forum-app-api.openapi.yaml"
  },
  {
    source: "apis/backend-api/forum/openapi.yaml",
    target: "sdks/sdkwork-forum-backend-sdk/openapi/sdkwork-forum-backend-api.openapi.yaml"
  },
  {
    source: "apis/open-api/forum/openapi.yaml",
    target: "sdks/sdkwork-forum-sdk/openapi/sdkwork-forum-open-api.openapi.yaml"
  }
];

const routeManifestMaterializations = [
  {
    source: "sdks/_route-manifests/app-api/sdkwork-router-forum-app-api.route-manifest.json",
    target: "sdks/_route-manifests/app-api/sdkwork-routes-forum-app-api.route-manifest.json"
  },
  {
    source: "sdks/_route-manifests/backend-api/sdkwork-router-forum-backend-api.route-manifest.json",
    target: "sdks/_route-manifests/backend-api/sdkwork-routes-forum-backend-api.route-manifest.json"
  },
  {
    source: "sdks/_route-manifests/open-api/sdkwork-router-forum-open-api.route-manifest.json",
    target: "sdks/_route-manifests/open-api/sdkwork-routes-forum-open-api.route-manifest.json"
  }
];

for (const item of [...materializations, ...routeManifestMaterializations]) {
  const target = join(root, item.target);
  mkdirSync(dirname(target), { recursive: true });
  copyFileSync(join(root, item.source), target);
  console.log(`${item.source} -> ${item.target}`);
}

console.log("forum OpenAPI materialization complete");
