import { Task } from '../../types';
import { useTaskStore } from '../../store/taskStore';

interface NowTaskProps {
  task: Task;
}

export function NowTask({ task }: NowTaskProps) {
  const { completeTask, deferTask } = useTaskStore();

  return (
    <section>
      <h2 className="text-sm font-bold text-stone-400 uppercase tracking-wider mb-3">Now</h2>
      <div className="bg-white border-2 border-stone-800 rounded-xl p-6 shadow-[4px_4px_0px_0px_rgba(44,41,41,1)] flex flex-col gap-4">
        <div className="flex justify-between items-start">
          <h3 className="text-2xl font-bold text-stone-900 leading-tight">{task.title}</h3>
          <span className="bg-stone-100 text-stone-600 text-xs px-2 py-1 rounded font-medium border border-stone-200 shrink-0 ml-2">
            Active
          </span>
        </div>
        
        {task.context && (
          <div className="text-stone-500 text-sm flex items-center gap-1">
            <span className="font-medium text-stone-400">Context:</span> {task.context}
          </div>
        )}
        
        <div className="flex gap-2 mt-2 pt-2 border-t border-stone-100">
          <button 
            onClick={() => completeTask(task.id)}
            className="flex-1 bg-stone-800 text-white px-6 py-3 rounded-lg font-medium hover:bg-stone-700 transition-colors flex items-center justify-center gap-2"
          >
            <span>Complete</span>
          </button>
          <button 
            onClick={() => deferTask(task.id)}
            className="px-4 py-3 border border-stone-300 text-stone-600 rounded-lg font-medium hover:bg-stone-50 transition-colors"
          >
            Defer
          </button>
        </div>
      </div>
    </section>
  );
}
