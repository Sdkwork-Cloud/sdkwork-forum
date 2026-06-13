import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();

function loadJson(file) {
  const path = join(root, file);
  if (!existsSync(path)) return null;
  return JSON.parse(readFileSync(path, "utf8"));
}

function loadYaml(file) {
  const path = join(root, file);
  if (!existsSync(path)) return null;
  return readFileSync(path, "utf8");
}

globalThis.forumTestFixtures = {
  root,
  loadJson,
  loadYaml,
  fileExists: (file) => existsSync(join(root, file)),
  readFile: (file) => readFileSync(join(root, file), "utf8"),
  routeManifests: {
    appApi: loadJson("sdks/_route-manifests/app-api/sdkwork-router-forum-app-api.route-manifest.json"),
    backendApi: loadJson("sdks/_route-manifests/backend-api/sdkwork-router-forum-backend-api.route-manifest.json"),
    openApi: loadJson("sdks/_route-manifests/open-api/sdkwork-router-forum-open-api.route-manifest.json"),
  },
  assemblies: {
    appSdk: loadJson("sdks/sdkwork-forum-app-sdk/.sdkwork-assembly.json"),
    backendSdk: loadJson("sdks/sdkwork-forum-backend-sdk/.sdkwork-assembly.json"),
    openSdk: loadJson("sdks/sdkwork-forum-sdk/.sdkwork-assembly.json"),
  },
  sdkgenConfigs: {
    appApi: loadYaml("sdks/sdkwork-forum-app-sdk/openapi/sdkwork-forum-app-api.sdkgen.yaml"),
    backendApi: loadYaml("sdks/sdkwork-forum-backend-sdk/openapi/sdkwork-forum-backend-api.sdkgen.yaml"),
    openApi: loadYaml("sdks/sdkwork-forum-sdk/openapi/sdkwork-forum-open-api.sdkgen.yaml"),
  },
  schema: loadYaml("specs/forum-database.schema.yaml"),
  apiServer: {
    routeCount: 66,
    surfaces: ["app-api", "backend-api", "open-api"],
    prefixes: {
      "app-api": "/app/v3/api",
      "backend-api": "/backend/v3/api",
      "open-api": "/forum/v3/api",
    },
  },
};

console.log("forum test fixtures loaded");
