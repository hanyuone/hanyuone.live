module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
        "./src/**/*.html",
        "./src/**/*.css",
    ],
    theme: {
        extend: {
            animation: {
                blink: "blink 1s infinite",
            },
            colors: {
                black: "#32302F",
                gray: "#504945",
                red: "#EA6962",
                orange: "#E78A4E",
                yellow: "#D8A657",
                green: "#A9B665",
                cyan: "#89B482",
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
            typography: (theme) => ({
                DEFAULT: {
                    css: {
                        ".callout-blockquote": {
                            backgroundColor: theme("colors.gray"),
                        },
                        ".callout-info": {
                            backgroundColor: theme("colors.blue"),
                        },
                        ".callout-tip": {
                            backgroundColor: theme("colors.green"),
                        },
                        ".callout-important": {
                            backgroundColor: theme("colors.cyan"),
                        },
                        ".callout-warning": {
                            backgroundColor: theme("colors.orange"),
                        },
                        ".callout-caution": {
                            backgroundColor: theme("colors.yellow"),
                        },
                    }
                }
            }),
        },
    },
    variants: {},
    plugins: [
        require('@tailwindcss/typography'),
    ],
};
