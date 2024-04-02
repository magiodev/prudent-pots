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
        stream: 'stream-browserify',
      },
      fallback: {
        crypto: require.resolve("crypto-browserify")
      }
    },
    plugins: [
      new webpack.ProvidePlugin({
        Buffer: ['buffer', 'Buffer'],
      }),
    ],
  },
});
