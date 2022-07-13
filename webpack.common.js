const path = require('path');
const Dotenv = require('dotenv-webpack');
const MiniCssExtractPlugin = require('mini-css-extract-plugin')

module.exports = {
  entry: './frontend/index.js',
  output: {
    path: path.resolve(__dirname, 'static/dist'),
    filename: 'bundle.js',
  },
  module: {
    rules: [
      // CSS, PostCSS, and Sass
      {
        test: /\.(scss|css)$/,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: "css-loader",
            options: {
              importLoaders: 2,
              sourceMap: true,
              url: false,
            }
          },
          {
            loader: 'postcss-loader',
            options: {
              postcssOptions: {
                plugins: [
                  function () {
                      return [ require('autoprefixer') ];
                  }
                ]
              }
            }
          },
          'sass-loader'
        ],
      },
      {
          test: /\.woff($|\?)|\.woff2($|\?)|\.ttf($|\?)|\.eot($|\?)|\.svg($|\?)/i,
          type: 'asset/resource',
          generator: {
              filename: 'fonts/[name][ext][query]'
          }
      },
    ],
  },

  // Define used plugins
  plugins: [
    // Load .env file for environment variables in JS
    new Dotenv({
      path: "./.env"
    }),

    // Extracts CSS into separate files
    new MiniCssExtractPlugin({
      filename: "[name].css",
      chunkFilename: "[id].css"
    }),
  ],
};