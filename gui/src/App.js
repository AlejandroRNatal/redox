
import './App.css';
import Display from './Display/screen';
import React from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import Dissassembler from './dissassembler/dissassembler';


function App() {

 
  return (
    <div className="App">
      <Display></Display>
      <Dissassembler></Dissassembler>
    </div>
  );
}

export default App;
