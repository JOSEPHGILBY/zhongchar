/** @type {import('tailwindcss').Config} */
module.exports = {
    //darkMode: 'class',
    content: {
        files: ["*.html", "./src/**/*.rs", "../../src/docs/**/*.rs"],
    },
    theme: {
        extend: {},
    },
    corePlugins: {
        preflight: false,
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
}