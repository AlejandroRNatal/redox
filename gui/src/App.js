
import './App.css';
import Display from './Display/screen';
import React from 'react';
import { invoke } from '@tauri-apps/api/tauri'


function App() {

 
  return (
    <div className="App">
      <Display></Display>
    </div>
  );
}

export default App;
