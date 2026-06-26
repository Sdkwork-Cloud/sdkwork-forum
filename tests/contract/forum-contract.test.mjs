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

console.log("Running forum contract tests...");

const requiredContractFiles = [
  "specs/forum-database.schema.yaml",
  "apis/app-api/forum/openapi.yaml",
  "apis/backend-api/forum/openapi.yaml",
  "apis/open-api/forum/openapi.yaml",
];

for (const file of requiredContractFiles) {
  if (!existsSync(join(root, file))) {
    fail(`missing required contract file: ${file}`);
  }
}

const forbiddenTerm = /\bthreads?\b/i;
const contractFiles = [
  "specs/forum-database.schema.yaml",
  "apis/app-api/forum/openapi.yaml",
  "apis/backend-api/forum/openapi.yaml",
  "apis/open-api/forum/openapi.yaml",
];

for (const file of contractFiles) {
  if (!existsSync(join(root, file))) continue;
  const text = read(file);
  if (forbiddenTerm.test(text)) {
    fail(`forbidden term "thread" found in contract: ${file}`);
  }
}

const openApiFile = "apis/open-api/forum/openapi.yaml";
if (existsSync(join(root, openApiFile))) {
  const openApi = read(openApiFile);
  if (/Access-Token|AuthToken|X-Tenant|X-Organization/i.test(openApi)) {
    fail("open API must not declare SDKWork dual-token headers");
  }
  if (!/security:\s*\[\]/.test(openApi)) {
    fail("open API must declare security: [] on public operations");
  }
}

const manifestSpecs = [
  ["sdks/_route-manifests/app-api/sdkwork-routes-forum-app-api.route-manifest.json", "app-api", "/app/v3/api"],
  ["sdks/_route-manifests/backend-api/sdkwork-routes-forum-backend-api.route-manifest.json", "backend-api", "/backend/v3/api"],
  ["sdks/_route-manifests/open-api/sdkwork-routes-forum-open-api.route-manifest.json", "open-api", "/forum/v3/api"],
];

for (const [file, surface, prefix] of manifestSpecs) {
  if (!existsSync(join(root, file))) {
    fail(`missing route manifest: ${file}`);
    continue;
  }
  const json = readJson(file);
  if (json.kind !== "sdkwork.route.manifest") fail(`${file} kind must be sdkwork.route.manifest`);
  if (json.surface !== surface) fail(`${file} surface mismatch: expected ${surface}`);
  if (json.domain !== "communication") fail(`${file} domain must be communication`);
  if (json.capability !== "forum") fail(`${file} capability must be forum`);
  if (json.prefix !== prefix) fail(`${file} prefix mismatch: expected ${prefix}`);
  if (!Array.isArray(json.routes) || json.routes.length === 0) fail(`${file} routes must be non-empty`);

  const seen = new Set();
  for (const route of json.routes ?? []) {
    if (!route.path?.startsWith(prefix)) fail(`${file} route path outside prefix: ${route.path}`);
    const key = `${route.method} ${route.path}`;
    if (seen.has(key)) fail(`${file} duplicate route: ${key}`);
    seen.add(key);
    if (!route.operationId || !/^[a-z][A-Za-z0-9]*(\.[a-z][A-Za-z0-9]*)+$/.test(route.operationId)) {
      fail(`${file} invalid operationId: ${route.operationId}`);
    }
    if (surface === "open-api" && route.auth?.mode !== "public") {
      fail(`${file} open route must use public auth: ${route.operationId}`);
    }
    if ((surface === "app-api" || surface === "backend-api") && route.auth?.mode !== "dual-token") {
      fail(`${file} protected route must use dual-token auth: ${route.operationId}`);
    }
  }
}

const assemblySpecs = [
  ["sdks/sdkwork-forum-app-sdk/.sdkwork-assembly.json", "sdkwork-forum-app-api", "sdkwork-forum-app-sdk", "/app/v3/api"],
  ["sdks/sdkwork-forum-backend-sdk/.sdkwork-assembly.json", "sdkwork-forum-backend-api", "sdkwork-forum-backend-sdk", "/backend/v3/api"],
  ["sdks/sdkwork-forum-sdk/.sdkwork-assembly.json", "sdkwork-forum-open-api", "sdkwork-forum-sdk", "/forum/v3/api"],
];

for (const [file, authority, family, prefix] of assemblySpecs) {
  if (!existsSync(join(root, file))) continue;
  const json = readJson(file);
  if (json.sdkOwner !== "sdkwork-forum") fail(`${file} sdkOwner mismatch`);
  if (json.apiAuthority !== authority) fail(`${file} apiAuthority mismatch`);
  if (json.sdkFamily !== family) fail(`${file} sdkFamily mismatch`);
}

if (failures.length > 0) {
  console.error("Contract test failures:");
  for (const f of failures) console.error(`  - ${f}`);
  process.exit(1);
}

console.log(`forum contract tests passed (${manifestSpecs.length} manifests, ${assemblySpecs.length} assemblies verified)`);
