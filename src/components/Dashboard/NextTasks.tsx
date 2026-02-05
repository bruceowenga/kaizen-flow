import { useState } from 'react';
import { Task } from '../../types';
import { useTaskStore } from '../../store/taskStore';
import { ConfirmationDialog } from '../ConfirmationDialog';

interface NextTasksProps {
  tasks: Task[];
  hasNowTask: boolean;
}

export function NextTasks({ tasks, hasNowTask }: NextTasksProps) {
  const { startTask } = useTaskStore();
  const [taskToStart, setTaskToStart] = useState<string | null>(null);

  const handleStartClick = (taskId: string) => {
    if (hasNowTask) {
      setTaskToStart(taskId);
    } else {
      startTask(taskId);
    }
  };

  const confirmStart = () => {
    if (taskToStart) {
      startTask(taskToStart);
      setTaskToStart(null);
    }
  };

  return (
    <section>
      <div className="flex justify-between items-end mb-3">
        <h2 className="text-sm font-bold text-stone-400 uppercase tracking-wider">Next</h2>
        <span className="text-xs text-stone-400 font-medium">{tasks.length} tasks</span>
      </div>
      
      <div className="flex flex-col gap-3">
        {tasks.length > 0 ? (
          tasks.map((task) => (
            <div key={task.id} className="bg-white border border-stone-200 rounded-lg p-4 hover:border-stone-400 transition-colors flex justify-between items-center group shadow-sm hover:shadow-md">
              <div className="flex-1 pr-4">
                <h4 className="font-medium text-stone-800">{task.title}</h4>
                <div className="flex gap-2 text-xs text-stone-400 mt-1">
                  {task.created_at && (
                    <span>Created {new Date(task.created_at * 1000).toLocaleDateString()}</span>
                  )}
                  {task.context && <span>• {task.context}</span>}
                </div>
              </div>
              
              <button 
                onClick={() => handleStartClick(task.id)}
                className="opacity-0 group-hover:opacity-100 bg-stone-800 text-white px-3 py-1.5 rounded-md text-xs font-bold hover:bg-stone-700 transition-all transform translate-x-2 group-hover:translate-x-0"
              >
                Start
              </button>
            </div>
          ))
        ) : (
          <div className="bg-stone-50 border border-dashed border-stone-200 rounded-lg p-8 text-center text-stone-400 text-sm">
            No next tasks. Capture something!
          </div>
        )}
      </div>

      <ConfirmationDialog
        isOpen={!!taskToStart}
        title="Switch Active Task?"
        message="This will move your current NOW task back to the Next list. Are you sure you want to switch focus?"
        confirmLabel="Switch Task"
        onConfirm={confirmStart}
        onCancel={() => setTaskToStart(null)}
      />
    </section>
  );
}
