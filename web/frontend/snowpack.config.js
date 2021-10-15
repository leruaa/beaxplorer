module.exports = {
  mount: {
    src: '/',
  },
  optimize: {
    bundle: true,
    entrypoints:
      [
        'src/js/main.ts'
      ]
  },
  buildOptions: {
    out: "dist",
    watch: true
  },
  plugins: [
    '@snowpack/plugin-typescript',
    '@snowpack/plugin-postcss'
  ]
}