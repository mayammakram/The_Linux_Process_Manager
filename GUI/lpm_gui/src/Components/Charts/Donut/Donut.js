import {
  Chart,
  ChartTitle,
  ChartLegend,
  ChartSeries,
  ChartSeriesItem,
  ChartSeriesLabels,
} from "@progress/kendo-react-charts";
import { COLORS } from "./constants";
import Box from '@mui/material/Box';
import Fade from 'react-reveal/Fade';
// Graph data
const applicationsStatusThisMonth = [
  {
    status: "Accepted",
    value: 14,
    color: COLORS.accepted,
  },
  {
    status: "Interviewing",
    value: 14,
    color: COLORS.interviewing,
  },
  {
    status: "Rejected",
    value: 40,
    color: COLORS.rejected,
  },
  {
    status: "Pending",
    value: 32,
    color: COLORS.pending,
  },
];

// Show category label for each item in the donut graph
const labelContent = e => e.category;

const Charts = props => {
  return (
    // <section id="Donut" sx={{ width: '100%', height: '500px' }}>
        <Fade duration={1000}>
          <div className="row">
            <div className="three columns">
              <Chart sx = {{flex: "1", length: "1000px"}}>
                <ChartTitle text="CPU Utilization per Process" />
                <ChartLegend visible={false} />
                <ChartSeries>
                  <ChartSeriesItem
                    type="donut"
                    data={applicationsStatusThisMonth}
                    categoryField="status"
                    field="value"
                  >
                    <ChartSeriesLabels
                      color="#fff"
                      background="none"
                      content={labelContent}
                    />
                  </ChartSeriesItem>
                </ChartSeries>
              </Chart>
          </div>
        </div>
      </Fade>
    // </section>
  );
};

export default Charts;