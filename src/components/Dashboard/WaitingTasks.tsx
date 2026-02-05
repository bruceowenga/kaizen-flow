import { Task } from '../../types';

interface WaitingTasksProps {
  tasks: Task[];
}

export function WaitingTasks({ tasks }: WaitingTasksProps) {
  if (tasks.length === 0) return null;

  return (
    <section>
      <h2 className="text-sm font-bold text-stone-400 uppercase tracking-wider mb-3">Waiting</h2>
      <div className="flex flex-col gap-2">
        {tasks.map((task) => (
          <div key={task.id} className="bg-stone-50 border border-stone-100 rounded-lg p-3 opacity-75">
            <h4 className="text-sm text-stone-600 font-medium">{task.title}</h4>
            {task.context && (
              <span className="text-xs text-stone-400 block mt-1">Context: {task.context}</span>
            )}
          </div>
        ))}
      </div>
    </section>
  );
}
