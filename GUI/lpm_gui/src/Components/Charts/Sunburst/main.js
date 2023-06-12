import React, { useState, useEffect } from "react";
import "./styles.css";
import Sunburst from "./Sunburst";
import data from "./data";
const API_URL = 'http://127.0.0.1:8080';

export default function SunBurst() {
  const [sunburstData] = useState(data);

  const [processes, setProcesses] = useState([]);

  useEffect(() => {
    
    fetch(`${API_URL}/processes`)
      .then(response => response.json())
      .then(data => {
        setProcesses(data);
      }).catch((error) => {
        console.error('Error:', error);
      });
      

  }, []);


  return (
    <div className="App">
      <Sunburst
        data={sunburstData}
        width="400"
        height="400"
        count_member="size"
        font_size={8}
        labelFunc={node => node.data.name}
      />
    </div>
  );
}