import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const jsonPath = path.join(__dirname, "environment.allowed-hosts.json");
const rawJson = JSON.parse(fs.readFileSync(jsonPath, "utf-8"));
const ALLOWED_HOSTS = rawJson.allowedHosts;

if (!Array.isArray(ALLOWED_HOSTS) || ALLOWED_HOSTS.length === 0) {
    console.error("ALLOWED_HOSTS is empty");
    process.exit(1);
}

const angularJsonPath = path.join(__dirname, "../angular.json");
const angularConfig = JSON.parse(fs.readFileSync(angularJsonPath, "utf-8"));
const serveConfig =
    angularConfig.projects.frontend.architect.serve.configurations;

Object.keys(serveConfig).forEach((configName) => {
    serveConfig[configName].allowedHosts = [...ALLOWED_HOSTS];
});

fs.writeFileSync(angularJsonPath, JSON.stringify(angularConfig, null, 2));
