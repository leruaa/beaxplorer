module.exports = {
    mount: {
      src: '/dist',
    },
    plugins: [
      '@snowpack/plugin-svelte',
      '@snowpack/plugin-typescript',
      '@snowpack/plugin-postcss'
    ]
  }