import { createSignal, type Component, createEffect } from 'solid-js';
import { webuiCall } from './utils';

const App: Component = () => {
  const [x, setX] = createSignal(0);
  const [y, setY] = createSignal(0);

  const [sum, setSum] = createSignal(0);

  createEffect(() => {
    webuiCall<number>("add2", { x: x(), y: y() }).then((res) => {
      setSum(res);
    });
  })

  return (
    <div class="min-h-screen flex items-center justify-center">
      <div class="bg-gray-100 p-8 rounded-md shadow-md">
        <h1 class="text-2xl font-semibold mb-4">Simple Calculator</h1>

        <div class="mb-4">
          <label class="block text-gray-700">X:</label>
          <input
            class="w-full border rounded-md p-2"
            type="number"
            value={x()}
            onInput={(e) => setX(Number(e.target.value))}
          />
        </div>

        <div class="mb-4">
          <label class="block text-gray-700">Y:</label>
          <input
            class="w-full border rounded-md p-2"
            type="number"
            value={y()}
            onInput={(e) => setY(Number(e.target.value))}
          />
        </div>
        <p class="text-xl font-semibold mb-4">Sum2: {sum()}</p>
      </div>
    </div>
  );
};

export default App;
