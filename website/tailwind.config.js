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
            height: {
                "fullmain": "calc(100vh - 7rem - 2px)",
                // Screen height minus header/footer and padding
                "main": "calc(100vh - 17rem - 2px)",
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
            margin: {
                // Screen height minus header/footer and padding
                "main": "calc(100vh - 17rem - 2px)",
            },
            typography: {
                DEFAULT: {
                    css: {
                        // Blockquotes
                        "blockquote p:first-of-type::before": false,
                        "blockquote p:first-of-type::after": false,
                        blockquote: {
                            fontWeight: false,
                            fontStyle: false,
                        },
                        // Tables
                        "thead th:first-child": false,
                        "thead th:last-child": false,
                        "tbody td:first-child, tfoot td:first-child": false,
                        "tbody td:last-child, tfoot td:last-child": false,
                        td: {
                            verticalAlign: "top",
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
    safelist: [
        "hover:bg-purple",
    ],
};
