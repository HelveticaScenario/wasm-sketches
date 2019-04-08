import React from 'react'
import { Link } from 'react-router-dom'
import { WasmContext, unwrapContextValue } from './wasmContext'

export default class Directory extends React.Component<{}, {}> {
	static contextType = WasmContext
	context!: React.ContextType<typeof WasmContext>
	render() {
		const { sketches } = unwrapContextValue(this.context)
		const links = sketches.map(({ url, name }) => (
			<li key={url}>
				<Link to={url}>{name}</Link>
			</li>
		))
		return <ul>{links}</ul>
	}
}
