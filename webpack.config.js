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
            './app/index.jsx'
        ],
        module: {
            loaders: [
                {
                    test: /\.jsx?$/,
                    exclude: /node_modules/,
                    use: {
                        loader: 'babel-loader',
                        options: {
                            presets: ['react']
                        }
                    }
                },
                {
                    test: /\.less$/,
                    use: [{
                        loader: "style-loader"
                    }, {
                        loader: "css-loader", options: {
                            sourceMap: true
                        }
                    }, {
                        loader: "less-loader", options: {
                            sourceMap: true
                        }
                    }]
                }
            ]
        },
        resolve: {
            extensions: ['*', '.js', '.jsx', '.less']
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
