require('./static/index.scss');

import('./pkg').then(module => {module.run_app();});
