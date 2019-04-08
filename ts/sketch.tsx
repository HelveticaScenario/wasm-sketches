import React from 'react'
import { RouteComponentProps } from 'react-router-dom'
import {
	WasmContext,
	SketchDescription,
	unwrapContextValue,
} from './wasmContext'
import { uw } from './utils'
import runSketch from './runSketch'

export interface SketchProps extends RouteComponentProps {
	sketch: SketchDescription
}

export default class Sketch extends React.Component<SketchProps, {}> {
	static contextType = WasmContext
	context!: React.ContextType<typeof WasmContext>
	canvasRef: React.RefObject<HTMLCanvasElement>
	run?: (now: number) => void
	constructor(props: SketchProps) {
		super(props)
		this.canvasRef = React.createRef()
	}

	loadSketch = (props: SketchProps) => {
		const { sketch } = props
		const { module } = unwrapContextValue(this.context)
		const canvas = uw(this.canvasRef.current)
		const run = runSketch(canvas, sketch, module)
		this.run = run

		const raf = (now: number) => {
			if (this.run === run) {
				requestAnimationFrame(raf)
				run(now)
			}
		}
		raf(performance.now())
	}

	componentDidMount() {
		this.loadSketch(this.props)
	}
	componentDidUpdate(prevProps: SketchProps) {
		if (prevProps.sketch !== this.props.sketch) {
			this.loadSketch(this.props)
		}
	}
	componentWillUnmount() {
		this.run = undefined
	}

	render() {
		return <canvas ref={this.canvasRef} />
	}
}
