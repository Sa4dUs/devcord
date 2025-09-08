import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import dotenv from "dotenv";

dotenv.config();

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const allowedHostsEnv = process.env.ALLOWED_HOSTS;

if (!allowedHostsEnv) {
    console.error("ALLOWED_HOSTS env is empty");
    process.exit(1);
}

const ALLOWED_HOSTS = allowedHostsEnv.split(",").map((h) => h.trim());

const angularJsonPath = path.join(__dirname, "angular.json");
const angularConfig = JSON.parse(fs.readFileSync(angularJsonPath, "utf-8"));
const serveConfig =
    angularConfig.projects.frontend.architect.serve.configurations;

Object.keys(serveConfig).forEach((configName) => {
    serveConfig[configName].allowedHosts = [...ALLOWED_HOSTS];
});

fs.writeFileSync(angularJsonPath, JSON.stringify(angularConfig, null, 2));
