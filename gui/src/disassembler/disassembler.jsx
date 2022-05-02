import { React, Component} from 'react'
import { invoke } from '@tauri-apps/api/tauri'

import './disassembler.css';

export default class Dissassembler extends Component {

    constructor(props){
        super(props);
        this.state = {data: ["Empty"]};
    }

    async componentDidMount(){
       await this.fetchData()
    }
    
    render(){
        const {data} = this.state
        
        return (
            <div className="disassembler">
                
                {data.map(( line, idx) => {
                    return (
                        <p key={idx}>{line}</p>
                    )
                })}
            </div>
        )
    }
    
    async fetchData() {
        let props = await invoke('read_rom');
        
        this.setState({data: [props]})
    }
}