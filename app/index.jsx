import React from 'react';
import ReactDOM from 'react-dom';

import './main.less';

class Application extends React.Component {
  render() {
    return (
      <svg viewBox="-1 -1 2 2" preserveAspectRatio="xMidYMid slice">
        <circle cx={0} cy={0} r={1} fill="red" />
      </svg>
    );
  }
}

const element = document.createElement('div');
ReactDOM.render(<Application />, element);
document.title='Hidden Line';
document.body.appendChild(element);
