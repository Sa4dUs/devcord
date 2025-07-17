import js from "@eslint/js";
import typescriptPlugin from "@typescript-eslint/eslint-plugin";
import typescriptParser from "@typescript-eslint/parser";
import angularPlugin from "@angular-eslint/eslint-plugin";
import angularTemplatePlugin from "@angular-eslint/eslint-plugin-template";
import angularTemplateParser from "@angular-eslint/template-parser";
import prettierPlugin from "eslint-plugin-prettier";
import prettierConfig from "eslint-config-prettier";

export default [
    js.configs.recommended,
    {
        ignores: [
            "node_modules/**",
            "dist/**",
            "build/**",
            "**/*.html",
            ".angular/**",
        ],
    },
    {
        files: ["**/*.ts"],
        languageOptions: {
            parser: typescriptParser,
            parserOptions: {
                project: ["./tsconfig.eslint.json"],
                sourceType: "module",
            },
            globals: {
                console: "readonly",
                process: "readonly",
                describe: "readonly",
                it: "readonly",
                expect: "readonly",
                beforeEach: "readonly",
                window: "readonly",
                document: "readonly",
                global: "readonly",
            },
        },
        plugins: {
            "@typescript-eslint": typescriptPlugin,
            "@angular-eslint": angularPlugin,
            prettier: prettierPlugin,
        },
        rules: {
            ...typescriptPlugin.configs.recommended.rules,
            ...prettierConfig.rules,
            "prettier/prettier": "error",
        },
    },

    {
        files: ["**/*.js"],
        languageOptions: {
            globals: {
                console: "readonly",
                process: "readonly",
                module: "writable",
                require: "readonly",
                __dirname: "readonly",
            },
        },
    },

    {
        files: ["**/*.html"],
        languageOptions: {
            parser: angularTemplateParser,
        },
        plugins: {
            "@angular-eslint/template": angularTemplatePlugin,
        },
        rules: {},
    },

    {
        files: ["**/*.{js,ts,tsx,css,scss,md,yml,yaml,graphql,mdx}"],
        plugins: {
            prettier: prettierPlugin,
        },
        rules: {
            ...prettierConfig.rules,
            "prettier/prettier": "error",
        },
    },
];
