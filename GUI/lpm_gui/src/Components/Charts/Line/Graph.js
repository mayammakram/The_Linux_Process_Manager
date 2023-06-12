import React from "react";
import "./styles.css";
import "chartjs-plugin-zoom";
import { Line } from "react-chartjs-2";
// import * as brush from "chartjs-plugin-brush";
let res = [
  { date: "1", value: 35.98 },
  { date: "2", value: 147.49 },
  { date: "3", value: 146.93 },
  { date: "4", value: 139.89 },
  { date: "5", value: 125.6 },
  { date: "6", value: 108.13 },
  { date: "7", value: 115 },
  { date: "8", value: 118.8 },
  { date: "9", value: 124.66 },
  { date: "10", value: 113.44 },
  { date: "11", value: 5.78 },
  { date: "12", value: 113.46 },
  { date: "13", value: 122 },
  { date: "14", value: 118.68 },
  { date: "15", value: 117.45 },
  { date: "16", value: 118.7 },
  { date: "17", value: 119.8 },
  { date: "18", value: 115.81 },
  { date: "19", value: 118.76 },
  { date: "20", value: 125.3 },
  { date: "21", value: 118.68 },
  { date: "22", value: 117.45 },
  { date: "23", value: 118.7 },
  { date: "24", value: 119.8 },
  { date: "25", value: 115.81 },
  { date: "26", value: 118.76 },
  { date: "27", value: 125.3 },
  { date: "28", value: 125.25 },
  { date: "29", value: 124.5 },
  { date: "30", value: 14.5 },
  { date: "31", value: 1.5 },
  { date: "32", value: 140.5 },
  { date: "33", value: 4.5 },
  { date: "34", value: 1.5 },
  { date: "35", value: 140.5 },
  { date: "36", value: 1.5 },
  { date: "37", value: 4.5 },
  { date: "38", value: 144.5 },
  { date: "39", value: 14.5 },
  { date: "40", value: 144.5 },
  { date: "41", value: 114.5 },
  { date: "42", value: 14.5 },
  { date: "43", value: 141.5 },
  { date: "44", value: 14.5 },
  { date: "45", value: 141.5 },
  { date: "46", value: 14.5 },
  { date: "47", value: 111.5 },
  { date: "48", value: 14.5 },
  { date: "49", value: 141.5 },
  { date: "50", value: 114.5 }
];
const data = {
  labels: res.map((e) => e.date),
  datasets: [
    {
      label: "First dataset",
      data: res.map((e) => e.value),
      fill: true,
      backgroundColor: "rgba(75,192,192,0.2)",
      borderColor: "rgba(75,192,192,1)"
    },
    {
      label: "Second dataset",
      data: res.map((e) => e.value),
      fill: true,
      backgroundColor: "rgba(11,200,122,0.2)",
      borderColor: "rgba(11,31,12,1)"
    }
  ]
};

const options = {
  scales: {
    xAxes: [
      {
        stacked: true
      }
    ],
    yAxes: [
      {
        stacked: true
      }
    ]
  },
  pan: {
    enabled: true,
    mode: "x"
  },
  zoom: {
    enabled: true,
    mode: "x",
    sensitivity: 0.5
  }
};

export default function LineG() {
  return (
    <div className="LineG">
      <Line data={data} options={options} />
    </div>
  );
}
