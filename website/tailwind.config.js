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
                red: "#FB4934",
                orange: "#FE8019",
                yellow: "#FABD2F",
                green: "#B8BB26",
                cyan: "#8EC07C",
                blue: "#83A598",
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
            typography: {
                DEFAULT: {
                    css: {
                        ".blockquote": {
                            backgroundColor: "#5049457F",
                        },
                        ".callout-note": {
                            backgroundColor: "#83A5987F",
                        },
                        ".callout-tip": {
                            backgroundColor: "#B8BB267F",
                        },
                        ".callout-important": {
                            backgroundColor: "#D3869B7F",
                        },
                        ".callout-warning": {
                            backgroundColor: "#FABD2F7F",
                        },
                        ".callout-caution": {
                            backgroundColor: "#FB49347F",
                        },
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
