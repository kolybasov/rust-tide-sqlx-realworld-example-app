module.exports = {
    future: {
        removeDeprecatedGapUtilities: true,
        purgeLayersByDefault: true,
    },
    purge: ['./web/templates/**/*.html', './web/templates/**/*.svg'],
    theme: {
        extend: {},
    },
    variants: {
        margin: ['responsive', 'first'],
        padding: ['responsive', 'first'],
        opacity: ['disabled'],
        cursor: ['disabled'],
    },
    plugins: [],
};
