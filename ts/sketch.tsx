import React from 'react'
import { RouteComponentProps } from 'react-router-dom'
import { WasmContext, SketchDescription } from './wasmContext'

export interface SketchProps extends RouteComponentProps {
	sketch: SketchDescription
}

export default class Sketch extends React.Component<SketchProps, {}> {
	static contextType = WasmContext
	context!: React.ContextType<typeof WasmContext>
	canvasRef: React.Ref<HTMLCanvasElement>
	constructor(props: SketchProps) {
		super(props)
		this.canvasRef = React.createRef()
	}

	componentDidMount() {}

	render() {
		return <canvas ref={this.canvasRef} />
	}
}
