import React from 'react'
import {  Link } from "react-router-dom"

import './navbar.css';


const linkStyle = {
  margin: "1rem",
  textDecoration: "none",
  color: 'white'
};

const Navbar = ()=> {
    return (
        <div className="navbar">
            <li>
                <Link to="/disassembler" style={linkStyle}> Disassembler </Link>
            </li>
            <li>
                <Link to="/" style={linkStyle}>Main</Link>
            </li>
        </div>
    )
}

export default Navbar;