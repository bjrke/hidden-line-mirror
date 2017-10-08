import React from 'react';
import ReactDOM from 'react-dom';

import '../styles/main.less';

export class Application extends React.Component {
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

/*
- 3dplot
- dreidext
- projekt
- polysweep
- polygon
- dreiecke
- linien
- punkte
- zeit
- vector 
 */
