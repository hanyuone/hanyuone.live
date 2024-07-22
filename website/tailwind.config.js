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
                black: {
                    dark: "#32302F",
                    light: "#504945",
                },
                red: "#EA6962",
                orange: "#E78A4E",
                yellow: "#D8A657",
                green: "#A9B665",
                blue: "#7DAEA3",
                purple: "#D3869B",
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
    plugins: [
        require('@tailwindcss/typography'),
    ],
};
