import MUIDataTable from 'mui-datatables';
import React, { useState, useEffect } from 'react';
import Switch from '@mui/material/Switch';
const API_URL = 'http://127.0.0.1:8080';

const Table = () => {
  const columns = [
    { label: 'Process', name: 'name' },
    { label: 'pid', name: 'pid' },
    { label: 'state', name: 'state', options: { sort: true } },
    { label: 'parent id', name: 'parent_id', options: { sort: true } },
    { label: 'Priority', name: 'priority', options: { sort: true } },
    { label: 'Nice', name: 'niceness', options: { sort: true } },
    { label: 'User', name: 'user_id', options: { sort: true } },
    { label: 'Memory', name: 'memory', options: { sort: true } },
    { label: 'CPU Time', name: 'cpu_time', options: { sort: true } },
    // { label: 'Network Bandwidth', name: 'Network Bandwidth', options: { sort: true } },
    { label: 'Number of Opened Files', name: 'opened_files', options: { sort: true } },
  ];
  const options = {
    filterType: 'checkbox'
  };

  const [processes, setProcesses] = useState([]);

  const [condition, setCondition] = useState(false);
  const handleChange = (event) => {
    setCondition(event.target.checked);
  };


useEffect(() => {
  const interval = setInterval(() => {
    if (condition) {
      clearInterval(interval); // Stop the interval if the condition holds true
    } else {
      fetch(`${API_URL}/processes`)
        .then(response => response.json())
        .then(data => {
          setProcesses(data);
        }).catch((error) => {
          console.error('Error:', error);
        });
    }
  }, 100); // Set the interval time in milliseconds

  return () => clearInterval(interval); // Clear the interval when the component unmounts
}, [condition]);


  return (
    <section id='Processes'>
    <div style={{ maxWidth: '100%', backgroundColor: '#ebeeee'}}>

      <Switch
        color="secondary"
        checked={condition}
        onChange={handleChange}
        inputProps={{ 'aria-label': 'controlled' }}
        label = "Pause"
      />
      <MUIDataTable
        columns={columns}
        data={processes}
        title='Processes'
        options={options}
      />
    </div>
    </section>
  );
};
export default Table;