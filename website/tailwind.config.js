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
            animation: {
                blink: "blink 1s infinite",
            },
            colors: {
                primary: {
                    light: "#637075",
                    dark: "#0E1428"
                },
                secondary: "#7B9E89",
            },
            keyframes: {
                blink: {
                    "0%, 100%": {
                        "background-color": "transparent",
                    },
                    "50%": {
                        "background-color": "white",
                    },
                },
            },
        },
    },
    variants: {},
    plugins: [],
};
