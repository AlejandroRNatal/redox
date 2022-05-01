import { React, Component} from 'react'
import { invoke } from '@tauri-apps/api/tauri'


export default class Dissassembler extends Component {

    constructor(props){
        super(props);
        this.state = {data: ["Empty"]};
    }

    render(){
        const {data} = this.state;

        return (
            <div className="dissassembler">
                <button onClick={this.fetchData.bind(this)}>click</button>
                
                {[...Array(data)].map((line) => {
                    return (
                        <p>{line}</p>
                    )
                })}
            </div>
        )
    }
    
    async fetchData() {
        let props = await invoke('read_rom');
        this.setState({data: props})
        // return props;
    }
}