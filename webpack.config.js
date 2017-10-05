var CopyWebpackPlugin = require('copy-webpack-plugin');
var webpack = require('webpack');
var UglifyJSPlugin = require('uglifyjs-webpack-plugin');

module.exports = function(env) {
    var result = {
        context: __dirname,
        plugins: [
            new CopyWebpackPlugin([
                { from: 'static' }
            ])
        ],
        entry: [
            './js/index.js'
        ],
        module: {
            loaders: [{
                test: /\.jsx?$/,
                exclude: /node_modules/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: ['react']
                    }
                }
            }]
        },
        resolve: {
            extensions: ['*', '.js', '.jsx']
        },
        output: {
            path: __dirname + '/dist',
            filename: 'bundle.js'
        }
    };
    switch (env) {
        case 'production':
            result.plugins.push(new UglifyJSPlugin());
        break;
    }
    return result;
};
