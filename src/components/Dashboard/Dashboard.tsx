import { useEffect } from 'react';
import { useTaskStore } from '../../store/taskStore';
import { Task } from '../../types';

export function Dashboard() {
  const { dashboard, fetchDashboard, isLoading, error } = useTaskStore();

  useEffect(() => {
    fetchDashboard();
  }, [fetchDashboard]);

  if (error) {
    return (
      <div className="p-8 text-center text-red-600 bg-red-50 rounded-lg border border-red-200">
        <h3 className="font-bold mb-2">Error loading dashboard</h3>
        <p>{error}</p>
        <button 
          onClick={() => fetchDashboard()}
          className="mt-4 px-4 py-2 bg-white border border-red-200 rounded hover:bg-red-50 text-sm"
        >
          Retry
        </button>
      </div>
    );
  }

  if (isLoading && !dashboard) {
    return (
      <div className="flex flex-col gap-6 animate-pulse">
        <div className="h-48 bg-stone-200 rounded-xl w-full"></div>
        <div className="h-32 bg-stone-200 rounded-lg w-full"></div>
        <div className="h-32 bg-stone-200 rounded-lg w-full"></div>
      </div>
    );
  }

  const nowTask = dashboard?.now_task;
  const nextTasks = dashboard?.next_tasks || [];
  const waitingTasks = dashboard?.waiting_tasks || [];

  return (
    <div className="flex flex-col gap-8">
      {/* NOW Section */}
      <section>
        <h2 className="text-sm font-bold text-stone-400 uppercase tracking-wider mb-3">Now</h2>
        {nowTask ? (
          <div className="bg-white border-2 border-stone-800 rounded-xl p-6 shadow-[4px_4px_0px_0px_rgba(44,41,41,1)] flex flex-col gap-4">
            <div className="flex justify-between items-start">
              <h3 className="text-2xl font-bold text-stone-900">{nowTask.title}</h3>
              <span className="bg-stone-100 text-stone-600 text-xs px-2 py-1 rounded font-medium border border-stone-200">
                Active
              </span>
            </div>
            {nowTask.context && (
              <div className="text-stone-500 text-sm flex items-center gap-1">
                Context: {nowTask.context}
              </div>
            )}
            <div className="flex gap-2 mt-2">
              <button 
                onClick={() => useTaskStore.getState().completeTask(nowTask.id)}
                className="bg-stone-800 text-white px-6 py-2 rounded-lg font-medium hover:bg-stone-700 transition-colors"
              >
                Complete
              </button>
              <button 
                onClick={() => useTaskStore.getState().deferTask(nowTask.id)}
                className="border border-stone-300 text-stone-600 px-4 py-2 rounded-lg font-medium hover:bg-stone-50 transition-colors"
              >
                Defer
              </button>
            </div>
          </div>
        ) : (
          <div className="bg-white border border-dashed border-stone-300 rounded-xl p-8 text-center text-stone-400">
            No active task. Pick one from Next!
          </div>
        )}
      </section>

      {/* NEXT Section */}
      <section>
        <div className="flex justify-between items-end mb-3">
          <h2 className="text-sm font-bold text-stone-400 uppercase tracking-wider">Next</h2>
          <span className="text-xs text-stone-400 font-medium">{nextTasks.length} tasks</span>
        </div>
        
        <div className="flex flex-col gap-3">
          {nextTasks.length > 0 ? (
            nextTasks.map((task: Task) => (
              <div key={task.id} className="bg-white border border-stone-200 rounded-lg p-4 hover:border-stone-400 transition-colors flex justify-between items-center group">
                <div className="flex-1">
                  <h4 className="font-medium text-stone-800">{task.title}</h4>
                  {task.created_at && (
                    <span className="text-xs text-stone-400">
                      Created {new Date(task.created_at * 1000).toLocaleDateString()}
                    </span>
                  )}
                </div>
                {!nowTask && (
                  <button 
                    onClick={() => useTaskStore.getState().quickCapture(task.title).then(() => useTaskStore.getState().deleteTask(task.id))} // Hack for now, replace with proper promote command later
                    className="opacity-0 group-hover:opacity-100 bg-stone-100 text-stone-600 px-3 py-1 rounded text-xs font-medium hover:bg-stone-200 transition-opacity"
                  >
                    Start
                  </button>
                )}
              </div>
            ))
          ) : (
            <div className="text-stone-400 text-sm italic py-4">No next tasks. Relax!</div>
          )}
        </div>
      </section>

      {/* WAITING Section */}
      {waitingTasks.length > 0 && (
        <section>
          <h2 className="text-sm font-bold text-stone-400 uppercase tracking-wider mb-3">Waiting</h2>
          <div className="flex flex-col gap-2">
            {waitingTasks.map((task: Task) => (
              <div key={task.id} className="bg-stone-50 border border-stone-100 rounded-lg p-3 opacity-75">
                <h4 className="text-sm text-stone-600">{task.title}</h4>
              </div>
            ))}
          </div>
        </section>
      )}
    </div>
  );
}
