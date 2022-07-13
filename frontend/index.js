import 'bootstrap';

// or get all of the named exports for further usage
// import * as bootstrap from 'bootstrap';

import 'bootstrap-icons/font/fonts/bootstrap-icons.woff';
import 'bootstrap-icons/font/fonts/bootstrap-icons.woff2';

if (process.env.NODE_ENV !== 'production') {
    console.log('Looks like we are in development mode!');
}

// Load Styles
import './scss/main.scss';

// Load Bootstrap init
import {initBootstrap} from "./bootstrap";

// Loading bootstrap with optional features
initBootstrap({
  tooltip: true,
  popover: true,
  toasts: true,
});
