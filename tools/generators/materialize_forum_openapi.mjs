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

for (const item of materializations) {
  const target = join(root, item.target);
  mkdirSync(dirname(target), { recursive: true });
  copyFileSync(join(root, item.source), target);
  console.log(`${item.source} -> ${item.target}`);
}

console.log("forum OpenAPI materialization complete");
