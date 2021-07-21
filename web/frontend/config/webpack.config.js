const path = require('path')
const { merge } = require('webpack-merge')
const parts = require('./webpack.parts')
const { mode } = require('webpack-nano/argv')

const common = merge([
  {
    entry: {
      "css/main": './src/css/main.css'
    },
    output: {
      clean: true,
    }
  },
  parts.extractCss(),
  parts.cleanDist(),
  parts.useWebpackBar()
])

const development = merge([
  { target: 'web' },
  parts.generateSourceMaps({ type: 'eval-source-map' }),
  parts.esbuild(),
  parts.devServer()
])

const production = merge(
  [
    parts.typescript(),
    parts.optimize(),
  ].filter(Boolean)
)

const getConfig = mode => {
  switch (mode) {
    case 'production':
      return merge(common, production, { mode })
    case 'development':
      return merge(common, development, { mode })
    default:
      throw new Error(`Unknown mode, ${mode}`)
  }
}

module.exports = getConfig(mode)