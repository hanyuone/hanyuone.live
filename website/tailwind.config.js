module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
    ],
    theme: {
        extend: {
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
};
