const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')

const dist = path.resolve(__dirname, 'dist')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
	entry: './ts/index.tsx',
	output: {
		path: dist,
		filename: 'bundle.js',
	},
	devServer: {
		contentBase: dist,
		historyApiFallback: true,
	},
	resolve: {
		extensions: ['.ts', '.tsx', '.js', '.wasm'],
	},
	plugins: [
		new HtmlWebpackPlugin({
			template: 'index.html',
		}),

		new WasmPackPlugin({
			crateDirectory: path.resolve(__dirname, 'crate'),
			// WasmPackPlugin defaults to compiling in "dev" profile. To change that, use forceMode: 'release':
			forceMode: 'release',
		}),
	],
	module: {
		rules: [
			{
				test: /\.tsx?$/,
				loader: 'ts-loader',
			},
		],
	},
}
