import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const failures = [];

function fail(message) {
  failures.push(message);
}

function readJson(file) {
  return JSON.parse(readFileSync(join(root, file), "utf8"));
}

function read(file) {
  return readFileSync(join(root, file), "utf8");
}

console.log("Running forum SDK tests...");

const sdkgenConfigs = [
  ["sdks/sdkwork-forum-app-sdk/openapi/sdkwork-forum-app-api.sdkgen.yaml", "sdkwork-forum-app-api", "sdkwork-forum-app-sdk", "/app/v3/api", "app-api"],
  ["sdks/sdkwork-forum-backend-sdk/openapi/sdkwork-forum-backend-api.sdkgen.yaml", "sdkwork-forum-backend-api", "sdkwork-forum-backend-sdk", "/backend/v3/api", "backend-api"],
  ["sdks/sdkwork-forum-sdk/openapi/sdkwork-forum-open-api.sdkgen.yaml", "sdkwork-forum-open-api", "sdkwork-forum-sdk", "/forum/v3/api", "open-api"],
];

for (const [file, authority, family, prefix, surface] of sdkgenConfigs) {
  if (!existsSync(join(root, file))) {
    fail(`missing sdkgen config: ${file}`);
    continue;
  }
  const text = read(file);
  if (!text.includes(`sdkFamily: ${family}`)) fail(`${file} sdkFamily mismatch`);
  if (!text.includes(`prefix: ${prefix}`)) fail(`${file} prefix mismatch`);
  if (!text.includes(`surface: ${surface}`)) fail(`${file} surface mismatch`);
  if (!text.includes("standardProfile: sdkwork-v3")) fail(`${file} missing standardProfile`);
  if (!text.includes("schemaVersion: 1")) fail(`${file} missing schemaVersion`);
}

const manifestFiles = [
  "sdks/_route-manifests/app-api/sdkwork-routes-forum-app-api.route-manifest.json",
  "sdks/_route-manifests/backend-api/sdkwork-routes-forum-backend-api.route-manifest.json",
  "sdks/_route-manifests/open-api/sdkwork-routes-forum-open-api.route-manifest.json",
];

for (const file of manifestFiles) {
  if (!existsSync(join(root, file))) continue;
  const json = readJson(file);
  if (!Array.isArray(json.routes)) fail(`${file} routes must be array`);
  for (const route of json.routes) {
    if (!route.operationId) fail(`${file} route missing operationId`);
    if (!route.method) fail(`${file} route missing method`);
    if (!route.path) fail(`${file} route missing path`);
    if (!route.auth?.mode) fail(`${file} route missing auth.mode`);
  }
}

const assemblyFiles = [
  "sdks/sdkwork-forum-app-sdk/.sdkwork-assembly.json",
  "sdks/sdkwork-forum-backend-sdk/.sdkwork-assembly.json",
  "sdks/sdkwork-forum-sdk/.sdkwork-assembly.json",
];

for (const file of assemblyFiles) {
  if (!existsSync(join(root, file))) continue;
  const json = readJson(file);
  if (!json.sdkOwner) fail(`${file} missing sdkOwner`);
  if (!json.apiAuthority) fail(`${file} missing apiAuthority`);
  if (!json.sdkFamily) fail(`${file} missing sdkFamily`);
  if (!json.discoverySurface) fail(`${file} missing discoverySurface`);
  if (!Array.isArray(json.sdkDependencies)) fail(`${file} sdkDependencies must be array`);
}

const composedFacades = [
  "sdks/sdkwork-forum-app-sdk/composed/src/index.ts",
  "sdks/sdkwork-forum-backend-sdk/composed/src/index.ts",
  "sdks/sdkwork-forum-sdk/composed/src/index.ts",
];

for (const file of composedFacades) {
  if (!existsSync(join(root, file))) {
    fail(`missing composed facade: ${file}`);
    continue;
  }
  const text = read(file);
  if (text.includes("throw new Error(\"TODO")) fail(`${file} contains TODO stub`);
  if (!text.includes("export class")) fail(`${file} missing exported class`);
  if (!text.includes("constructor")) fail(`${file} missing constructor`);
}

if (failures.length > 0) {
  console.error("SDK test failures:");
  for (const f of failures) console.error(`  - ${f}`);
  process.exit(1);
}

console.log(`forum SDK tests passed (${sdkgenConfigs.length} sdkgen configs, ${manifestFiles.length} manifests, ${composedFacades.length} facades verified)`);
