import React, {Component} from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import { Grid } from '../App';
import styled from 'styled-components';

import './display.css';

const WIDTH_PIXELS = 308; //Onlty the first 240 are used, the remaining 68 are H-Blank
const HEIGHT_PIXELS = 228; //bottom 68 are V-Blank
const STARTING_COLOR = "#FF00FF";// Start screen off as White

const pixel_w = 10;
const pixel_h = 10;

class Pixel extends Component {

  constructor(props){
      super(props);
      this.state = {
        col: props.col,
        row: props.row,
        width: props.width,
        height: props.height,
        color: props.color,
    };
  }

  render(){

    const {
        col,
        row,
        width,
        height,
        color,
    } = this.props;

    return (
        <div
        style = { {
                    // marginBottom: -10,
                    display: "inline-block",
                    backgroundColor: color,
                    outline: "1px solid rgb(175, 216, 248)",
                    width: width,
                    height: height }}
            id={`pixel-${this.state.row}-${this.state.col}`}
            className={`pixel`}
            ></div>
      
    );
  }

  async getFrame() {
   let response = await fetch("data-location here");
   console.log(response); 
  }

}


export default class Display extends Component {
    constructor() {
        super();
        this.state = {
            pixels: [],
        };
    }

    componentDidMount() {
        // const invoke = window.__TAURI_IPC__.invoke
        // invoke('read_rom').then((rom) => console.log(rom[1]))
        const pixels = getInitialPixels();
        this.setState({pixels});
    }

    render() {
        const {pixels} = this.state;// Tuple unpacking?

        return (
            // <>
                <div className="pixels">
                    {pixels.map((row, rowIndex) => {
                    
                        return (
                            <div key={rowIndex}>
                                {row.map((pixel, pixelIndex) => {
                                const {row, col, color, width, height} = pixel;
                                    return (
                                        <Pixel 
                                            key={pixelIndex}
                                            row={row}
                                            col={col}
                                            color={color}
                                            width={width}
                                            height={height}>
                                                
                                            </Pixel>
                                    );
                                })}
                            </div>
                        );
                    })}
                </div>
            /* </> */
        );
    }
}

const getInitialPixels = () => {
    const pixels =[];

    for(let row = 0; row < 20; row++){
        const current = [];
        for(let col = 0; col < 30; col++){
            current.push(createPixel(col, row))
        }
        pixels.push(current)
    }
    return pixels;
};

const createPixel = (row, col) => {
    return {
        col,
        row,
        color: STARTING_COLOR,
        height: pixel_h,
        width: pixel_w,
    }
}