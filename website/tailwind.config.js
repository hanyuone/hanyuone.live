module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    theme: {
        extend: {
            colors: {
                primary: {
                    light: "#637075",
                    dark: "#0E1428"
                },
                secondary: "#7B9E89",
            }
        }
    },
    variants: {},
    plugins: [],
};
