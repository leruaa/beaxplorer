module.exports = {
  mount: {
    src: '/',
  },
  buildOptions: {
    out: "dist",
    metaUrlPath: "_lib",
    watch: true
  },
  plugins: [
    '@snowpack/plugin-typescript',
    '@snowpack/plugin-postcss'
  ]
}