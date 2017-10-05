import React from 'react';
import ReactDOM from 'react-dom';

import Greeting from './greeting'

const element = document.createElement('div');
document.body.appendChild(element);
ReactDOM.render(React.createElement(Greeting), element);
document.title='Hidden Line';
