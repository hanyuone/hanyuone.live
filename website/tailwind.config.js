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
                        "blockquote p:first-of-type::before": false,
                        "blockquote p:first-of-type::after": false,
                        blockquote: {
                            fontWeight: false,
                            fontStyle: false,
                        }
                    },
                },
            },
        },
    },
    variants: {},
    plugins: [
        require('@tailwindcss/typography'),
    ],
    safelist: [
        "hover:bg-purple",
    ],
};
