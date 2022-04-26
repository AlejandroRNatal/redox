
import { React, Component} from 'react'
import { writeFile, Dir } from '@tauri-apps/api/fs'

import { invoke } from '@tauri-apps/api/tauri'
import { fetch } from '@tauri-apps/api/http'

export default class FilePicker extends Component {
    constructor(props){
    }

    findFile(){
        //await(invoke('rust function name'))
    }

}
