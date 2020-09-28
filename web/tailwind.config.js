module.exports = {
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
  },
  purge: ["./templates/**/*.html"],
  theme: {
    extend: {},
  },
  variants: {
    margin: ['responsive', 'first'],
    padding: ['responsive', 'first']
  },
  plugins: [],
}
