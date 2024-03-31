const {defineConfig} = require('@vue/cli-service')
const webpack = require('webpack')

module.exports = defineConfig({
  transpileDependencies: true,
  css: {
    loaderOptions: {
      sass: {
        implementation: require('sass'),
      }
    }
  },
  configureWebpack: {
    resolve: {
      alias: {
        'stream': 'stream-browserify'
      },
      fallback: {
        "crypto": require.resolve("crypto-browserify")
      }
    },
    plugins: [
      new webpack.ProvidePlugin({
        //process: 'process/browser', TODO check why this is bugging, maybe it has been deprecated
        Buffer: ['buffer', 'Buffer']
      })
    ]
  }
})
