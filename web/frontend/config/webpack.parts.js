const path = require('path')
const preprocess = require('svelte-preprocess')
const { ESBuildPlugin } = require('esbuild-loader')
const { WebpackPluginServe } = require('webpack-plugin-serve')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const WebpackBar = require('webpackbar')
const CssMinimizerPlugin = require('css-minimizer-webpack-plugin')

exports.devServer = () => ({
  watch: true,
  plugins: [
    new WebpackPluginServe({
      port: 3000,
      static: path.resolve(process.cwd(), 'dist'),
      historyFallback: true
    })
  ]
})

exports.generateSourceMaps = ({ type }) => ({ devtool: type })

exports.loadImages = ({ limit } = {}) => ({
  module: {
    rules: [
      {
        test: /\.(png|jpg|gif|webp)$/,
        type: 'asset',
        parser: { dataUrlCondition: { maxSize: limit } }
      }
    ]
  }
})

exports.optimize = () => ({
  optimization: {
    minimize: true,
    splitChunks: {
      chunks: 'all'
    },
    runtimeChunk: { name: 'runtime' },
    minimizer: [`...`, new CssMinimizerPlugin()]
  }
})

exports.typescript = () => ({
  module: { rules: [{ test: /\.ts$/, use: 'ts-loader', exclude: /node_modules/ }] }
})

exports.loadSvg = () => ({
  module: { rules: [{ test: /\.svg$/, type: 'asset' }] }
})

exports.postcss = () => ({
  loader: 'postcss-loader'
})

exports.svelte = mode => {
  const prod = mode === 'production'

  return {
    resolve: {
      alias: {
        svelte: path.dirname(require.resolve('svelte/package.json'))
      },
      extensions: ['.mjs', '.js', '.svelte', '.ts'],
      mainFields: ['svelte', 'browser', 'module', 'main']
    },
    module: {
      rules: [
        {
          test: /\.svelte$/,
          use: {
            loader: 'svelte-loader',
            options: {
              compilerOptions: {
                dev: !prod
              },
              emitCss: prod,
              hotReload: !prod,
              preprocess: preprocess({
                postcss: true,
                typescript: true
              })
            }
          }
        },
        {
          test: /node_modules\/svelte\/.*\.mjs$/,
          resolve: {
            fullySpecified: false
          }
        }
      ]
    }
  }
}

exports.esbuild = () => {
  return {
    module: {
      rules: [
        {
          test: /\.js$/,
          loader: 'esbuild-loader',
          options: {
            target: 'es2015'
          }
        },
        {
          test: /\.ts$/,
          loader: 'esbuild-loader',
          options: {
            loader: 'ts',
            target: 'es2015'
          }
        }
      ]
    },
    plugins: [new ESBuildPlugin()]
  }
}

exports.cleanDist = () => ({
  plugins: [new CleanWebpackPlugin()]
})

exports.useWebpackBar = () => ({
  plugins: [new WebpackBar()]
})

exports.useDotenv = () => ({
  plugins: [new DotenvPlugin()]
})