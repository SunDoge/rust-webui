/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';

import App from './App';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}


function renderAfterWebui() {
  try {
    if (webui !== undefined) {
      render(() => <App />, root!);
    }
  } catch {
    setTimeout(renderAfterWebui, 100)
  }
}

renderAfterWebui()


// render(() => <App />, root!);
