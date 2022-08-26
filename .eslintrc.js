module.exports = {
    parser: "@typescript-eslint/parser", // add the TypeScript parser
    plugins: [
        "svelte3",
        "@typescript-eslint", // add the TypeScript plugin
    ],
    overrides: [
        // this stays the same
        {
            /* files: ['*.svelte'],*/
            files: [
                "**/*.svelte",
            ] /* <= 修正（こうしないと階層が深くなると対応できない） */,
            processor: "svelte3/svelte3",
        },
    ],
    rules: {
        // ...
    },
    settings: {
        "svelte3/typescript": require("typescript"), // pass the TypeScript package to the Svelte plugin
        // ...
    },
    env: {
        browser: true,
        es2021: true,
    },
    extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
    // "parser": "@typescript-eslint/parser",
    parserOptions: {
        ecmaVersion: 12,
        sourceType: "module",
    },
    // "plugins": [
    //     "@typescript-eslint"
    // ],
    // "rules": {
    // }
};
