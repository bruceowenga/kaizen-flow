import { useState } from 'react';
import { Layout } from './components/Layout';
import { Dashboard } from './components/Dashboard/Dashboard';
import { useTaskStore } from './store/taskStore';

function App() {
  const [inputValue, setInputValue] = useState("");
  const { quickCapture } = useTaskStore();

  const handleCapture = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputValue.trim()) return;
    
    await quickCapture(inputValue);
    setInputValue("");
  };

  return (
    <Layout>
      <div className="mb-8">
        <form onSubmit={handleCapture} className="w-full">
          <input
            type="text"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            placeholder="What needs to be done? (Press Enter)"
            className="w-full bg-white border border-stone-300 text-stone-900 text-lg rounded-xl px-4 py-3 shadow-sm focus:outline-none focus:ring-2 focus:ring-stone-500 focus:border-transparent transition-shadow"
            autoFocus
          />
        </form>
      </div>

      <Dashboard />
    </Layout>
  );
}

export default App;
