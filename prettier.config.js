/**
 * @type {import('prettier').Config}
 */
const config = {
    semi: true,
    singleQuote: false,
    jsxSingleQuote: false,
    trailingComma: "all",
    printWidth: 100,
    tabWidth: 4,
    plugins: ["prettier-plugin-organize-imports", "prettier-plugin-tailwindcss"],
    tailwindFunctions: ["clsx", "cn"],
};

export default config;
