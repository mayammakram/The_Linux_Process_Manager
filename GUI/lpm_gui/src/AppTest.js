// import logo from './logo.svg';
// import './App.css';
// import Graph from './Graph.js'
// import Chart from './chart.js'
// import * as React from 'react';
// import PropTypes from 'prop-types';
// import Tabs from '@mui/material/Tabs';
// import Tab from '@mui/material/Tab';
// import Typography from '@mui/material/Typography';
// import Box from '@mui/material/Box';

// import Main from './components/Header';

// function TabPanel(props) {
//   const { children, value, index, ...other } = props;

//   return (
//     <div
//       role="tabpanel"
//       hidden={value !== index}
//       id={`simple-tabpanel-${index}`}
//       aria-labelledby={`simple-tab-${index}`}
//       {...other}
//     >
//       {value === index && (
//         <Box sx={{ p: 3 }}>
//           <Typography>{children}</Typography>
//         </Box>
//       )}
//     </div>
//   );
// }

// TabPanel.propTypes = {
//   children: PropTypes.node,
//   index: PropTypes.number.isRequired,
//   value: PropTypes.number.isRequired,
// };

// function a11yProps(index) {
//   return {
//     id: `simple-tab-${index}`,
//     'aria-controls': `simple-tabpanel-${index}`,
//   };
// }

// export default function BasicTabs() {
//   const [value, setValue] = React.useState(0);

//   const handleChange = (event, newValue) => {
//     setValue(newValue);
//   };

//   return (

//     <Box sx={{ width: '100%' }}>
//       <Main />
//       <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
//         <Tabs value={value} onChange={handleChange} aria-label="basic tabs example" centered>
//           <Tab label="Processes" {...a11yProps(0)} />
//           <Tab label="Resources" {...a11yProps(1)} />
//           <Tab label="Item Three" {...a11yProps(2)} />
//         </Tabs>
//       </Box>
//       <TabPanel value={value} index={0}>
//       </TabPanel>
//       <TabPanel value={value} index={1}>
//         Item Two
//       </TabPanel>
//       <TabPanel value={value} index={2}>
//         Item Three
//       </TabPanel>
//     </Box>
//   );
// }

import * as React from 'react';
import { useState } from "react"; 
import PropTypes from 'prop-types';
import Toolbar from '@mui/material/Toolbar';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
// import SearchIcon from '@mui/icons-material/Search';
import Typography from '@mui/material/Typography';
import Link from '@mui/material/Link';

import {CCollapse, CHeader, CHeaderBrand, CHeaderToggler, CHeaderNav, CContainer, CNavItem, CNavLink, CForm, CFormInput, CButton} from '@coreui/react'

export default function Header(props) {

  const [visible, setVisible] = useState(false)
  return (
    <>
      <CHeader>
        <CContainer fluid>
          <CHeaderBrand href="#">Header</CHeaderBrand>
          <CHeaderToggler onClick={() => setVisible(!visible)} />
          <CCollapse className="header-collapse" visible={visible}>
            <CHeaderNav>
              <CNavItem>
                <CNavLink href="#" active>
                  Home
                </CNavLink>
              </CNavItem>
              <CNavItem>
                <CNavLink href="#">Link</CNavLink>
              </CNavItem>

              <CNavItem>
                <CNavLink href="#" disabled>
                  Disabled
                </CNavLink>
              </CNavItem>
            </CHeaderNav>
            <CForm className="d-flex">
              <CFormInput className="me-2" type="search" placeholder="Search" />
              <CButton type="submit" color="success" variant="outline">
                Search
              </CButton>
            </CForm>
          </CCollapse>
        </CContainer>
      </CHeader>
    </>
  )
  
}

