import { React, Component} from 'react'
import { invoke } from '@tauri-apps/api/tauri'

export default class Decompiler extends Component {

    constructor(props){
        this.state = {props}
    }

    render(){

        const {rows} = this.state

        return (
            <table>
               
                {rows.map((row , i) => { 

                    // Process row data

                    //return the row HTML
                    return (
                        <tr>
                            {row.map( content => {
                                return <td>{content}</td>
                            })}
                        </tr>
                    )

                } )}
                    
            </table>
        )
    }
}