var CopyWebpackPlugin = require('copy-webpack-plugin');
var webpack = require('webpack');

module.exports = function(env) {
    var result = {
        context: __dirname,
        plugins: [
            new CopyWebpackPlugin([
                { from: 'static' }
            ])
        ],
        entry: [
            'babel-polyfill',
            './js/index.js'
        ],
        module: {
            loaders: [{
                test: /\.jsx?$/,
                exclude: /node_modules/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: ['env', 'react']
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
            result.plugins.push(new webpack.optimize.UglifyJsPlugin());
        break;
    }
    return result;
};
