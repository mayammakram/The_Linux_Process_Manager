import React, { useEffect, useReducer, useRef } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend } from 'recharts';

const API_URL = 'http://127.0.0.1:8080';

function cpuUsageReducer(state, action) {
  switch (action.type) {
    case 'add':
      const coreData = state[action.payload.core_id] || [];
      return { ...state, [action.payload.core_id]: [...coreData, action.payload.data] };
    default:
      return state;
  }
}

function CpuUsageGraph() {
  const [cpuUsageData, dispatch] = useReducer(cpuUsageReducer, {});
  const colors = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042', '#AF19FF', '#FF1919'];
  const chartRef = useRef();

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch(`${API_URL}/cpu`);
        const data = await response.json();
        const timestamp = new Date().getTime();
        data.forEach(d => {
          const coreId = `core_${d.core_id}`;
          dispatch({
            type: 'add',
            payload: {
              core_id: coreId,
              data: { usage: d.usage, time: timestamp },
            },
          });
          if (!cpuUsageData[coreId]) {
            dispatch({
              type: 'add',
              payload: { core_id: coreId, data: [] },
            });
          }
        });
        chartRef.current.scrollToEnd();
      } catch (error) {
        console.error('Error:', error);
      }
    };
    const interval = setInterval(fetchData, 700);
    return () => clearInterval(interval);
  }, [cpuUsageData]);

  const combinedData = Object.entries(cpuUsageData).map(([core_id, data]) => ({ core_id, data }));

  return (
    <div>
      <LineChart width={800} height={400} data={combinedData} ref={chartRef}>
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis dataKey="time" type="number" domain={['auto', 'auto']} tickFormatter={time => new Date(time).toLocaleTimeString()} />
        <YAxis />
        <Tooltip labelFormatter={time => new Date(time).toLocaleTimeString()} />
        <Legend />
        {combinedData.map(({ core_id, data }, index) => (
          <Line
            type="monotone"
            dataKey="usage"
            data={data.filter(d => d != null)}
            strokeWidth={2}
            key={core_id}
            stroke={colors[index % colors.length]}
            name={core_id}
            label={{ position: 'top', value: core_id, fill: colors[index % colors.length] }}
          />
        ))}
      </LineChart>
    </div>
  );
}


export default CpuUsageGraph;
