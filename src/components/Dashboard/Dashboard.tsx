import { useEffect } from 'react';
import { useTaskStore } from '../../store/taskStore';
import { NowTask } from './NowTask';
import { NextTasks } from './NextTasks';
import { WaitingTasks } from './WaitingTasks';

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
      {nowTask ? (
        <NowTask task={nowTask} />
      ) : (
        <section>
          <h2 className="text-sm font-bold text-stone-400 uppercase tracking-wider mb-3">Now</h2>
          <div className="bg-white border border-dashed border-stone-300 rounded-xl p-8 text-center text-stone-400">
            No active task. Pick one from Next!
          </div>
        </section>
      )}

      <NextTasks tasks={nextTasks} hasNowTask={!!nowTask} />
      
      <WaitingTasks tasks={waitingTasks} />
    </div>
  );
}
