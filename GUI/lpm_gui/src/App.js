import React, { Component } from "react";
import ReactGA from "react-ga";
import $ from "jquery";
import "./App.css";
import Header from "./Components/Header";
import Footer from "./Components/Footer";

//Table
import Table from "./Components/Table/Table";

//Charts 
// import Donut from "./Components/charts/Donut/Donut";
// import Chart from "./Components/charts/chart";
// import Sunburst from "./Components/charts/Sunburst/main";

// import "./Components/charts/Sunburst/styles.css";
//Pages
import About from "./Components/About";
import Resources from "./Components/Charts/Resources";
import Contact from "./Components/Contact";
import Portfolio from "./Components/Portfolio";

class App extends React.Component {
  onSelect(event) {
    console.log(event);
  }
  constructor(props) {
    super(props);
    this.state = {
      foo: "bar",
      resumeData: {}  
      //sunburstData: {}
    };

    ReactGA.initialize("UA-110570651-1");
    ReactGA.pageview(window.location.pathname);
  }

  getResumeData() {
    $.ajax({
      url: "./resumeData.json",
      dataType: "json",
      cache: false,
      success: function(data) {
        this.setState({ resumeData: data });
      }.bind(this),
      error: function(xhr, status, err) {
        console.log(err);
        alert(err);
      }
    });
  }

  // getSunburstData() {
  //   $.ajax({
  //     url: "./sunburstData.json",
  //     dataType: "json",
  //     cache: false,
  //     success: function(data) {
  //       this.setState({ sunburstData: data });
  //     }.bind(this),
  //     error: function(xhr, status, err) {
  //       console.log(err);
  //       alert(err);
  //     }
  //   });
  // }


  componentDidMount() {
    this.getResumeData();
  }
  render() 
  { 
    return (
      <div className="App">
        <Header data={this.state.resumeData.main} />
        {/* <Donut /> */}
        <Table/>
        {/* <Portfolio data={this.state.resumeData.portfolio} /> */}
        <Resources data={this.state.resumeData.Resources} />
        {/* <Contact data={this.state.resumeData.main} /> */}
        <Footer data={this.state.resumeData.main} />
      </div>
    );
  }
}

export default App;


// import { useEffect, useState } from 'react'
// ;
// const API_URL = 'http://127.0.0.1:8080';

// function App() {
//   const [processes, setProcesses] = useState([]);

//   useEffect(() => {
    
//     fetch(`${API_URL}/processes`)
//       .then(response => response.json())
//       .then(data => {
//         setProcesses(data);
//       }).catch((error) => {
//         console.error('Error:', error);
//       });
      

//   }, []);

//   return (
//     <table>
//       <thead>
//         <tr>
//           <th>Name</th>
//           <th>PID</th>
//           <th>State</th>
//           <th>Parent ID</th>
//           <th>Priority</th>
//           <th>Niceness</th>
//           <th>User ID</th>
//           <th>Memory</th>
//           <th>CPU Time</th>
//           {/* <th>Network Bandwidth</th> */}
//           <th>Opened Files</th>

          
//         </tr>
//       </thead>
//       <tbody>
//         {processes.map(process => (
//           <tr key={process.pid}>
//             <td>{process.name}</td>
//             <td>{process.pid}</td>
//             <td> {process.state}</td>
//             <td>{process.parent_id}</td>
//             <td>{process.priority}</td>
//             <td>{process.niceness}</td>
//             <td>{process.user_id}</td>
//             <td>{process.memory}</td>
//             <td>{process.cpu_time}</td>
//             {/* <td>{process.network_bandwidth}</td> */}
//             <td>{process.opened_files}</td>
//             {/* <td>{process.cpu_usage}</td> */}
//           </tr>
//         ))}
//       </tbody>
//     </table>
//   );
// }

// export default App;
