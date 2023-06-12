import React, { Component } from "react";
import Slide from "react-reveal";

import SunBurst from "./Sunburst/main";
import Chart from "./chart";
// import LineG from "./Line/Graph";

class Resources extends Component {

  render() {
    if (!this.props.data) return null;

    const Sunburst = this.props.data.Sunburst.map(function (Sunburst) {
      return (
        <div key={Sunburst.header}>
          <h3>{Sunburst.header}</h3>
          <p className="info">
            <SunBurst />
          </p>
          <p>{Sunburst.description}</p>
        </div>
      );
    });

    const Graph = this.props.data.Graph.map(function (Graph) {
      return (
        <div key={Graph.company}>
          <h3>{Graph.company}</h3>
          <p className="info">
            <Chart />
          </p>
          <p>{Graph.description}</p>
        </div>
      );
    });

    return (
      <section id="Resources" style = {{backgroundColor : '#ebeeee'}}>
        <h1> . </h1>
        <Slide left duration={1300}>
          <div className="row Sunburst">
            <div className="three columns header-col" bodyAttributes={{style: 'background-color : #fff'}}>
              <h1>
                <span>Resources </span>
              </h1>
            </div>

            <div className="nine columns main-col">
              <div className="row item" >
                <div className="twelve columns">{Sunburst}</div>
              </div>
            </div>
          </div>
        </Slide>

        <Slide left duration={1300}>
          <div className="row Graph">
            <div className="three columns header-col">
              <h1>
                <span>Graph</span>
              </h1>
            </div>

            <div className="nine columns main-col">{Graph}</div>
          </div>
        </Slide>

      </section>
    );
  }
}

export default Resources;
