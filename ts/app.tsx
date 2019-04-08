import React from 'react'
import { Route } from 'react-router-dom'
import { WasmContext, unwrapContextValue } from './wasmContext'
import Sketch from './sketch'
import Directory from './directory'

export default class App extends React.Component {
	static contextType = WasmContext
	context!: React.ContextType<typeof WasmContext>
	render() {
		const { sketches } = unwrapContextValue(this.context)
		const routes = []
		for (let i = 0; i < sketches.length; i++) {
			const sketch = sketches[i]
			routes.push(
				<Route
					key={`/${sketch.url}`}
					path={`/${sketch.url}`}
					exact
					render={props => (
						<Sketch key={sketch.index} sketch={sketch} {...props} />
					)}
				/>
			)
		}
		return (
			<React.Fragment>
				<Route path="/" exact component={Directory} />
				{routes}
			</React.Fragment>
		)
	}
}
