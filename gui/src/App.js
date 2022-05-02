
import './App.css';
import Display from './Display/display';
import React from 'react';
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Dissassembler from './disassembler/disassembler';
import Navbar from './navbar/navbar';



function App() {

 
  return (

    <Router>
      <Navbar />
      <Routes>
          <Route path="/" element={<Display/>}/>
          <Route path="/disassembler" element={<Dissassembler/>}/>
      </Routes>
      
    </Router>

    
  );
}

export default App;
